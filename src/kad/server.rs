use std::collections::HashMap;
use std::mem::forget;
use std::net::{SocketAddr, UdpSocket};
use std::slice::from_raw_parts;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use bencode::variables::bencode_object::BencodeObject;
use bencode::variables::inter::bencode_variable::BencodeVariable;
use crate::kad::kademlia_base::KademliaBase;
use crate::kademlia::Kademlia;
use crate::messages::find_node_request::FindNodeRequest;
use crate::messages::inter::message_base::{MessageBase, TID_KEY};
use crate::messages::inter::message_exception::MessageException;
use crate::messages::inter::message_key::MessageKey;
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};
use crate::messages::inter::method_message_base::MethodMessageBase;
use crate::messages::ping_request::PingRequest;
use crate::messages::ping_response::PingResponse;
use crate::rpc::call::Call;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::events::request_event::RequestEvent;
use crate::rpc::events::response_event::ResponseEvent;
use crate::rpc::request_listener::RequestCallback;
use crate::rpc::response_tracker::{ByteWrapper, ResponseTracker};
use crate::utils;
use crate::utils::net::address_utils::is_bogon;
use crate::utils::node::Node;
use crate::utils::uid::{ID_LENGTH, UID};

pub const TID_LENGTH: usize = 6;

pub struct Server {
    pub(crate) kademlia: Option<Box<dyn KademliaBase>>,
    server: Option<Arc<UdpSocket>>,
    tracker: ResponseTracker,//Arc<Mutex<ResponseTracker>>,
    //running: Arc<AtomicBool>, //MAY NOT BE NEEDED
    request_mapping: HashMap<String, Vec<RequestCallback>>,
    messages: HashMap<MessageKey, fn() -> Box<dyn MethodMessageBase>>
}

impl Server {

    pub fn new(/*kademlia: Box<dyn KademliaBase>*/) -> Self {
        let mut self_ = Self {
            kademlia: None,
            server: None,
            tracker: ResponseTracker::new(),
            //running: Arc::new(AtomicBool::new(false)), //MAY NOT BE NEEDED
            request_mapping: HashMap::new(),
            messages: HashMap::new()
        };

        self_.register_message(|| Box::new(PingRequest::default()));
        self_.register_message(|| Box::new(PingResponse::default()));
        self_.register_message(|| Box::new(FindNodeRequest::default()));
        //self_.register_message(|| Box::new(FindNodeResponse::default()));

        //CAN THIS BE MOVED TO k_request_listener?
        let ping_callback: RequestCallback = |event| {
            println!("{}", event.get_message().to_string());

            let mut response = PingResponse::default();
            response.set_transaction_id(*event.get_message().get_transaction_id());
            response.set_destination(event.get_message().get_origin().unwrap());
            response.set_public(event.get_message().get_origin().unwrap());
            event.set_response(Box::new(response));
        };


        let find_node_callback: RequestCallback = |event| {
            println!("{}", event.get_message().to_string());
        };

        self_.register_request_listener("ping", ping_callback);
        self_.register_request_listener("find_node", find_node_callback);

        self_
    }

