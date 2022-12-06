use std::{fs, collections::HashSet};

struct Window {
    chars: String,
    index: usize,
    size: usize,
}



impl Window {
    pub fn new(chars: String) -> Window {
        Window { chars, index: 0, size: 14 }
    }

    /// if step is 0, consider it as 1
    pub fn next(&mut self, step: usize) {
        let step = if step == 0 { 1 } else { step };
        self.index += step;
    }

    pub fn is_unique(&self) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        let chars: Vec<char> = self.chars.chars().into_iter().collect();
        for ch in &chars[self.index..self.index+self.size] {
            if set.contains(ch) {
                return false;
            }
            set.insert(*ch);
        }
        true
    }

    pub fn position(&self) -> usize {
        self.index + self.size
    }
}

fn main() {
    // read input
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    // init
    let mut win = Window::new(contents);
    while ! win.is_unique() {
        win.next(0);
    }
    println!("{}", win.position());
}
