use crate::utils::node::Node;

const MAX_BUCKET_SIZE: u32 = 5;
const MAX_STALE_COUNT: u32 = 1;

struct KBucket {
    nodes: Vec<Node>,
    cache: Vec<Node>
}

impl KBucket {

    fn new() -> Self {
        Self {
            nodes: None,
            cache: None
        }
    }

    fn insert(n: Node) {

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

}
