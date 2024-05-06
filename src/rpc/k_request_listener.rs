use crate::rpc::request_listener::RequestListener;

pub struct KRequestListener {

}

impl KRequestListener {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl RequestListener for KRequestListener {

    fn on_request(&self) {
        todo!()
    }
}
