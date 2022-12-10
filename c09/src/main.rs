use std::{collections::HashSet, rc::Rc, cell::RefCell};

fn main() {
    // read input
    let lines: Vec<String> = std::fs::read_to_string("input")
        .expect("Should have been able to read the file").split("\n").map(|s| String::from(s)).collect();
    // init
    let mut rope = Rope::new(10);
    for line in lines {
        rope.move_head(line);
    }
    println!("{} < 3060", rope.count());
}

#[derive(Clone, Debug)]
struct Knot {
    x: i64,
    y: i64,
}

impl Knot {
    pub fn new() -> Knot {
        Knot { x: 0, y: 0 }
    }

    /// y += 1
    fn up(&mut self) {
        self.y += 1;
    }

    /// y -= 1
    fn down(&mut self) {
        self.y -= 1;
    }

    /// x += 1
    fn right(&mut self) {
        self.x += 1;
    }

    /// x -= 1
    fn left(&mut self) {
        self.x -= 1;
    }

    fn is_neighbor(&self, other: &Knot) -> bool {
        (self.x - other.x).abs() < 2 && (self.y - other.y).abs() < 2
    }

    fn align_x(&mut self, other: &Knot) -> String {
        //self.x smaller then going right
        let mut dir = "";
        if self.x < other.x {
            dir = "R";
            self.x += 1;
        } else {
            dir = "L";
            self.x -= 1;
        };
        String::from(dir)
    }

    fn align_y(&mut self, other: &Knot) -> String {
        //self.y smaller then going up
        let mut dir= "";
        if self.y < other.y {
            dir = "U";
            self.y += 1;
        } else { 
            dir = "D";
            self.y -= 1;
        };
        String::from(dir)
    }
}

#[derive(Clone)]
struct Rope {
    knots: Vec<Rc<RefCell<Knot>>>,
    tail_pos: HashSet<(i64, i64)>,
}

impl Rope {
    pub fn debug(&self) {
        for (i, knot) in self.knots.iter().enumerate() {
            let rc = Rc::clone(knot);
            let borrow = rc.try_borrow().unwrap();
            let sym = if i == 0 { "H".to_string() } else { i.to_string() };
            println!("{sym} {},{}", borrow.x, borrow.y);
        }
        println!("---");
    }

    pub fn new(size: usize) -> Rope {
        let mut knots = vec![];
        for _ in 0..size {
            knots.push(Rc::new(RefCell::new(Knot::new())));
        }
        let mut tail_pos = HashSet::new();
        tail_pos.insert((0, 0));
        Rope { knots, tail_pos }
    }

    /// Return a reference to the RC of a knot
    fn knot(&self, index: usize) -> &Rc<RefCell<Knot>> {
        self.knots.get(index).unwrap()
    }

    /// DIRECTIONS

    fn up(&mut self, index: usize) {
        let rc = self.knots.get(index).unwrap();
        let mut borrowed = rc.borrow_mut();
        borrowed.up();
    }

    fn down(&mut self, index: usize) {
        let rc = self.knots.get(index).unwrap();
        let mut borrowed = rc.borrow_mut();
        borrowed.down();
    }

    fn left(&mut self, index: usize) {
        let rc = self.knots.get(index).unwrap();
        let mut borrowed = rc.borrow_mut();
        borrowed.left();
    }

    fn right(&mut self, index: usize) {
        let rc = self.knots.get(index).unwrap();
        let mut borrowed = rc.borrow_mut();
        borrowed.right();
    }

    /// Read one line. Move head one position and update tails once at the time
    fn move_head(&mut self, line: String) {
        let words = line.split(" ").collect::<Vec<&str>>();
        let (dir, dist) = (*words.get(0).unwrap(), words.get(1).unwrap().parse::<u64>().unwrap());
        for i in 0..dist {
            match dir {
                "U" => self.up(0),
                "D" => self.down(0),
                "L" => self.left(0),
                "R" => self.right(0),
                _ => panic!("unknown instruction")
            };
            let (x, y) = self.update_tails();
            self.add_tail_pos(x, y);
            //println!("{line} #{i}");
            //self.debug();
        }
    }

    /// Update all tails one by one.
    /// Return the position of the last tail.
    fn update_tails(&mut self) -> (i64, i64) {
        let rope_size = self.knots.len();
        let mut previous_knot_rc = Rc::clone(self.knot(0));
        for i in 1..rope_size {
            let current_knot_rc = Rc::clone(self.knot(i));
            let mut current_borrowed_mut = current_knot_rc.borrow_mut();
            let previous_borrowed = previous_knot_rc.try_borrow().unwrap();
            //println!("{current_borrowed_mut:?} {previous_borrowed:?}");
            // dont do anything if within 8-neighborhood
            if current_borrowed_mut.is_neighbor(&previous_borrowed) {
                // Don't forget to update !
                drop(previous_borrowed);
                previous_knot_rc = Rc::clone(self.knot(i));
                continue;
            }
            // we will move the current knot position below
            // if only one axis is different, update it
            if current_borrowed_mut.x == previous_borrowed.x && current_borrowed_mut.y != previous_borrowed.y {
                if current_borrowed_mut.y < previous_borrowed.y {
                    current_borrowed_mut.up();
                } else {
                    current_borrowed_mut.down();
                }
            } else if current_borrowed_mut.x != previous_borrowed.x && current_borrowed_mut.y == previous_borrowed.y {
                if current_borrowed_mut.x < previous_borrowed.x {
                    current_borrowed_mut.right();
                } else {
                    current_borrowed_mut.left();
                }
            } else {
                // else, need to move two directions
                if current_borrowed_mut.x < previous_borrowed.x {
                    current_borrowed_mut.right();
                } else {
                    current_borrowed_mut.left();
                }
                if current_borrowed_mut.y < previous_borrowed.y {
                    current_borrowed_mut.up();
                } else {
                    current_borrowed_mut.down();
                }
            }
            // finally update `previous_knot` and repeat
            drop(previous_borrowed);
            previous_knot_rc = Rc::clone(self.knot(i));
        }
        let tail_rc = Rc::clone(self.knot(rope_size-1));
        let tail_borrowed = tail_rc.try_borrow().unwrap();
        (tail_borrowed.x, tail_borrowed.y)
    }

    fn add_tail_pos(&mut self, x: i64, y: i64) {
        self.tail_pos.insert((x, y));
    }

    /*fn update_tail(&mut self, initial_dir: &str) {
        // dont do anything if within 8-neighborhood
        if self.tail.is_neighbor(&self.head) {
            return;
        }
        match dir {
            "U" | "D" => {
                match dir {
                    "U" => self.tail.up(),
                    "D" => self.tail.down(),
                    _ => panic!("unknown instruction")
                }
                // alignment
                if self.head.x != self.tail.x {
                    self.tail.align_x(&self.head);
                }
            }
            "L" | "R" => {
                match dir {
                    "L" => self.tail.left(),
                    "R" => self.tail.right(),
                    _ => panic!("unknown instruction")
                }
                // alignment
                if self.head.y != self.tail.y {
                    self.tail.align_y(&self.head);
                }
            }
            _ => panic!("unknown instruction")
        };
        // record tail position
        self.tail_pos.insert((self.tail.x, self.tail.y));
    }*/

    pub fn count(&self) -> usize {
        self.tail_pos.len()
    }
}