use sciter;

use std::{thread, time};
use std::sync::mpsc;

use searcher::Searcher;

pub fn spawn_search_thread(receiver: mpsc::Receiver<String>, root: sciter::Element) {
    thread::spawn(move || {
        let (mut _search_sender, mut search_receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let mut is_first_take = false;
        let mut still_receiving = false;
        loop {
            match receiver.try_recv() {
                Ok(search_term) => {
                    let (search_sender_clone, search_receiver_clone) = mpsc::channel();
                    _search_sender = search_sender_clone;
                    search_receiver = search_receiver_clone;
                    let _ = root.call_function("search.clearQueueAndSearchResult", &make_args!());
                    if search_term != "" {
                        spawn_search_worker(search_term, _search_sender);
                        still_receiving = true;
                        is_first_take = true;
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Terminated!");
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
            if still_receiving {
                still_receiving = send_candidates_to_queue(&search_receiver, &root, &mut is_first_take);
            }
        }
    });
}

fn spawn_search_worker(search_term: String, sender: mpsc::Sender<String>) {
    thread::spawn(move || {
        let searcher = Searcher::new(search_term);
        let result = searcher.search();
        for candidate in &result {
            thread::sleep(time::Duration::from_millis(100));
            let send_result = sender.send(candidate.to_string());
            if send_result.is_err() {
                break;
            }
        }
    });
}

fn send_candidates_to_queue(receiver: &mpsc::Receiver<String>, root: &sciter::Element, is_first_take: &mut bool) -> bool {
    let mut still_receiving = true;
    match receiver.try_recv() {
        Ok(candidate) => {
            let _ = root.call_function("search.addToQueue", &make_args!(candidate));
            if *is_first_take {
                let _ = root.call_function("candidates.resetColor", &make_args!());
                *is_first_take = false;
            }
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            println!("Terminated!");
            still_receiving = false;
        }
        Err(mpsc::TryRecvError::Empty) => {}
    }
    still_receiving
}
