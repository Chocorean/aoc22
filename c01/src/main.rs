use std::fs;

struct Elf {
    snacks: Vec<u32>,
}

impl Elf {
    pub fn total(&self) -> u32 {
        self.snacks.iter().copied().reduce(|x, y| x+y ).unwrap()
    }
}

fn main() {
    // read input
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    // build elves
    let mut elves = vec![];
    let chunks: Vec<&str> = contents.split("\n\n").collect();
    println!("elves #: {}", chunks.len());
    for chunk in chunks {
        let lines: Vec<&str> = chunk.split("\n").collect();
        let mut snacks = vec![];
        for line in lines {
            let snack = line.parse().expect("NaN");
            snacks.push(snack);
        }
        //let snacks: Vec<u32> = lines.iter().map(|x| x.parse::<u32>().expect("NaN")).collect();
        elves.push(Elf { snacks });
    }
    // Doing stuff
    let mut max_cal = vec![0, 0, 0];
    for elf in elves {
        let total = elf.total();

        max_cal.push(total);
        max_cal.sort();
        max_cal.remove(0);
    }
    println!("total cal: {}", max_cal.iter().copied().reduce(|a, b| a+b).unwrap());
}