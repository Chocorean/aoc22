fn main() {
    // read input
    let lines: Vec<String> = std::fs::read_to_string("input")
        .expect("Should have been able to read the file").split("\n").map(|s| String::from(s)).collect();
    // init
    let mut commands = vec![];
    for line in lines.iter() {
        commands.push(Command::new(line.clone()));
    }
    let mut register = 1_i64;
    let mut cycles = 0_i64;
    //let mut strengths = vec![];
    // work
    let mut line = String::new();
    for command in commands {
        for _ in 0..command.cycles() {
            cycles += 1;
            let cursor = cycles % 40;
            //println!("R (+/-1): {register} // Cursor: {cursor}");
            if (register-1..=register+1).contains(&(cursor -1)) {
                line.push('#');
            } else {
                line.push('.');
            }
            if cycles % 40 == 0 {
                println!("{line}");
                line = String::new();
            }
        }
        register += command.execute();
        //println!("end cycle #{cycles}; R = {register}; L = {line}");
    }
    // result
    //println!("{}", strengths.iter().map(|c| *c).reduce(|a,b| a+b).unwrap());
}

enum Command {
    Noop,
    Addx(i64),
}

impl Command {
    /// Constructor
    pub fn new(command: String) -> Command {
        if command.contains("noop") {
            Command::Noop
        } else if command.contains("addx") {
            let v_as_str = *command.split(" ").collect::<Vec<&str>>().get(1).unwrap();
            let v = v_as_str.parse::<i64>().expect("NaN");
            Command::Addx(v)
        } else {
            panic!("unknown command!");
        }
    }

    /// Return the # of cycles it takes for a command to complete
    pub fn cycles(&self) -> i64 {
        match self {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        }
    }

    pub fn execute(&self) -> i64 {
        match self {
            Command::Noop => 0,
            Command::Addx(v) => *v,
        }
    }
}