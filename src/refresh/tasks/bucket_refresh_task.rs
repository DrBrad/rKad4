use crate::kad::kademlia_base::KademliaBase;
use crate::routing::kb::k_bucket::MAX_BUCKET_SIZE;
use crate::utils::uid::ID_LENGTH;
use super::inter::task::Task;

#[derive(Clone)]
pub struct BucketRefreshTask {

}

impl BucketRefreshTask {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Task for BucketRefreshTask {

    fn execute(&self, kademlia: &Box<dyn KademliaBase>) {
        let listener; //FindNodeResponseListener listener = new FindNodeResponseListener();
        println!("EXECUTING BUCKET REFRESH");

        for i in 1..ID_LENGTH*8 {
            if kademlia.get_routing_table().lock().unwrap().bucket_size(i) < MAX_BUCKET_SIZE {
                let k = kademlia.get_routing_table().lock().unwrap().get_derived_uid().generate_node_id_by_distance(i);

                let closest = kademlia.get_routing_table().lock().unwrap().find_closest(&k, MAX_BUCKET_SIZE);
                if closest.is_empty() {
                    continue;
                }

                for node in closest {
                    /*
                    FindNodeRequest request = new FindNodeRequest();
                    request.setDestination(n.getAddress());
                    request.setTarget(k);

                    try{
                        getServer().send(request, n, listener);

                    }catch(IOException e){
                        e.printStackTrace();
                    }
                    */
                }
            }
        }


    }

    fn clone_dyn(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}
