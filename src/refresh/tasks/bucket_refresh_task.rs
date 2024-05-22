use crate::kad::kademlia_base::KademliaBase;
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
        println!("BucketRefresh");
        /*
        FindNodeResponseListener listener = new FindNodeResponseListener();
        System.out.println("EXECUTING BUCKET REFRESH");

        for(int i = 1; i < UID.ID_LENGTH*8; i++){
            if(getRoutingTable().getBucketSize(i) < KBucket.MAX_BUCKET_SIZE){ //IF THE BUCKET IS FULL WHY SEARCH... WE CAN REFILL BY OTHER PEER PINGS AND LOOKUPS...
                UID k = getRoutingTable().getDerivedUID().generateNodeIdByDistance(i);

                List<Node> closest = getRoutingTable().findClosest(k, KBucket.MAX_BUCKET_SIZE);
                if(closest.isEmpty()){
                    continue;
                }

                for(Node n : closest){
                    FindNodeRequest request = new FindNodeRequest();
                    request.setDestination(n.getAddress());
                    request.setTarget(k);

                    try{
                        getServer().send(request, n, listener);

                    }catch(IOException e){
                        e.printStackTrace();
                    }
                }
            }
        }
        */
    }

    fn clone_dyn(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}
