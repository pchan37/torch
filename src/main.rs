extern crate plugins;
#[macro_use]
extern crate sciter;

mod load_handler;
mod spawner;

use std::sync::mpsc;

struct EventHandler {
    sender: mpsc::Sender<String>,
} 

impl EventHandler {

    fn execute_primary_action(&self, search_term: String, candidate: String) -> bool {
        if let Some(plugin) = plugins::get_plugin(search_term.clone()) {
            return plugin.execute_primary_action(candidate);
        }
        false
    }

    fn execute_secondary_action(&self, search_term: String, candidate: String) -> bool {
        if let Some(plugin) = plugins::get_plugin(search_term.clone()) {
            return plugin.execute_secondary_action(candidate);
        }
        false
    }

    fn query(&self, search_term: String) -> bool {
        let _ = self.sender.send(search_term);
        true
    }

}

impl sciter::EventHandler for EventHandler {

    dispatch_script_call! {
        fn execute_primary_action(String, String);
        fn execute_secondary_action(String, String);
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

    let root = sciter::Element::from_window(frame.get_hwnd()).unwrap();
    spawner::spawn_plugin_thread(receiver, root);

    frame.event_handler(event_handler);
    frame.run_app();
}
