use std::{ops::Index, rc::Rc, cell::RefCell, fmt::{Display, Write, Debug}, collections::HashSet};

use std::hash::Hash;

fn main() {
    // read input
    let content: String = std::fs::read_to_string("input")
    .expect("Should have been able to read the file");
    // init
    let map_rc = Rc::new(RefCell::new(Map::new()));
    {
        let mut borrowed = map_rc.borrow_mut();
        borrowed.fill(content);
        //borrowed.show();
    }
    // work
    let borrowed = map_rc.borrow();
    //let start = borrowed.start();
    let end = borrowed.end();
    drop(borrowed);
    let mut borrowed = map_rc.borrow_mut();
    // part 1
    // borrowed.breadth_first(start.clone(), end.clone());
    // part 2
    borrowed.breadth_first_tweaked(end.clone(), 1);
}

#[derive(Clone)]
struct Cell {
    x: usize,
    y: usize,
    value: u8,
    parent: Box<Option<Cell>>,
}

impl Cell {
    pub fn new(x: usize, y: usize, value: u8) -> Cell {
        Cell { x, y, value, parent: Box::new(None) }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Cell {
}

impl Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char(self.value))
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{};{} = {}]", self.x, self.y, self.value))
    }
}

#[derive(Clone)]
struct Map {
    cells: Vec<Cell>,
    length: (usize, usize),
    visited: HashSet<Cell>,
}

impl Map {
    pub fn new() -> Map {
        Map { cells: vec![], length: (0, 0), visited: HashSet::new() }
    }

    /// Add a single `Cell` to a `Map`
    pub fn push(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    /// Fill attributes of a `Map` instance
    pub fn fill(&mut self, content: String) {
        // reading content
        let lines: Vec<String> = content.split("\n").map(|s| String::from(s)).collect();
        self.length = (lines.len(), lines.get(0).unwrap().chars().count());
        // filling map
        let mut line_count = 0;
        for line in lines {
            let mut char_count = 0;
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                let cell = Cell::new(line_count, char_count, height(c));
                self.push(cell);
                char_count += 1;
            }
            line_count += 1;
        }
    }


    /// Replace an existing cell
    pub fn replace(&mut self, cell: Cell) {
        let index = self.cells.iter().position(|c| c.x == cell.x && c.y == cell.y).unwrap();
        let _ = std::mem::replace(&mut self.cells[index], cell);
    }

    /// Return neighbors of a `Cell`
    pub fn neighbors(&self, cell: Cell) -> Vec<Cell> {
        let mut n = vec![];
        if cell.x != 0 {
            n.push(self[(cell.x-1, cell.y)].clone());
        }
        if cell.x != self.length.0 - 1 {
            n.push(self[(cell.x+1, cell.y)].clone());
        }
        if cell.y != 0 {
            n.push(self[(cell.x, cell.y-1)].clone());
        }
        if cell.y != self.length.1 - 1 {
            n.push(self[(cell.x, cell.y+1)].clone());
        }
        n
    }

    /// Return the starting Cell
    pub fn start(&self) -> Cell {
        self.cells.iter().filter(|c| c.value == 0).next().unwrap().clone()
    }

    /// Return the ending Cell
    pub fn end(&self) -> Cell {
        self.cells.iter().filter(|c| c.value == 27).next().unwrap().clone()
    }

    pub fn breadth_first(&mut self, start: Cell, end: Cell) {
        let cell = start.clone();
        let mut queue = vec![];
        self.visited.insert(cell.clone());
        queue.push(cell.clone());
        while queue.len() != 0 {
            let cell = queue.remove(0);
            if cell == end { // We found E !
                let mut count = 0;
                let mut cell = cell;
                while let Some(c) = *cell.parent {
                    count += 1;
                    cell = c;
                }
                println!("size {count}");
                return;
            }
            for mut neighbor in self.neighbors(cell.clone()) {
                if self.visited.contains(&neighbor) {
                    continue;
                }
                if neighbor.value <= cell.value + 1 {
                    self.visited.insert(neighbor.clone());
                    neighbor.parent = Box::new(Some(cell.clone()));
                    self.replace(neighbor.clone());
                    queue.push(neighbor);
                }
            }
        }
    }

    pub fn breadth_first_tweaked(&mut self, start: Cell, end: u8) {
        let cell = start.clone();
        let mut queue = vec![];
        self.visited.insert(cell.clone());
        queue.push(cell.clone());
        while queue.len() != 0 {
            let cell = queue.remove(0);
            //println!("{cell:?}");
            // if cell == end { // We found E !
            if cell.value == end {
                let mut count = 0;
                let mut cell = cell;
                while let Some(c) = *cell.parent {
                    count += 1;
                    cell = c;
                }
                println!("size {count}");
                return;
            }
            for mut neighbor in self.neighbors(cell.clone()) {
                if self.visited.contains(&neighbor) {
                    continue;
                }
                //println!("  {neighbor:?} {}", neighbor.value + 1 == cell.value || neighbor.value == cell.value);
                if cell.value - 1 <= neighbor.value {
                    self.visited.insert(neighbor.clone());
                    neighbor.parent = Box::new(Some(cell.clone()));
                    self.replace(neighbor.clone());
                    queue.push(neighbor);
                }
            }
        }
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells.iter().filter(|c| c.x == index.0 && c.y == index.1).next().unwrap()
    }
}


/// Convert char to number
fn height(c: char) -> u8 {
    match c {
        'S' => 0,
        'E' => 27,
        _ => c as u8 - 97 + 1, // 'a' as u8 == 97
    }
}

fn char(n: u8) -> char {
    match n {
        0 => 'S',
        27 => 'E',
        _ => (n + 97 - 1) as char
    }
}
