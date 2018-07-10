#[macro_use]

use sciter;

use std::{thread, time};
use std::sync::mpsc;

struct Searcher<'a> {
    search_term: String,
    index: i32,
    root: &'a sciter::Element,
    _initialized: bool,
}

impl<'a> Searcher<'a> {

    pub fn uninitialized(root: &'a sciter::Element) -> Self {
        Self {
            search_term: String::from(""),
            index: 0,
            root: root,
            _initialized: false,
        }
    }

    pub fn new(search_term: String, root: &'a sciter::Element) -> Self {
        Self {
            search_term: search_term.clone(),
            index: 0,
            root: root,
            _initialized: true,
        }
    }

    pub fn search_once(&mut self) {
        if !self._initialized {
            return;
        }
        let _ = self.root.call_function("search.addToQueue", &make_args!(format!("{} {}", self.search_term, self.index)));
        self.index += 1;
    }

}

pub fn spawn_search_thread(receiver: mpsc::Receiver<String>, root: sciter::Element) {
    thread::spawn(move || {
        let mut searcher = Searcher::uninitialized(&root);
        loop {
            match receiver.try_recv() {
                Ok(search_term) => {
                    let _ = searcher.root.call_function("search.clearQueueAndSearchResult", &make_args!());
                    searcher = Searcher::new(search_term, &root);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Terminated!");
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
            searcher.search_once();
            thread::sleep(time::Duration::from_millis(100));
        }
    });
}
