use crate::rpc::events::inter::message_event::MessageEvent;
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

    fn on_response(&self, event: ResponseEvent) {
        println!("RESPONSE RECEIVED");
        println!("RES  {}", event.get_message().to_string());
    }

    //fn on_error_response(&self, event: ErrorResponseEvent);

    //fn on_stalled(&self, event: StalledEvent);
}
