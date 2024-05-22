use crate::kad::kademlia_base::KademliaBase;
use crate::messages::find_node_request::FindNodeRequest;
use crate::messages::inter::message_base::MessageBase;
use crate::routing::kb::k_bucket::MAX_BUCKET_SIZE;
use crate::rpc::events::error_response_event::ErrorResponseEvent;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::events::response_event::ResponseEvent;
use crate::rpc::events::stalled_event::StalledEvent;
use crate::utils::uid::ID_LENGTH;
use super::inter::task::Task;

#[derive(Clone)]
pub struct BucketRefreshTask {
    kademlia: Box<dyn KademliaBase>
}

impl BucketRefreshTask {

    pub fn new(kademlia: &dyn KademliaBase) -> Self {
        Self {
            kademlia: kademlia.clone_dyn()
        }
    }
}

impl Task for BucketRefreshTask {

    fn execute(&self) {
        let mut listener = Box::new(FindNodeResponseListener::new(self.kademlia.as_ref()));
        println!("EXECUTING BUCKET REFRESH");

        for i in 1..ID_LENGTH*8 {
            if self.kademlia.get_routing_table().lock().unwrap().bucket_size(i) < MAX_BUCKET_SIZE {
                let k = self.kademlia.get_routing_table().lock().unwrap().get_derived_uid().generate_node_id_by_distance(i);

                let closest = self.kademlia.get_routing_table().lock().unwrap().find_closest(&k, MAX_BUCKET_SIZE);
                if closest.is_empty() {
                    continue;
                }

                for node in closest {
                    let mut request = FindNodeRequest::default();
                    request.set_destination(node.address);
                    request.set_target(k);

                    self.kademlia.get_server().lock().unwrap().send_with_callback(&mut request, listener.clone()); //ADD NODE...
                }
            }
        }
    }

    fn clone_dyn(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct FindNodeResponseListener {
    kademlia: Box<dyn KademliaBase>
}

impl FindNodeResponseListener {

    pub fn new(kademlia: &dyn KademliaBase) -> Self {
        Self {
            kademlia: kademlia.clone_dyn()
        }
    }
}

impl ResponseCallback for FindNodeResponseListener {

    fn on_response(&self, event: ResponseEvent) {
        println!("RESPONSE CALLBACK - BUCKET REFRESH {}", event.get_message().to_string());
    }

    fn on_error_response(&self, event: ErrorResponseEvent) {

    }

    fn on_stalled(&self, event: StalledEvent) {

    }
}

/*
    private class FindNodeResponseListener extends ResponseCallback {

        private PingResponseListener listener;
        private List<Node> queries;

        public FindNodeResponseListener(){
            listener = new PingResponseListener(getRoutingTable());
            queries = new ArrayList<>();
        }

        @Override
        public void onResponse(ResponseEvent event){
            event.getNode().setSeen();
            System.out.println("SEEN FN "+event.getNode());
            FindNodeResponse response = (FindNodeResponse) event.getMessage();

            if(response.hasNodes()){
                List<Node> nodes = response.getAllNodes();

                long now = System.currentTimeMillis();
                for(int i = nodes.size()-1; i > -1; i--){
                    if(queries.contains(nodes.get(i)) || getRoutingTable().hasQueried(nodes.get(i), now)){
                        nodes.remove(nodes.get(i));
                    }
                }

                queries.addAll(nodes);

                for(Node n : nodes){
                    if(getRoutingTable().isSecureOnly() && !n.hasSecureID()){
                        System.out.println("SKIPPING "+now+"  "+n.getLastSeen()+"  "+n);
                        continue;
                    }

                    PingRequest req = new PingRequest();
                    req.setDestination(n.getAddress());
                    try{
                        getServer().send(req, n, listener);
                    }catch(IOException e){
                        e.printStackTrace();
                    }
                }
            }
        }

        @Override
        public void onErrorResponse(ErrorResponseEvent event){
            event.getNode().setSeen();
        }

        @Override
        public void onStalled(StalledEvent event){
            event.getNode().markStale();
        }
    }
*/
