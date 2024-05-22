use crate::kad::kademlia_base::KademliaBase;
use super::inter::task::Task;

#[derive(Clone)]
pub struct StaleRefreshTask {
    kademlia: Box<dyn KademliaBase>
}

impl StaleRefreshTask {

    pub fn new(kademlia: &dyn KademliaBase) -> Self {
        Self {
            kademlia: kademlia.clone_dyn()
        }
    }
}

impl Task for StaleRefreshTask {

    fn execute(&self) {
        println!("StaleRefresh");
    }

    fn clone_dyn(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}
