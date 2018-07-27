extern crate search_candidate;

use sciter;

use self::search_candidate::Key;
use std::{thread, time};
use std::sync::mpsc;

use plugins;
use plugins::Plugin;

pub fn spawn_plugin_thread(receiver: mpsc::Receiver<String>, root: sciter::Element) {
    thread::spawn(move || {
        let (mut _search_sender, mut search_receiver): (mpsc::Sender<Vec<String>>, mpsc::Receiver<Vec<String>>) =
            mpsc::channel();
        let mut is_first_take = false;
        let mut still_receiving = false;
        loop {
            match receiver.try_recv() {
                Ok(search_term) => {
                    let (search_sender_clone, search_receiver_clone) = mpsc::channel();
                    _search_sender = search_sender_clone;
                    search_receiver = search_receiver_clone;

                    let _ = root.call_function("search.clearQueueAndSearchResult", &make_args!());
                    if let Some(plugin) = plugins::get_plugin(search_term.clone()) {
                        spawn_plugin_worker(plugin, search_term.clone(), _search_sender);
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

fn spawn_plugin_worker(plugin: Box<Plugin + Send>, search_term: String, sender: mpsc::Sender<Vec<String>>) {
    thread::spawn(move || {
        let results = plugin.get_search_result(search_term);
        match results {
            Ok(search_results) => {
                for candidate in &search_results {
                    thread::sleep(time::Duration::from_millis(100));
                    let send_result = sender.send(vec![
                        candidate.get_value(Key::SearchText),
                        candidate.get_value(Key::DisplayText),
                        candidate.get_value(Key::IconPath),
                    ]);
                    if send_result.is_err() {
                        break;
                    }
                }
            }
            Err(()) => {}
        }
    });
}

fn send_candidates_to_queue(receiver: &mpsc::Receiver<Vec<String>>, root: &sciter::Element, is_first_take: &mut bool) -> bool {
    let mut still_receiving = true;
    match receiver.try_recv() {
        Ok(candidate) => {
            let _ = root.call_function("search.addToQueue", &make_args!(candidate[0].clone(),
                                                                        candidate[1].clone(),
                                                                        candidate[2].clone()));
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
