use std::ops::Index;
use crate::utils::node::Node;

const MAX_BUCKET_SIZE: usize = 5;
const MAX_STALE_COUNT: u32 = 1;

struct KBucket { //CHANGE TO HASH CODE SYSTEM...
    nodes: Vec<Node>,
    cache: Vec<Node>
}

impl KBucket {

    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            cache: Vec::new()
        }
    }

    fn insert(&self, n: Node) { //&mut self
        if self.nodes.contains(&n) {
            self.nodes.get(self.cache.index(&n)).unwrap().seen();
            //RE-SORT THE LIST

        } else if self.nodes.len() >= MAX_BUCKET_SIZE {
            if self.cache.contains(&n) {
                self.cache.get(self.cache.index(&n)).unwrap().seen();

            } else if self.cache.len() >= MAX_BUCKET_SIZE {
                let mut stale: Node;


            }

        }


    }

    fn contains_ip(n: Node) -> bool {
        false
    }

    fn contains_uid(n: Node) -> bool {
        false
    }

    fn has_queried(n: Node, now: u64) -> bool {
        false
    }

    fn all_nodes() -> Vec<Node> {

    }

    fn unqueried_nodes(now: u64) -> Vec<Node> {

    }

    /*
    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn csize(&self) -> usize {
        self.cache.len()
    }
    */
}
