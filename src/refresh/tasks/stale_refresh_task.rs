use crate::kad::kademlia_base::KademliaBase;
use super::inter::task::Task;

#[derive(Clone)]
pub struct StaleRefreshTask {

}

impl StaleRefreshTask {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Task for StaleRefreshTask {

    fn execute(&self, kademlia: &Box<dyn KademliaBase>) {
        println!("StaleRefresh");
    }

    fn clone_dyn(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}
