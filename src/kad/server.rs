use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::mpsc::{channel, TryRecvError};
use bencode::variables::bencode_object::BencodeObject;
use bencode::variables::inter::bencode_variable::BencodeVariable;
use crate::kad::kademlia_base::KademliaBase;
use crate::messages::error_response::ErrorResponse;
use crate::messages::inter::message_base::{MessageBase, TID_KEY};
use crate::messages::inter::message_exception::MessageException;
use crate::messages::inter::message_key::MessageKey;
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};
use crate::messages::inter::method_message_base::MethodMessageBase;
use crate::rpc::call::Call;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::events::request_event::RequestEvent;
use crate::rpc::events::response_event::ResponseEvent;
use crate::rpc::request_listener::RequestCallback;
use crate::rpc::response_tracker::ResponseTracker;
use crate::utils;
use crate::utils::net::address_utils::is_bogon;
use crate::utils::node::Node;

pub const TID_LENGTH: usize = 6;

pub struct Server {
    pub(crate) kademlia: Option<Box<dyn KademliaBase>>,
    server: Option<Arc<UdpSocket>>,
    tracker: ResponseTracker,//Arc<Mutex<ResponseTracker>>,
    running: Arc<AtomicBool>, //MAY NOT BE NEEDED
    request_mapping: HashMap<String, Vec<RequestCallback>>,
    messages: HashMap<MessageKey, fn() -> Box<dyn MethodMessageBase>>
}

impl Server {

    pub fn new(/*kademlia: Box<dyn KademliaBase>*/) -> Self {
        let mut self_ = Self {
            kademlia: None,
            server: None,
            tracker: ResponseTracker::new(),
            running: Arc::new(AtomicBool::new(false)), //MAY NOT BE NEEDED
            request_mapping: HashMap::new(),
            messages: HashMap::new()
        };

        self_
    }

