use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::events::response_event::ResponseEvent;

pub struct JoinNodeListener {

}

impl JoinNodeListener {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl ResponseCallback for JoinNodeListener {

    fn test(&self) {
        println!("RESPONSE RECEIVED TEST");
    }

    fn on_response(&self, event: ResponseEvent) {
        println!("RESPONSE RECEIVED");
    }

    //fn on_error_response(&self, event: ErrorResponseEvent);

    //fn on_stalled(&self, event: StalledEvent);
}
