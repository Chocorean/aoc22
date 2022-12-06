use std::{fs, cmp::Ordering};

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    pub fn value(&self) -> u32 {
        match self {
            Move::Rock => 1_u32,
            Move::Paper => 2_u32,
            Move::Scissors => 3_u32
        }
    }

    fn cmp(&self, other: Move) -> Ordering {
        match (self, other) {
            (Move::Rock, Move::Paper) => Ordering::Less,
            (Move::Paper, Move::Scissors) => Ordering::Less,
            (Move::Scissors, Move::Rock) => Ordering::Less,
            _ => Ordering::Greater
        }
    }

    pub fn lt(&self, other: Move) -> bool {
        self.cmp(other) == Ordering::Less
    }

    pub fn gt(&self, other: Move) -> bool {
        other.cmp(*self) == Ordering::Less
    }

    pub fn strong_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper
        }
    }

    pub fn weak_against(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock
        }
    }
}

struct Round {
    foe: Move,
    me: Move
}

impl Round {
    pub fn new(foe: &str, me: &str) -> Round {
        let move_foe = match foe {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("unknown move!")
        };
        let move_me = match me {
            "X" => move_foe.strong_against(),
            "Y" => move_foe.clone(),
            "Z" => move_foe.weak_against(),
            _ => panic!("unknown move!")
        };
        Round { foe: move_foe, me: move_me }
    }

    fn result(&self) -> u32 {
        if self.foe.lt(self.me) {
            6_u32
        } else if self.foe == self.me {
            3_u32
        } else if self.foe.gt(self.me) {
            0_u32
        } else {
            panic!("the hell is going on");
        }
    }

    pub fn score(&self) -> u32 {
        self.result() + self.me.value()
    }
}

fn main() {
    // read input
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file"); 
    // init
    let mut score: u32 = 0;
    let mut rounds = vec![];
    for line in contents.split("\n").collect::<Vec<&str>>() {
        let moves = line.split(" ").collect::<Vec<&str>>();
        rounds.push(Round::new(moves[0], moves[1]));
    }
    // work
    println!("total: {}", rounds.iter().map(|r| r.score()).reduce(|a,b| a+b).unwrap());
}
