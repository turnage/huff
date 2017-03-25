pub mod data;

use std::io::Read;

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
    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Err(e) => println!("Failed to read frequencies from stdin: {}", e),
        Ok(_) => {
            let mut pairs = Vec::new();
            for line in buffer.lines() {
                let mut terms = line.split_whitespace();
                let freq = terms.next().unwrap().parse::<usize>().unwrap();
                let elem = terms.next().unwrap();
                pairs.push(KVPair::from(freq, Node::from(String::from(elem))));
            }
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

            for (elem, code) in root.codes(String::from("")) {
                println!("{}\t{}", elem, code);
            }
        }
    };
}
