use std::fmt::Debug;
use std::cmp::Ordering;


// Stolen from fasterthanlime. I was not really inspired.
use serde::Deserialize;
#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}
// End of steal


impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Node::Number(n) => {
                match other {
                    // two values
                    Node::Number(n2) => {
                        Some(n.cmp(n2))
                    },
                    // one value and one list
                    Node::List(_) => {
                        Node::List(vec![Node::Number(*n)]).partial_cmp(other)
                    }
                }
            },
            Node::List(l) => {
                match other {
                    // one list and one value
                    Node::Number(n2) => {
                        self.partial_cmp(&Node::List(vec![Node::Number(*n2)]))
                    },
                    // this is the tricky part: comparing two lists
                    Node::List(l2) => {
                        for i in 0..l.len().max(l2.len()) {
                            let a = l.get(i);
                            match a {
                                Some(node) => {
                                    let b = l2.get(i);
                                    match b {
                                        // compare two nodes
                                        Some(node2) => {
                                            let order = node.partial_cmp(node2).unwrap();
                                            if node.partial_cmp(node2).unwrap().is_eq() {
                                                continue;
                                            } else {
                                                return Some(order);
                                            }
                                        }
                                        // b shorter than a
                                        None => {
                                            //println!("  b shorter than a {l:?} // {l2:?}");
                                            return Some(Ordering::Greater)
                                        }
                                    }
                                },
                                // a shorter or same sime than b
                                None => {
                                    let b = l2.get(i);
                                    match b {
                                        // shorter
                                        Some(_) => return Some(Ordering::Less),
                                        // same size
                                        None => {
                                            return Some(Ordering::Equal)
                                        }
                                    }
                                }
                            };
                        }
                        // we kept `continue`-ing
                        Some(Ordering::Equal)
                    }
                }
            }
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Number(n) => f.write_fmt(format_args!("{n}")),
            Node::List(l) => f.debug_list().entries(l).finish(),
        }
    }
}

impl Node {
    pub fn parse(line: &String) -> Node {
        serde_json::from_str::<Node>(line).unwrap()
    }
}

fn main() {
    // read input
    let content: Vec<String> = std::fs::read_to_string("input")
        .expect("Should have been able to read the file").split("\n\n").map(|s| String::from(s)).collect();
    // init
    let mut indexes = vec![];
    let mut nodes = vec![];
    // work
    for (index, duo) in content.iter().enumerate() {
        let packets: Vec<String> = duo.split("\n").map(|s| String::from(s)).collect();
        let node_a = Node::parse(packets.get(0).unwrap());
        let node_b = Node::parse(packets.get(1).unwrap());
        if node_a <= node_b {
            indexes.push(index+1);
        }
        nodes.push(node_a);
        nodes.push(node_b);
    }
    println!("{:?}, {}", indexes, indexes.iter().map(|e| *e).reduce(|a, b| a + b).unwrap_or(0));
    // part 2
    let dividers = vec![Node::parse(&String::from("[[2]]")), Node::parse(&String::from("[[6]]"))];
    nodes.extend(dividers.clone());
    nodes.sort();
    // this line was stolen as well from FTL, glad I discovered `binary_search`. Much better than my hack for day12
    let total: usize = dividers.iter().map(|d| nodes.binary_search(d).unwrap() + 1).product();
    println!("{total}");
}