use std::ops::Index;
use crate::utils::node::Node;

const MAX_BUCKET_SIZE: usize = 5;
const MAX_STALE_COUNT: u32 = 1;

pub struct KBucket { //CHANGE TO HASH CODE SYSTEM...
    pub(crate) nodes: Vec<Node>,
    pub(crate) cache: Vec<Node>
}

impl KBucket {

    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            cache: Vec::new()
        }
    }

    pub fn insert(&mut self, n: Node) {
        if self.nodes.contains(&n) {
            self.nodes.get(self.cache.index(&n)).unwrap().seen();
            //RE-SORT THE LIST

        } else if self.nodes.len() >= MAX_BUCKET_SIZE {
            if self.cache.contains(&n) {
                self.cache.get(self.cache.index(&n)).unwrap().seen();

            } else if self.cache.len() >= MAX_BUCKET_SIZE {
                let mut stale: Node = None;

                for s in self.cache {
                    if s.stale >= MAX_STALE_COUNT {
                        if stale == None || s.stale > stale.stale {
                            stale = s;
                        }
                    }
                }


                if(stale != None){
                    if let Some(element) = self.cache.remove(self.cache.index(stale)) {
                        self.cache.push(element);
                    }
                }

            }else{
                self.cache.push(&n);
            }
        }else{
            self.nodes.push(&n);
            //RE-SORT THE LIST
        }
    }

    pub fn contains_ip(&self, n: Node) -> bool {
        self.nodes.contains(&n) || self.cache.contains(&n)
    }

    pub fn contains_uid(&self, n: Node) -> bool {
        self.nodes.iter().any(|c| c.verify(&n)) || self.cache.iter().any(|c| c.verify(&n))
    }

    pub fn has_queried(&self, n: Node, now: u64) -> bool {
        for c in self.nodes {
            if c.eq(&n) {
                return c.has_queried(now);
            }
        }

        false
    }

    /*
    fn all_nodes() -> Vec<Node> {

    }

    fn unqueried_nodes(now: u64) -> Vec<Node> {

    }

    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn csize(&self) -> usize {
        self.cache.len()
    }
    */
}
