use std::fs;

use regex::Regex;

#[derive(Debug)]
pub struct Stack {
    crates: Vec<char>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { crates: vec![] }
    }

    pub fn push(&mut self, c: char) {
        self.crates.push(c);
    }

    pub fn extend(&mut self, crates: Vec<char>) {
        for c in crates {
            self.crates.push(c);
        }
    }

    pub fn pop(&mut self, quantity: usize) -> Vec<char> {
        let mut popped = vec![];
        for _ in 0..quantity {
            popped.push(self.crates.pop().unwrap());
        }
        popped.reverse();
        popped
    }

    /// For init purpose only
    pub fn reverse(&mut self) {
        self.crates.reverse();
    }

    pub fn last(&self) -> char {
        match self.crates.last() {
            Some(c) => *c,
            None => ' '
        }
    }
}


/// Return the relevant indexes at which the letters will be in the input
/// Let's try without regex first
pub fn relevant_indexes(line_size: usize) -> Vec<usize> {
    let mut indexes = vec![1];
    let mut last = 1;
    while indexes.len() != line_size {
        let new = last + 4;
        indexes.push(new as usize);
        last = new;
    }
    indexes
}


pub fn message(stacks: &mut Vec<Stack>) -> String {
    let mut str = String::from("");
    for stack in stacks {
        str.push(stack.last());
    }
    str
}

fn main() {
    // read input
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    // init
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut line = String::from(lines[0]);
    let line_size = line.chars().count();
    let stack_number = (line_size + 1) / 4; // always an integer
    // Now that we know the number of stacks, we can parse easily
    let indexes = relevant_indexes(stack_number);
    let mut stacks = vec![];
    for _ in 0..stack_number {
        stacks.push(Stack::new());
    }
    let mut line_nbr = 0;
    while line != String::from("") {
        for (i, index) in indexes.iter().enumerate() {
            let c = line.chars().nth(*index).unwrap();
            if c == '1' {
                // We hit the useless line; skipping
                break;
            }
            if c != ' ' {
                stacks[i].push(c);
            }
        }
        line_nbr += 1;
        line = String::from(lines[line_nbr]);
    }
    for i in 0..stack_number {
        stacks[i].reverse();
    }
    // 
    // now the crates are filled properly
    for i in 0..stack_number {
        println!("{:?}", stacks[i]);
    }

    // Fuck #L46
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    for line in &lines[line_nbr+1..] {
        println!("{line}"); // DEBUG
        for cap in re.captures_iter(line) {
            println!("{:?}", cap);
            let quantity: usize = (&cap[1]).to_string().parse().unwrap();
            let from: usize = (&cap[2]).to_string().parse::<usize>().unwrap() - 1;
            let to: usize = (&cap[3]).to_string().parse::<usize>().unwrap() - 1;

            let moved = stacks[from].pop(quantity);
            println!("{:?}", moved);
            stacks[to].extend(moved);
            for i in 0..stack_number {
                println!("{:?}", stacks[i]);
            }
        }
    }
    println!("{}@{}@", stacks.len(), message(&mut stacks));
}