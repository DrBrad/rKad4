use crate::utils::node::Node;

const MAX_BUCKET_SIZE: u32 = 5;
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

    fn insert(&self, n: Node) {
        if self.nodes.contains(n) {

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
    fn size(&self) -> u32 {
        self.nodes.len()
    }

    fn csize() -> u32 {
        0
    }
    */
}
