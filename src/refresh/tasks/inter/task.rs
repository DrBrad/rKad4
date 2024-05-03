use crate::kad::kademlia_base::KademliaBase;

pub trait Task: Send + Sync {

    fn execute(&self, kademlia: &Box<dyn KademliaBase>);

    fn clone_dyn(&self) -> Box<dyn Task>;
}

impl Clone for Box<dyn Task> {

    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
