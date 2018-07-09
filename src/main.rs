#[macro_use]
extern crate sciter;

mod load_handler;

struct EventHandler; 

impl EventHandler {

}

impl sciter::EventHandler for EventHandler {

    dispatch_script_call! {
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

    frame.event_handler(EventHandler);
    frame.run_app();
}
