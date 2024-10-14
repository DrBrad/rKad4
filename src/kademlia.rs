use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::messages::find_node_request::FindNodeRequest;
use crate::messages::find_node_response::FindNodeResponse;
use crate::messages::inter::message_base::MessageBase;
use crate::messages::ping_request::PingRequest;
use crate::messages::ping_response::PingResponse;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::refresh::tasks::bucket_refresh_task::BucketRefreshTask;
use crate::refresh::tasks::inter::task::Task;
use crate::refresh::tasks::stale_refresh_task::StaleRefreshTask;
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_bucket::MAX_BUCKET_SIZE;
use crate::routing::kb::k_routing_table::KRoutingTable;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::join_node_listener::JoinNodeListener;

#[derive(Clone)]
pub struct Kademlia {
    routing_table: Arc<Mutex<dyn RoutingTable>>,
    server: Arc<Mutex<Server>>,
    refresh: Arc<Mutex<RefreshHandler>>
}

impl Default for Kademlia {

    fn default() -> Self {
        let mut server = Server::new();

        server.register_message(|| Box::new(PingRequest::default()));
        server.register_message(|| Box::new(PingResponse::default()));
        server.register_message(|| Box::new(FindNodeRequest::default()));
        server.register_message(|| Box::new(FindNodeResponse::default()));

        server.register_request_listener("ping", Box::new(move |event| {
            //println!("{}", event.get_message().to_string());

            let mut response = PingResponse::default();
            response.set_transaction_id(*event.get_message().get_transaction_id());
            response.set_destination(event.get_message().get_origin().unwrap());
            response.set_public(event.get_message().get_origin().unwrap());
            event.set_response(Box::new(response));
        }));

        let self_ = Self {
            routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: Arc::new(Mutex::new(server)),
            refresh: Arc::new(Mutex::new(RefreshHandler::new()))
        };

        let bucket_refresh = BucketRefreshTask::new(&self_);
        let bucket_refresh_clone = BucketRefreshTask::new(&self_).clone();
        self_.routing_table.lock().unwrap().add_restart_listener(Arc::new(move || {
            bucket_refresh_clone.execute();
        }));

        self_.refresh.lock().unwrap().add_operation(Box::new(bucket_refresh));
        self_.refresh.lock().unwrap().add_operation(Box::new(StaleRefreshTask::new(&self_)));

        let self_clone = self_.clone();
        self_.server.lock().unwrap().register_request_listener("find_node", Box::new(move |event| {
            //println!("{}", event.get_message().to_string());
            if event.is_prevent_default() {
                return;
            }

            let request = event.get_message().as_any().downcast_ref::<FindNodeRequest>().unwrap();

            let mut nodes = self_clone.get_routing_table().lock().unwrap()
                .find_closest(&request.get_target().unwrap(), MAX_BUCKET_SIZE);
            nodes.retain(|&n| n != event.get_node());

            if !nodes.is_empty() {
                let mut response = FindNodeResponse::default();
                response.set_transaction_id(*event.get_message().get_transaction_id());
                response.set_destination(event.get_message().get_origin().unwrap());
                response.set_public(event.get_message().get_origin().unwrap());
                response.add_nodes(nodes);
                event.set_response(Box::new(response));
            }
        }));

        self_.server.lock().unwrap().kademlia = Some(self_.clone_dyn());

        self_
    }
}

impl From<BucketTypes> for Kademlia {

    fn from(bucket_type: BucketTypes) -> Self {
        let mut server = Server::new();

        server.register_message(|| Box::new(PingRequest::default()));
        server.register_message(|| Box::new(PingResponse::default()));
        server.register_message(|| Box::new(FindNodeRequest::default()));
        server.register_message(|| Box::new(FindNodeResponse::default()));

        server.register_request_listener("ping", Box::new(move |event| {
            //println!("{}", event.get_message().to_string());

            let mut response = PingResponse::default();
            response.set_transaction_id(*event.get_message().get_transaction_id());
            response.set_destination(event.get_message().get_origin().unwrap());
            response.set_public(event.get_message().get_origin().unwrap());
            event.set_response(Box::new(response));
        }));

        let self_ = Self {
            routing_table: bucket_type.routing_table(),
            server: Arc::new(Mutex::new(server)),
            refresh: Arc::new(Mutex::new(RefreshHandler::new()))
        };

        let bucket_refresh = BucketRefreshTask::new(&self_);
        let bucket_refresh_clone = BucketRefreshTask::new(&self_).clone();
        self_.routing_table.lock().unwrap().add_restart_listener(Arc::new(move || {
            bucket_refresh_clone.execute();
        }));

        self_.refresh.lock().unwrap().add_operation(Box::new(bucket_refresh));
        self_.refresh.lock().unwrap().add_operation(Box::new(StaleRefreshTask::new(&self_)));

        let self_clone = self_.clone();
        self_.server.lock().unwrap().register_request_listener("find_node", Box::new(move |event| {
            //println!("{}", event.get_message().to_string());
            if event.is_prevent_default() {
                return;
            }

            let request = event.get_message().as_any().downcast_ref::<FindNodeRequest>().unwrap();

            let mut nodes = self_clone.get_routing_table().lock().unwrap()
                .find_closest(&request.get_target().unwrap(), MAX_BUCKET_SIZE);
            nodes.retain(|&n| n != event.get_node());

            if !nodes.is_empty() {
                let mut response = FindNodeResponse::default();
                response.set_transaction_id(*event.get_message().get_transaction_id());
                response.set_destination(event.get_message().get_origin().unwrap());
                response.set_public(event.get_message().get_origin().unwrap());
                response.add_nodes(nodes);
                event.set_response(Box::new(response));
            }
        }));

        self_.server.lock().unwrap().kademlia = Some(self_.clone_dyn());

        self_
    }
}

