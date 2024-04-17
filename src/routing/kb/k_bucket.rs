use crate::utils::node::Node;

const MAX_BUCKET_SIZE: usize = 5;
const MAX_STALE_COUNT: u32 = 1;

#[derive(Clone)]
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
        if let Some(node) = self.nodes.iter_mut().find(|c| n.eq(c)) {
            node.seen();
            //RE-SORT THE LIST

        } else if self.nodes.len() >= MAX_BUCKET_SIZE {
            if let Some(node) = self.cache.iter_mut().find(|c| n.eq(c)) {
                node.seen();

            } else if self.cache.len() >= MAX_BUCKET_SIZE {
                //let mut stale: Option<Node> = None;
                let mut index = MAX_BUCKET_SIZE+1;

                for i in 0..=self.cache.len() {
                    if self.cache.get(i).unwrap().stale >= MAX_STALE_COUNT {
                        if index < MAX_BUCKET_SIZE && self.cache.get(i).unwrap().stale > self.cache.get(index).unwrap().stale {
                            index = i;
                        }
                    }
                }


                if(index < MAX_BUCKET_SIZE){
                    let n = self.cache.remove(index);
                    self.cache.push(n);
                    //if let Some(ref mut existing_stale) = stale {
                    //    self.cache.remove(existing_stale);
                    //}
                    /*
                    if let Some(element) = self.cache.remove(self.cache.index(stale)) {
                        self.cache.push(element);
                    }
                    */
                }

            }else{
                self.cache.push(n);
            }
        }else{
            self.nodes.push(n);
            //RE-SORT THE LIST
        }
    }

    pub fn contains_ip(&self, n: &Node) -> bool {
        self.nodes.contains(&n) || self.cache.contains(&n)
    }

    pub fn contains_uid(&self, n: &Node) -> bool {
        self.nodes.iter().any(|c| c.verify(&n)) || self.cache.iter().any(|c| c.verify(&n))
    }

    pub fn has_queried(&self, n: &Node, now: u64) -> bool {
        /*
        for c in self.nodes {
            if c.eq(&n) {
                return c.has_queried(now);
            }
        }

        false
        */
        false
    }

    /*
    fn all_nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }
    */

    fn unqueried_nodes(&self, now: u64) -> Vec<Node> {
        self.nodes.iter().filter(|&n| !n.has_queried(now)).cloned().collect()
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