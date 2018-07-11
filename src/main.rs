#[macro_use]
extern crate sciter;

mod load_handler;
mod search;
mod searcher;

use std::sync::mpsc;

struct EventHandler {
    sender: mpsc::Sender<String>,
} 

impl EventHandler {

    fn query(&self, search_term: String) -> bool {
        let _ = self.sender.send(search_term);
        true
    }

}

impl sciter::EventHandler for EventHandler {

    dispatch_script_call! {
        fn query(String);
    }

}

fn main() {
    let _ = sciter::set_options(sciter::RuntimeOptions::DebugMode(true));

    let resources = include_bytes!("resources.rc");
    let handler = load_handler::LoadHandler::new(resources);

    let mut frame = sciter::window::Builder::main()
        .fixed()
        .create();

    frame.sciter_handler(handler);
    frame.load_file("this://app/html/index.htm");

    let (sender, receiver) = mpsc::channel();
    let event_handler = EventHandler {
        sender: sender,
    };

    search::spawn_search_thread(receiver, sciter::Element::from_window(frame.get_hwnd()).unwrap());

    frame.event_handler(event_handler);
    frame.run_app();
}