    pub fn start(&mut self, port: u16) {
        if self.is_running() {
            //panic or something...
            return;
        }

        self.running.store(true, Ordering::Relaxed);

        self.server = Some(Arc::new(UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], port))).expect("Failed to bind socket")));
        let (tx, rx) = channel();
        let sender = tx.clone();
        let server = Arc::clone(self.server.as_ref().unwrap());
        let running = Arc::clone(&self.running);

        let receiver_handle = thread::spawn(move || {
            let mut buf = [0u8; 65535];

            while running.load(Ordering::Relaxed) {
                let (size, src_addr) = {
                    server.recv_from(&mut buf).expect("Failed to receive message")
                };

                sender.send((buf[..size].to_vec(), src_addr)).unwrap();

                /*
                let data = &buf[..size];

                let bytes = data.as_ptr();
                let len = data.len();
                forget(data);

                unsafe {
                    sender.send((from_raw_parts(bytes, len), src_addr)).expect("Failed to send packet to handler");
                }
                */
            }
        });

        let mut kademlia = self.kademlia.clone();
        let running = Arc::clone(&self.running);

        let handler_handle = thread::spawn(move || {
            let mut kademlia = kademlia.unwrap();
            while running.load(Ordering::Relaxed) {
                match rx.try_recv() {
                    Ok((data, src_addr)) => {
                        Self::on_receive(kademlia.as_mut(), data.as_slice(), src_addr);
                    }
                    Err(TryRecvError::Empty) => {
                    }
                    Err(TryRecvError::Disconnected) => break
                }

                kademlia.get_server().lock().unwrap().tracker.remove_stalled();
            }
        });


        // Join the threads
        //receiver_handle.join().expect("Receiver thread panicked");
        //handler_handle.join().expect("Handler thread panicked");
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    //REGISTER MESSAGES...

    pub fn register_request_listener(&mut self, key: &str, callback: RequestCallback) {
        let key = key.to_string();
        if self.request_mapping.contains_key(&key) {
            self.request_mapping.get_mut(&key).unwrap().push(callback);
            return;
        }
        let mut mapping = Vec::new();
        mapping.push(callback);
        self.request_mapping.insert(key.to_string(), mapping);
    }

    pub fn register_message(&mut self, constructor: fn() -> Box<dyn MethodMessageBase>) {
        let message = constructor();
        self.messages.insert(MessageKey::new(message.get_method(), message.get_type()), constructor);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn on_receive(kademlia: &mut dyn KademliaBase, data: &[u8], src_addr: SocketAddr) {
        if is_bogon(src_addr) {
            //return;
        }

        match BencodeObject::decode(data) {
            Ok(ben) => {
                if !ben.contains_key(TID_KEY) || !ben.contains_key(TYPE_KEY) {
                    //panic
                    return;
                }

                let t = MessageType::from_rpc_type_name(ben.get_string(TYPE_KEY).unwrap().to_string()).unwrap();

                match t {
                    MessageType::ReqMsg => {
                        if let Err(e) = || -> Result<(), MessageException> {
                            let message_key = MessageKey::new(ben.get_string(t.rpc_type_name())
                                                                  .map_err(|e| MessageException::new("Method Unknown", 204))?, t);

                            let mut m = kademlia.get_server().lock().as_ref().unwrap().messages.get(&message_key).unwrap()();
                            //let mut m = constructor();

                            let mut tid = [0u8; TID_LENGTH];
                            tid.copy_from_slice(ben.get_bytes(TID_KEY).map_err(|e| MessageException::new("Method Unknown", 204))?);

                            m.set_transaction_id(tid);
                            m.decode(&ben)?;
                            m.set_origin(src_addr);

                            let node = Node::new(m.get_uid(), m.get_origin().unwrap());
                            kademlia.get_routing_table().lock().unwrap().insert(node);


                            let k = ben.get_string(t.rpc_type_name()).unwrap().to_string();

                            if !kademlia.get_server().lock().as_ref().unwrap().request_mapping.contains_key(&k) {
                                return Err(MessageException::new("Method Unknown", 204));
                            }

                            let mut event = RequestEvent::new(m.upcast());
                            event.set_node(node);

                            for callback in kademlia.get_server().lock().as_ref().unwrap().request_mapping.get(&k).unwrap() {
                                callback(&mut event);
                            }

                            if event.is_prevent_default() {
                                //RETURN NOTHING - NO ERROR
                                return Err(MessageException::new("Method Unknown", 204));
                            }

                            if !event.has_response() {
                                return Err(MessageException::new("Method Unknown", 204));
                            }

                            kademlia.get_server().lock().as_ref().unwrap().send(event.get_response().unwrap()).unwrap();

                            if !kademlia.get_refresh_handler().lock().unwrap().is_running() {
                                kademlia.get_refresh_handler().lock().unwrap().start();
                            }

                            Ok(())

                        }() {
                            println!("{}", e.get_message());

                            let mut tid = [0u8; TID_LENGTH];
                            tid.copy_from_slice(ben.get_bytes(TID_KEY).map_err(|e| MessageException::new("Method Unknown", 204)).unwrap());

                            let mut response = ErrorResponse::new(tid);
                            response.set_destination(src_addr);
                            response.set_public(src_addr);
                            response.set_code(e.get_code());
                            response.set_description(e.get_message());

                            kademlia.get_server().lock().as_ref().unwrap().send(&mut response).unwrap();
                        }
                    },
                    MessageType::RspMsg => {
                        if let Err(e) = || -> Result<(), MessageException> {
                            let mut tid = [0u8; TID_LENGTH];
                            tid.copy_from_slice(ben.get_bytes(TID_KEY).expect("Failed to find TID key."));

                            let call = kademlia.get_server().lock().as_mut().unwrap().tracker.poll(&tid).ok_or(MessageException::new("Server Error", 202))?;

                            //PROBLEM LINE BELOW... - NEED TO MAKE THE MESSAGE FIND_NODE_RESPONSE...
                            let message_key = MessageKey::new(call.get_message().get_method(), t);

                            let mut m = kademlia.get_server().lock().as_ref().unwrap().messages.get(&message_key).unwrap()();

                            m.set_transaction_id(tid);
                            m.decode(&ben)?;
                            m.set_origin(src_addr);

                            if m.get_public().is_some() {
                                kademlia.get_routing_table().lock().unwrap()
                                    .update_public_ip_consensus(m.get_origin().unwrap().ip(), m.get_public().unwrap().ip());
                            }

                            if call.get_message().get_destination() != m.get_origin() {
                                return Err(MessageException::new("Generic Error", 201));
                            }

                            let mut event;

                            if call.has_node() {
                                if call.get_node().uid != m.get_uid() {
                                    return Err(MessageException::new("Generic Error", 201));
                                }

                                event = ResponseEvent::new(m.as_ref().upcast(), call.get_node());

                            } else {
                                event = ResponseEvent::new(m.as_ref().upcast(), Node::new(m.get_uid(), m.get_origin().unwrap()));
                            }

                            event.received();
                            event.set_sent_time(call.get_sent_time());
                            event.set_request(call.get_message().upcast());

                            call.get_response_callback().on_response(event);

                            Ok(())

                        }() {
                            println!("{}", e.get_message());
                        }
                    },
                    MessageType::ErrMsg => {
                        println!("ERR  {}", ben.to_string());
                    }
                }
            },
            Err(e) => {
                println!("{}", e.to_string());
                /*
                        byte[] tid = ben.getBytes(TID_KEY);
                        Call call = tracker.poll(new ByteWrapper(tid));

                        try{
                            if(call == null){
                                throw new MessageException("Server Error", 202);
                            }

                            ErrorResponse m = new ErrorResponse(tid);
                            m.decode(ben);
                            m.setOrigin(packet.getAddress(), packet.getPort());

                            if(m.getPublic() != null){
                                kademlia.getRoutingTable().updatePublicIPConsensus(m.getOriginAddress(), m.getPublicAddress());
                            }

                            if(!call.getMessage().getDestination().equals(m.getOrigin())){
                                throw new MessageException("Generic Error", 201);
                            }

                            ErrorResponseEvent event;

                            if(call.hasNode()){
                                event = new ErrorResponseEvent(m, call.getNode());

                            }else{
                                event = new ErrorResponseEvent(m);
                            }

                            event.received();
                            event.setSentTime(call.getSentTime());
                            event.setRequest(call.getMessage());

                            call.getResponseCallback().onErrorResponse(event);

                        }catch(MessageException e){
                            e.printStackTrace();
                        }
                */
            }
        }
    }

    pub fn send(&self, message: &mut dyn MessageBase) -> Result<(), String> {
        if message.get_destination().is_none() {
            return Err("Message destination set to null".to_string());
        }

        if is_bogon(message.get_destination().unwrap()) {
            //return Err("Message destination set to bogon".to_string());
        }

        if message.get_type() != MessageType::ErrMsg {
            message.set_uid(self.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap().get_derived_uid());
        }

        if let Some(server) = &self.server {
            server.send_to(message.encode().encode().as_slice(), message.get_destination().unwrap()).unwrap(); //probably should return if failed to send...
        }

        Ok(())
    }

    pub fn send_with_callback(&mut self, message: &mut dyn MethodMessageBase, callback: Box<dyn ResponseCallback>) -> Result<(), String> {
        if message.get_type() != MessageType::ReqMsg {
            return self.send(message.upcast_mut());
        }

        let tid = self.generate_transaction_id();
        message.set_transaction_id(tid);
        self.tracker.add(tid, Call::new(message, callback));
        return self.send(message.upcast_mut())
    }

    pub fn send_with_node_callback(&mut self, message: &mut dyn MethodMessageBase, node: Node, callback: Box<dyn ResponseCallback>) -> Result<(), String> {
        if message.get_type() != MessageType::ReqMsg {
            return self.send(message.upcast_mut());
        }

        let tid = self.generate_transaction_id();
        message.set_transaction_id(tid);
        let mut call = Call::new(message, callback);
        call.set_node(node);
        self.tracker.add(tid, call);
        return self.send(message.upcast_mut())
    }

    pub fn generate_transaction_id(&self) -> [u8; TID_LENGTH] {
        utils::random::gen_array::<TID_LENGTH>()
    }
}
