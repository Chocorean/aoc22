use std::{rc::{Rc, Weak}, cell::{RefCell}};

#[derive(Clone)]
struct Node {
    pub parent: Option<Weak<RefCell<Node>>>,
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
    size: Option<u64>,
}

impl Node {
    pub fn new(parent: Option<Weak<RefCell<Node>>>, name: String, size: Option<u64>) -> Node {
        Node { parent, name, children: vec![], size }
    }

    pub fn parent(&self) -> Option<Weak<RefCell<Node>>> {
        match &self.parent {
            None => None,
            Some(weak) => Some(Weak::clone(&weak))
        }
    }

    /// Return absolute path of `self`
    pub fn name(&self) -> String {
        match &self.parent {
            Some(node) => {
                let mut name = node.upgrade().unwrap().borrow().name().clone();
                name.push('/');
                name.push_str(&self.name.clone());
                name
            },
            None => self.name.clone()
        }
    }

    /// Return size of `self.children` is `self.size` is `None`, `self.size` otherwise
    fn size(&self) -> u64 {
        match self.size {
            None => self.children.iter().map(|node| node.borrow().size()).sum(),
            Some(s) => s
        }
    }

    /// Print `self` size and all underlying directories' sizes
    /// For debug purpose only
    pub fn show_size(&self) {
        match self.size {
            None => {
                println!("size of {}: {}", self.name(), self.size());
                for child in &self.children {
                    let child = child.borrow();
                    if child.is_dir() {
                        child.show_size();
                    }
                }
            },
            Some(_) => {} // Don't do anything for files yet
        }
    }

    pub fn gather_below(&self, threshold: u64) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes = vec![];
        for child in &self.children {
            let borrowed = child.borrow();
            if borrowed.is_dir() {
                if borrowed.size() < threshold {
                    nodes.push(Rc::clone(child));
                }
                nodes.extend(borrowed.gather_below(threshold))
            }
        }
        nodes
    }

    /// Please don't blame me I'm tired let's just duplicate
    pub fn gather_above(&self, threshold: u64) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes = vec![];
        for child in &self.children {
            let borrowed = child.borrow();
            if borrowed.is_dir() {
                if borrowed.size() >= threshold {
                    nodes.push(Rc::clone(child));
                }
                nodes.extend(borrowed.gather_above(threshold))
            }
        }
        nodes
    }

    fn is_dir(&self) -> bool {
        match self.size {
            None => true,
            Some(_) => false
        }
    }

    pub fn child(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        for child in &self.children {
            let borrowed = child.borrow();
            if borrowed.name == name {
                return Some(Rc::clone(child));
            }
        }
        None
    }

    pub fn push(&mut self, child: Node) {
        self.children.push(Rc::new(RefCell::new(child)));
    }
}


fn main() {
    // read input
    let lines: Vec<String> = std::fs::read_to_string("input")
        .expect("Should have been able to read the file").split("\n").map(|s| String::from(s)).collect();
    // init
    let root_node = Node::new(None, String::from("."), None);
    let rc_root = Rc::new(RefCell::new(root_node));
    let mut current_node = Rc::clone(&rc_root);
    // work
    for i in 1..lines.len() {  // ignoring `$ cd /`
        let line = lines.get(i).unwrap();
        {
            //println!("{} {}", current_node.borrow().name(), line);
        }
        if line.contains("$ ls") {  // ignoring al `& ls` lines
            continue
        } else if line.contains("$ cd") {  // update `current_node`
            let dirname = &line[5..];
            let local_rc = Rc::clone(&current_node);
            let borrowed = local_rc.borrow();
            if dirname == ".." {
                current_node = borrowed.parent().unwrap().upgrade().unwrap();
            } else { 
                current_node = borrowed.child(dirname).unwrap();
            }
        } else {  // can be either `dir <dirname>` or `<size> <filename>`
            if line.contains("dir ") {
                let dirname = String::from(&line[4..]);
                let new_node = Node::new(Some(Rc::downgrade(&current_node)), dirname, None);
                let local_rc = Rc::clone(&current_node);
                let mut borrowed = local_rc.borrow_mut();
                borrowed.push(new_node);
            } else {
                let parts: Vec<&str> = line.split(" ").collect();
                let size: Option<u64> = Some(parts[0].parse().expect("NaN"));
                let name = String::from(parts[1]);
                let new_node = Node::new(Some(Rc::downgrade(&current_node)), name, size);
                let local_rc = Rc::clone(&current_node);
                let mut borrowed = local_rc.borrow_mut();
                borrowed.push(new_node)
            }
        }
    }
    // DEBUG: display size of all dirs
    rc_root.borrow().show_size();

    // chall 1
    {
        let dirs = rc_root.borrow().gather_below(100000);
        println!("{}", dirs.iter().map(|rc| rc.borrow().size()).sum::<u64>());
    }

    // chall 2
    let current_size = rc_root.borrow().size();
    let available = 70000000 - current_size;
    let needed = 30000000_u64;
    let mininum_size = needed - available;
    let dirs = rc_root.borrow().gather_above(mininum_size);
    println!("{}", dirs.iter().reduce(|a, b| if a.borrow().size() < b.borrow().size() {a} else {b}).unwrap().borrow().size())
}
