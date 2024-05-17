use crate::rpc::events::response_event::ResponseEvent;

pub trait ResponseCallback: Send {

    fn on_response(&self, event: ResponseEvent);

    //fn on_error_response(&self, event: ErrorResponseEvent);

    //fn on_stalled(&self, event: StalledEvent);
}
