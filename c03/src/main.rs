use std::fs;
use std::collections::HashSet;

struct Bag {
    size: usize,
    content: String
}

impl Bag {
    pub fn new(size: usize, content: String) -> Bag {
        Bag {size, content}
    }

    pub fn common_within(&self) -> Option<char> {
        assert!(self.size % 2 == 0);
        let size = self.size / 2;
        let mut first_chars = HashSet::new();
        let mut second_chars = HashSet::new();
        for i in 0..size {
            first_chars.insert(self.content.chars().nth(i).unwrap());
            second_chars.insert(self.content.chars().nth(i+size).unwrap());
        }
        for ch in &first_chars {
            if second_chars.contains(ch) {
                return Some(*ch);
            }
        }
        // We know it won't happen
        None
    }

    pub fn set(&self) -> HashSet<char> {
        let mut chars = HashSet::new();
        for i in 0..self.size {
            chars.insert(self.content.chars().nth(i).unwrap());
        }
        chars
    }
}

struct Group {
    bags: Vec<Bag>
}

impl Group {
    pub fn new(bags: Vec<Bag>) -> Group {
        assert!(bags.len() == 3);
        Group { bags }
    }

    pub fn common(&self) -> char {
        let a = self.bags[0].set();
        let b = self.bags[1].set();
        let c = self.bags[2].set();
        let ab: HashSet<&char> = a.intersection(&b).collect();
        let ac = a.intersection(&c).collect();
        let inter: Vec<&&char> = ab.intersection(&ac).collect();
        assert!(inter.len() == 1);
        **inter[0]
    }
}

pub fn priority(item: char) -> u8 {
    let nbr = item as u8;
    if nbr > 96 { // not capital, a = 97
        nbr - 96
    } else { // capital, A = 65
        nbr - 64 + 26
    }
}

fn main() {
    // read input
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    // init
    let mut total = 0_u32;
    let mut groups = vec![];
    let mut bags = vec![];
    let bags_content: Vec<&str> = contents.split("\n").collect();
    for content in bags_content {
        let bag_content = String::from(content);
        bags.push(Bag::new(bag_content.len(), bag_content));

        if bags.len() == 3 {
            groups.push(Group::new(bags));
            bags = vec![];
        }
    }
    // work
    for group in groups {
        total += priority(group.common()) as u32;
    }
    println!("total: {total}");
}
