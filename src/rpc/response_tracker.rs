use std::collections::{HashMap, LinkedList};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::kad::server::TID_LENGTH;
use crate::rpc::call::Call;

pub const MAX_ACTIVE_CALLS: usize = 512;
pub const STALLED_TIME: u128 = 60000;


#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ByteWrapper {
    // Define your ByteWrapper struct here
    pub(crate) b: [u8]
}

// Define your Call struct here

pub struct ResponseTracker {
    calls: HashMap<[u8; TID_LENGTH], Call>,
}

impl ResponseTracker {

    pub fn new() -> Self {
        ResponseTracker {
            //calls: HashMap::with_capacity(MAX_ACTIVE_CALLS),
            calls: HashMap::with_capacity(MAX_ACTIVE_CALLS),
        }
    }

    pub fn add(&mut self, tid: [u8; TID_LENGTH], call: Call) {
        self.calls.insert(tid, call);
    }

    pub fn get(&self, tid: &[u8; TID_LENGTH]) -> Option<&Call> {
        self.calls.get(tid)
    }

    pub fn contains(&self, tid: &[u8; TID_LENGTH]) -> bool {
        self.calls.contains_key(tid)
    }

    pub fn remove(&mut self, tid: &[u8; TID_LENGTH]) -> Option<Call> {
        self.calls.remove(tid)
    }

    pub fn poll(&mut self, tid: &[u8; TID_LENGTH]) -> Option<Call> {
        self.calls.remove(tid)
    }

    pub fn remove_stalled(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut stalled = Vec::new();

        for (tid, call) in self.calls.iter() {
            if !call.is_stalled(now) {
                break;
            }

            stalled.push(tid.clone());
        }

        for tid in stalled {
            //if let Some(call) = self.calls.remove(&tid) {
            //    println!("STALLED {}", call.get_node().to_string());

                /*
                if let Some(response_callback) = call.get_response_callback() {
                    let event = StalledEvent {
                        message: call.get_message(),
                        sent_time: call.get_sent_time(),
                        node: call.get_node(),
                    };
                    response_callback.on_stalled(event);
                }
                */
            //}
        }
    }
}
