extern crate sciter;

use sciter::host;

pub struct LoadHandler {
    archive: host::Archive,
}

impl LoadHandler {

    pub fn new(archive: &[u8]) -> Self {
        Self {
            archive: host::Archive::open(archive).expect("Unable to open archive."),
        }
    }

}

impl sciter::HostHandler for LoadHandler {

    fn on_data_load(&mut self, pnm: &mut host::SCN_LOAD_DATA) -> Option<host::LOAD_RESULT> {
        let uri =  w2s!(pnm.uri);

        if uri.starts_with("this://app/") {
            if let Some(data) = self.archive.get(&uri) {
                self.data_ready(pnm.hwnd, &uri, data, None);
            } else {
                eprintln!("[handler] error: can't load {:?}", uri);
            }
        }
        return Some(host::LOAD_RESULT::LOAD_DEFAULT);
    }

}
