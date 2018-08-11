extern crate config;
extern crate plugins;
#[macro_use]
extern crate sciter;
extern crate settings;

mod load_handler;
mod spawner;

use config::Config;
use config::keybindings_config;
use std::sync::mpsc;

struct EventHandler {
    sender: mpsc::Sender<String>,
} 

impl EventHandler {

    fn execute_primary_action(&self, search_term: String, candidate: String) -> bool {
        if let Some(plugin) = plugins::get_plugin(&search_term) {
            return plugin.execute_primary_action(&candidate);
        }
        false
    }

    fn execute_secondary_action(&self, search_term: String, candidate: String) -> bool {
        if let Some(plugin) = plugins::get_plugin(&search_term) {
            return plugin.execute_secondary_action(&candidate);
        }
        false
    }

    fn get_action_from_keybindings(&self, signature: String) -> String {
        let config = keybindings_config::KeybindingsConfig::new("keybindings.yaml").unwrap();
        match config.get_key_from_value(&signature) {
            Some(value) => value,
            None => String::new(),
        }
    }

    fn get_os(&self) -> String {
        if cfg!(target_os = "linux") {
            return String::from("linux");
        } else if cfg!(target_os = "macos") {
            return String::from("Mac OS");
        } else if cfg!(target_os = "windows") {
            return String::from("Windows");
        } else {
            return String::from("Unknown");
        }
    }

    fn query(&self, search_term: String) -> bool {
        let _ = self.sender.send(search_term);
        true
    }

    fn show_settings_window(&self) -> bool {
        settings::show()
    }

}

impl sciter::EventHandler for EventHandler {

    dispatch_script_call! {
        fn execute_primary_action(String, String);
        fn execute_secondary_action(String, String);
        fn get_action_from_keybindings(String);
        fn get_os();
        fn query(String);
        fn show_settings_window();
    }

}

fn main() {
    let _ = sciter::set_options(sciter::RuntimeOptions::DebugMode(true));

    let resources = include_bytes!("resources.rc");
    let handler = load_handler::LoadHandler::new(resources);

    let mut frame = sciter::window::Builder::main()
        .resizeable()
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
