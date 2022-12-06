use std::fs;

fn main() {
    // read input
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    // init
    let mut total = 0;
    let couples: Vec<&str> = contents.split("\n").collect();
    for couple in couples {
        let assignments: Vec<&str> = couple.split(",").collect();
        let assignment_a = assignments[0].split("-").collect::<Vec<&str>>();
        let assignment_b = assignments[1].split("-").collect::<Vec<&str>>();
        let a_inf: u8 = assignment_a[0].parse().unwrap();
        let a_sup: u8 = assignment_a[1].parse().unwrap();
        let b_inf: u8 = assignment_b[0].parse().unwrap();
        let b_sup: u8 = assignment_b[1].parse().unwrap();
        if a_sup >= b_inf && a_inf <= b_sup || b_sup >= a_inf && b_inf <= a_sup {
            println!("{couple}");
            total += 1;
        }
    }
    println!("total: {total}");
}