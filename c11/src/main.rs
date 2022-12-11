use std::{cell::RefCell, rc::Rc};

use regex::Regex;

fn main() {
    // read input
    let binding = std::fs::read_to_string("input")
        .expect("Should have been able to read the file");
    let mut lines = binding.lines();
    // regex
    let monkey_r = Regex::new(r"Monkey (\d+):").unwrap();
    let items_r = Regex::new(r"  Starting items: (\d+(?:, \d+)*)").unwrap();
    let operation_r = Regex::new(r"  Operation: new = old ([+*-]) ((?:\d+)|(?:old))").unwrap();
    let test_r = Regex::new(r"  Test: divisible by (\d+)").unwrap();
    let true_r = Regex::new(r"    If true: throw to monkey (\d+)").unwrap();
    let false_r = Regex::new(r"    If false: throw to monkey (\d+)").unwrap();
    // init
    let mut monkeys = vec![];
    while let Some(line) = lines.next() {
        // monkey
        let mut captures: Vec<_> = monkey_r.captures_iter(line).collect();
        let current_monkey = captures[0][1].parse::<u64>().unwrap();
        // items
        let mut line = lines.next().unwrap();
        captures = items_r.captures_iter(line).collect();
        let current_items = captures[0][1].split(", ").collect::<Vec<&str>>().iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        // operation
        line = lines.next().unwrap();
        captures = operation_r.captures_iter(line).collect();
        let current_operation = Operation::new(&captures[0][1], &captures[0][2]);
        // test
        line = lines.next().unwrap();
        captures = test_r.captures_iter(line).collect();
        let current_test = captures[0][1].parse::<u64>().unwrap();
        // true
        line = lines.next().unwrap();
        captures = true_r.captures_iter(line).collect();
        let current_true = captures[0][1].parse::<u64>().unwrap();
        // false
        line = lines.next().unwrap();
        captures = false_r.captures_iter(line).collect();
        let current_false = captures[0][1].parse::<u64>().unwrap();
        // empty line
        lines.next();
        // build monkey
        let monkey = Monkey::new(current_monkey, current_items, current_operation, current_test, current_true, current_false);
        monkeys.push(Rc::new(RefCell::new(monkey)));
    }
    let pgdc = monkeys.iter().map(|e| Rc::clone(e).borrow().test).reduce(|a,b| a*b).unwrap(); // don't know the acronym in English
    // work
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey_rc = Rc::clone(monkeys.get(i).unwrap());
            let mut borrowed = monkey_rc.borrow_mut();
            borrowed.play(&mut monkeys, pgdc);
        }
    }
    // finish
    for monkey in &monkeys {
        let rc = Rc::clone(monkey);
        let borrowed = (*rc).borrow();
        println!("id #{} - {}", borrowed.id, borrowed.inspections);
    }
    println!("{}", result(monkeys));
}

fn result(monkeys: Vec<Rc<RefCell<Monkey>>>) -> u64 {
    let mut m = monkeys.clone();
    m.sort_by_key(|k| (*Rc::clone(k)).borrow().inspections);
    m.reverse();

    let v0 = (*Rc::clone(m.get(0).unwrap())).borrow().inspections;
    let v1 = (*Rc::clone(m.get(1).unwrap())).borrow().inspections;
    v0 * v1
}

#[derive(Clone)]
struct Operation {
    op: String,
    nb: Option<u64>,
}

impl Operation {
    pub fn new(op: &str, nb: &str) -> Self {
        let op = String::from(op);
        let nb = if nb == "old" { None } else { Some(nb.parse::<u64>().unwrap()) };
        Operation { op, nb }
    }

    pub fn compute(&self, v: u64) -> u64 {
        let nb = match self.nb {
            None => v,
            Some(nb) => nb,
        };
        if self.op == "+" {
            return nb + v;
        } else if self.op == "*" {
            return nb * v;
        } else if self.op == "-" {
            return nb - v;
        } else {
            panic!("jpp");
        }
    }
}

#[derive(Clone)]
struct Monkey {
    id: u64,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    true_id: u64,
    false_id: u64,
    inspections: u64,
}

impl Monkey {
    pub fn new(id: u64, items: Vec<u64>, operation: Operation, test: u64, true_id: u64, false_id: u64) -> Self {
        Monkey { id, items, operation, test, true_id, false_id, inspections: 0 }
    }

    pub fn play(&mut self, monkeys: &mut Vec<Rc<RefCell<Monkey>>>, pgdc: u64) {
        for i in 0..self.items.len() {
            let item = *self.items.get(i).unwrap();
            let new_item = self.operation.compute(item) % pgdc; // / 3;
            let new_monkey_id = if new_item % self.test == 0 { self.true_id } else { self.false_id };
            let new_monkey_rc = Rc::clone(monkeys.get_mut(new_monkey_id as usize).unwrap());
            let mut borrowed = new_monkey_rc.borrow_mut();
            borrowed.items.push(new_item);
            self.inspections += 1;
        }
        self.items = vec![];
    }
}