    pub fn start(&mut self, port: u16) {
        //self.running.store(true, Ordering::Relaxed);
        //let running = Arc::clone(&self.running);

        /*
        let handle = thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                println!("{}", kademlia.get_routing_table().lock().unwrap().get_derived_uid().to_string());
                sleep(Duration::from_secs(1));
            }
        });
        */

        self.server = Some(Arc::new(UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind socket")));
        let (tx, rx) = channel();
        let sender = tx.clone();
        let server = Arc::clone(self.server.as_ref().unwrap());

        let receiver_handle = thread::spawn(move || {
            let mut buf = [0u8; 65535];

            loop {
                let (size, src_addr) = {
                    server.recv_from(&mut buf).expect("Failed to receive message")
                };

                let data = &buf[..size];

                let bytes = data.as_ptr();
                let len = data.len();
                forget(data);

                unsafe {
                    sender.send((from_raw_parts(bytes, len), src_addr)).expect("Failed to send packet to handler");
                }
            }
        });


        let kademlia = self.kademlia.clone();

        // Start the handler thread
        let handler_handle = thread::spawn(move || {
            loop {
                // Receive packets from the receiver
                match rx.recv() {
                    Ok((data, src_addr)) => {
                        // Process the received packet (e.g., parse, handle, etc.)
                        //let message = String::from_utf8_lossy(data);
                        //println!("Received message '{}' from {}", message, src_addr);

                        kademlia.as_ref().unwrap().get_server().lock().unwrap().on_receive(data, src_addr);
                        //Server::on_receive(data, src_addr);
                        //kademlia.get_server().lock().unwrap().is_running();

                    }
                    Err(_) => break, // Break the loop if the channel is closed
                }
            }
        });


        // Join the threads
        //receiver_handle.join().expect("Receiver thread panicked");
        //handler_handle.join().expect("Handler thread panicked");
    }

    pub fn stop(&self) {
        //self.running.store(false, Ordering::Relaxed);
    }

    //REGISTER MESSAGES...

    pub fn register_request_listener(&mut self, key: &str, callback: RequestCallback) {
        let key = key.to_string();
        if self.request_mapping.contains_key(&key) {
            self.request_mapping.get_mut(&key).unwrap().push(callback);
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
        false
    }

    pub fn on_receive(&mut self, data: &[u8], src_addr: SocketAddr) {
        if is_bogon(src_addr) {
            //return;
        }

        println!("RECEIVED: {}", self.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap().get_derived_uid().to_string());


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

                            let constructor = self.messages.get(&message_key).unwrap();
                            let mut m = constructor();

                            let mut tid = [0u8; TID_LENGTH];
                            tid.copy_from_slice(ben.get_bytes(TID_KEY).map_err(|e| MessageException::new("Method Unknown", 204))?);

                            m.set_transaction_id(tid);
                            m.decode(&ben);
                            m.set_origin(src_addr);

                            let node = Node::new(m.get_uid(), m.get_origin().unwrap());
                            self.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap().insert(node);


                            let k = ben.get_string(t.rpc_type_name()).unwrap().to_string();

                            if !self.request_mapping.contains_key(&k) {
                                return Err(MessageException::new("Method Unknown", 204));
                            }

                            let mut event = RequestEvent::new(m.upcast());
                            event.set_node(node);

                            let callbacks = self.request_mapping.get(&k).unwrap();

                            if !callbacks.is_empty() {
                                for callback in callbacks {
                                    callback(&mut event);
                                }

                                if event.is_prevent_default() {
                                    //RETURN NOTHING - NO ERROR
                                    return Err(MessageException::new("Method Unknown", 204));
                                }

                                if !event.has_response() {
                                    return Err(MessageException::new("Method Unknown", 204));
                                }

                                //REMOVE - ONLY FOR TESTING...
                                event.get_response().unwrap().set_uid(self.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap().get_derived_uid());
                                //REMOVE ^^^^^^^^^^^

                                println!("RESPONSE: {}", event.get_response().unwrap().to_string());

                                self.send(event.get_response().unwrap());
                            }

                            if !self.kademlia.as_ref().unwrap().get_refresh_handler().lock().unwrap().is_running() {
                                self.kademlia.as_ref().unwrap().get_refresh_handler().lock().unwrap().start();
                            }

                            Ok(())
                        }() {
                            println!("{}", e.get_message());

                            /*
                            ErrorResponse response = new ErrorResponse(ben.getBytes(TID_KEY));
                            response.setDestination(packet.getAddress(), packet.getPort());
                            response.setPublic(packet.getAddress(), packet.getPort());
                            response.setCode(e.getCode());
                            response.setDescription(e.getMessage());
                            send(response);
                            */
                        }
                    },
                    MessageType::RspMsg => {
                        if let Err(e) = || -> Result<(), MessageException> {
                            let mut tid = [0u8; TID_LENGTH];
                            tid.copy_from_slice(ben.get_bytes(TID_KEY).expect("Failed to find TID key."));

                            let call = self.tracker.poll(&tid).ok_or(MessageException::new("Server Error", 202))?;

                            //PROBLEM LINE BELOW... - NEED TO MAKE THE MESSAGE FIND_NODE_RESPONSE...
                            let message_key = MessageKey::new(ben.get_string(t.rpc_type_name())
                                    .map_err(|e| MessageException::new("Method Unknown", 204))?, t);

                            let constructor = self.messages.get(&message_key).unwrap();
                            let mut m = constructor();

                            m.set_transaction_id(tid);
                            m.decode(&ben);
                            m.set_origin(src_addr);

                            if m.get_public().is_some() {
                                println!("UPDATE CONSENSUS IP");
                            }


                            /*
                                if(m.getPublic() != null){
                                    kademlia.getRoutingTable().updatePublicIPConsensus(m.getOriginAddress(), m.getPublicAddress());
                                }

                                if(!call.getMessage().getDestination().equals(m.getOrigin())){
                                    throw new MessageException("Generic Error", 201);
                                }

                                ResponseEvent event;

                                if(call.hasNode()){
                                    if(!call.getNode().getUID().equals(m.getUID())){
                                        throw new MessageException("Generic Error", 201);
                                    }
                                    event = new ResponseEvent(m, call.getNode());

                                }else{
                                    event = new ResponseEvent(m, new Node(m.getUID(), m.getOrigin()));
                                }

                                event.received();
                                event.setSentTime(call.getSentTime());
                                event.setRequest(call.getMessage());

                                call.getResponseCallback().onResponse(event);

                            }catch(MessageException e){
                                e.printStackTrace();
                            }
                            */


                            self.tracker.get(&tid).unwrap().get_response_callback().test();


                            println!("RES  {}", ben.to_string());

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
            }
        }

        /*
        if !ben.contains_key(TID_KEY) || !ben.contains_key(TYPE_KEY) {
            //panic
            return;
        }

        let t = MessageType::from_rpc_type_name(ben.get_string(TYPE_KEY).unwrap().to_string()).unwrap();

        match t {
            MessageType::ReqMsg => {
                let k = ben.get_string(t.rpc_type_name()).unwrap(); //PROBABLY SHOULD ERR OUT IF NO RESULT...





                println!("REQ  {}", k);
            },
            MessageType::RspMsg => {
                println!("RES  {}", ben.to_string());
            },
            MessageType::ErrMsg => {
                println!("ERR  {}", ben.to_string());
            }
        }
        */
    }

    pub fn send(&self, message: &mut dyn MessageBase) {
        if let Some(server) = &self.server {
            message.set_uid(self.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap().get_derived_uid());
            server.send_to(message.encode().encode().as_slice(), message.get_destination().unwrap()).unwrap(); //probably should return if failed to send...
        }
    }

    pub fn send_with_callback(&mut self, message: &mut dyn MethodMessageBase, callback: Box<dyn ResponseCallback>) {
        if let Some(server) = &self.server {
            //self.tracker.add(ByteWrapper::from(self.generate_transaction_id()), callback);
            let tid = self.generate_transaction_id();
            message.set_transaction_id(tid);
            message.set_uid(self.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap().get_derived_uid());

            let call = Call::new(message, callback);
            println!("{}", call.get_message().to_string());
            self.tracker.add(tid, call);

            server.send_to(message.encode().encode().as_slice(), message.get_destination().unwrap()).unwrap(); //probably should return if failed to send...
        }
    }

    pub fn generate_transaction_id(&self) -> [u8; TID_LENGTH] {
        utils::random::gen_array::<TID_LENGTH>()
    }
}