impl TryFrom<&str> for Kademlia {

    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut server = Server::new();

        server.register_message(|| Box::new(PingRequest::default()));
        server.register_message(|| Box::new(PingResponse::default()));
        server.register_message(|| Box::new(FindNodeRequest::default()));
        server.register_message(|| Box::new(FindNodeResponse::default()));

        server.register_request_listener("ping", Box::new(move |event| {
            //println!("{}", event.get_message().to_string());

            let mut response = PingResponse::default();
            response.set_transaction_id(*event.get_message().get_transaction_id());
            response.set_destination(event.get_message().get_origin().unwrap());
            response.set_public(event.get_message().get_origin().unwrap());
            event.set_response(Box::new(response));
        }));

        let self_ = Self {
            routing_table: BucketTypes::from_string(value)?.routing_table(),
            server: Arc::new(Mutex::new(server)),
            refresh: Arc::new(Mutex::new(RefreshHandler::new()))
        };

        let bucket_refresh = BucketRefreshTask::new(&self_);
        let bucket_refresh_clone = BucketRefreshTask::new(&self_).clone();
        self_.routing_table.lock().unwrap().add_restart_listener(Arc::new(move || {
            bucket_refresh_clone.execute();
        }));

        self_.refresh.lock().unwrap().add_operation(Box::new(bucket_refresh));
        self_.refresh.lock().unwrap().add_operation(Box::new(StaleRefreshTask::new(&self_)));

        let self_clone = self_.clone();
        self_.server.lock().unwrap().register_request_listener("find_node", Box::new(move |event| {
            //println!("{}", event.get_message().to_string());
            if event.is_prevent_default() {
                return;
            }

            let request = event.get_message().as_any().downcast_ref::<FindNodeRequest>().unwrap();

            let mut nodes = self_clone.get_routing_table().lock().unwrap()
                .find_closest(&request.get_target().unwrap(), MAX_BUCKET_SIZE);
            nodes.retain(|&n| n != event.get_node());

            if !nodes.is_empty() {
                let mut response = FindNodeResponse::default();
                response.set_transaction_id(*event.get_message().get_transaction_id());
                response.set_destination(event.get_message().get_origin().unwrap());
                response.set_public(event.get_message().get_origin().unwrap());
                response.add_nodes(nodes);
                event.set_response(Box::new(response));
            }
        }));

        self_.server.lock().unwrap().kademlia = Some(self_.clone_dyn());

        Ok(self_)
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {
        self.server.lock().unwrap().start(port);
    }

    fn join(&self, local_port: u16, addr: SocketAddr) -> Result<(), String> {
        self.server.lock().unwrap().start(local_port);

        let mut request = FindNodeRequest::default();
        request.set_destination(addr);
        request.set_target(self.routing_table.lock().unwrap().get_derived_uid());

        self.server.lock().unwrap().send_with_callback(&mut request, Box::new(JoinNodeListener::new(self)))
    }

    fn stop(&self) {
        self.server.lock().unwrap().stop();
        self.refresh.lock().unwrap().stop();
    }

    fn get_server(&self) -> &Arc<Mutex<Server>> {
        &self.server
    }

    fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>> {
        &self.routing_table
    }

    fn get_refresh_handler(&self) -> &Arc<Mutex<RefreshHandler>> {
        &self.refresh
    }

    fn join_thread(&self) {
        if self.server.lock().unwrap().is_running() {
            let handle = self.server.lock().as_mut().unwrap().handle.take().unwrap();
            handle.join().unwrap();
        }
    }

    fn clone_dyn(&self) -> Box<dyn KademliaBase> {
        Box::new(self.clone())
    }
}
