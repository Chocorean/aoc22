fn main() {
    // read input
    let content: Vec<String> = std::fs::read_to_string("input")
        .expect("Should have been able to read the file").split('\n').map(String::from).collect();
    // init
    let mut grid = Grid::new();
    for line in content {
        let coords: Vec<&str> = line.split(" -> ").collect();
        for i in 0..coords.iter().len()-1 {
            let current_pair: Vec<&str> = coords.get(i).unwrap().split(',').collect();
            let next_pair: Vec<&str> = coords.get(i+1).unwrap().split(',').collect();
            // coords
            let x_a = current_pair.first().unwrap().parse::<u32>().unwrap();
            let y_a = current_pair.get(1).unwrap().parse::<u32>().unwrap();
            let coord_a = Coord::new(x_a, y_a);
            let x_b = next_pair.first().unwrap().parse::<u32>().unwrap();
            let y_b = next_pair.get(1).unwrap().parse::<u32>().unwrap();
            let coord_b = Coord::new(x_b, y_b);
            // path
            let current_path = Path::new(coord_a, coord_b);
            grid.push(current_path);
        }
    }
    grid.show();
    let sand_gen_coord = Coord::new(500, 0);
    // work
    let mut reached_top = false;
    while ! reached_top {
        grid.sands.push(sand_gen_coord.clone());
        for index in 0..grid.sands.len() {
            let sand = grid.sands.get(index).unwrap();
            let new_sand = grid.fall(*sand);
            if new_sand != *sand { // update grid
                grid.sands.remove(index);
                grid.sands.insert(index, new_sand);
            } else { // cannot move. check if at top
                if sand.y == sand_gen_coord.y {
                    reached_top = true;
                    break
                }
            }
        }
        let len = grid.sands.len();
        if len % 100 == 0 {
            println!("{len}");
        }
        let len = grid.sands.len();
        if len % 10 == 0 {
            println!("{len}");
        }
    }

    println!("{}", grid.sands.len());
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Coord {
        Coord { x, y }
    }
}

struct Path {
    a: Coord,
    b: Coord
}

impl Path {
    pub fn new(a: Coord, b: Coord) -> Path {
        Path { a, b }
    }

    /// compute all coords of a path
    pub fn coords(&self) -> Vec<Coord> {
        let dx = (self.a.x as i32 - self.b.x as i32).abs();
        let dy = (self.a.y as i32 - self.b.y as i32).abs();
        let count = (dx+1).max(dy+1) as u32;
        let mut points = vec![];
        if dx == 0 {
            let min_y = self.a.y.min(self.b.y);
            for i in 0..count {
                points.push(Coord::new(self.a.x, min_y + i));
            }
        } else if dy == 0 {
            let min_x = self.a.x.min(self.b.x);
            for i in 0..count {
                points.push(Coord::new(min_x + i, self.a.y));
            }
        } else {
            panic!("not a straight line");
        }
        points
    }
}

struct Grid {
    paths: Vec<Path>,
    coords: Vec<Coord>,  // try to optimize this slow code
    sands: Vec<Coord>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid { paths: vec![], coords: vec![], sands: vec![] }
    }

    /// add a path
    pub fn push(&mut self, p: Path) {
        for c in p.coords() {
            self.coords.push(c);
        }
        self.paths.push(p);
    }


    /// makes one sand fall
    /// return true is it fell
    /// not sure if this work
    pub fn fall(&self, sand: Coord) -> Coord {
        // pre-check if touching the ground
        let (_, _, _, max_y) = self.borders();
        if sand.y == max_y {
            return sand;
        }

        let bottom = Coord::new(sand.x, sand.y+1);
        let bottom_left = Coord::new(sand.x-1, sand.y+1);
        let bottom_right = Coord::new(sand.x+1, sand.y+1);
        for coord in [bottom, bottom_left, bottom_right] {
            if self.available(coord) {
                return coord;
            }
        }
        sand
    }

    /// print the grid
    pub fn show(&self) {
        // init
        let(min_x, min_y, max_x, max_y) = self.borders();
        let mut screen = vec![];
        for _ in min_y..=max_y {
            screen.push((min_x..=max_x).map(|_| ".").collect::<String>());
        }
        let first_line = screen.get_mut(0).unwrap();
        let sand_x = (500 - min_x) as usize;

        // add sand source
        first_line.replace_range(sand_x..=sand_x, "+");
        drop(first_line);
        // add paths
        for coord in &self.coords {
            let str_x = (coord.x - min_x) as usize;
            let str_y = (coord.y - min_y) as usize;
            let current_line = screen.get_mut(str_y).unwrap();
            current_line.replace_range(str_x..=str_x, "#");
        }

        // add sand
        for sand in &self.sands {
            let (x, y) = ((sand.x - min_x) as usize, sand.y as usize);
            let current_line = screen.get_mut(y as usize).unwrap();
            current_line.replace_range(x..=x, "o");
        }

        for line in screen {
            println!("{line}");
        }
        // finally, print floor
        println!("{}", (min_x..=max_x).map(|_| "#").collect::<String>());

    }

    /// compute the borders of the grid, with an extra bottom depth and an extra
    /// width on both sides
    fn borders(&self) -> (u32, u32, u32, u32) {
        let coords = &self.coords;
        let min_x = coords.iter().map(|c| c.x).reduce(u32::min).unwrap() - 1;
        let min_y = 0;
        let max_x = coords.iter().map(|c| c.x).reduce(u32::max).unwrap() + 1;
        let max_y = coords.iter().map(|c| c.y).reduce(u32::max).unwrap() + 1;
        (min_x, min_y, max_x, max_y)
    }

    /// say if a given cell is available
    pub fn available(&self, coord: Coord) -> bool {
        if self.sands.contains(&coord) {
            return false;
        }
        let coords = &self.coords;
        if coords.contains(&coord) {
            return false;
        }
        true
    }
}