pub mod data;

use data::Countable;
use data::elem::KVPair;
use data::heap::BinaryHeap;
use data::abstr::PriorityQueue;

const EMPTY: &'static str = "";

#[derive(Debug)]
struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn from(v: String) -> Self {
        Node {
            val: v,
            left: None,
            right: None,
        }
    }

    fn codes(&self, path: String) -> Vec<(String, String)> {
        let mut codes = Vec::new();
        if self.val != EMPTY {
            codes.push((String::from(self.val.as_str()), String::from(path.as_str())))
        }
        if let Some(ref lc) = self.left {
            codes.append(&mut lc.codes(format!("{}0", &path)));
        }
        if let Some(ref rc) = self.right {
            codes.append(&mut rc.codes(format!("{}1", &path)));
        }
        codes
    }
}

fn main() {
    let pairs = vec![KVPair::from(1, Node::from(String::from("e"))),
                     KVPair::from(2, Node::from(String::from("d"))),
                     KVPair::from(3, Node::from(String::from("c"))),
                     KVPair::from(4, Node::from(String::from("b"))),
                     KVPair::from(5, Node::from(String::from("a")))];
    let mut pq = BinaryHeap::min();

    for p in pairs {
        pq.enqueue(p)
    }

    while pq.len() > 1 {
        let (pkey, pval) = pq.dequeue().unwrap().consume();
        let (qkey, qval) = pq.dequeue().unwrap().consume();
        let r = Node {
            val: String::from(EMPTY),
            left: Some(Box::new(pval)),
            right: Some(Box::new(qval)),
        };
        pq.enqueue(KVPair::from(pkey + qkey, r));
    }

    let (_, root) = pq.dequeue().unwrap().consume();

    println!("{:?}", root.codes(String::from("")));
}
