fn main() {
    // read input
    let lines: Vec<String> = std::fs::read_to_string("input")
        .expect("Should have been able to read the file").split("\n").map(|s| String::from(s)).collect();
    // init
    let mut grid = vec![];
    for _ in 0..lines.len() {
        grid.push(vec![]);
    }
    for (i, line) in lines.iter().enumerate() {
        let grid_line = grid.get_mut(i).unwrap();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            let str = String::from(c);
            let nbr: usize = str.parse().unwrap();
            grid_line.push(nbr);
        }
    }
    let grid_size = grid.len();
    // work
    /* chall 1
    let mut visible_trees = (grid_size-1) * 4;  // forest border
    for i in 1..grid_size-1 {
        for j in 1..grid_size-1 {
            if visible(&grid, i, j) {
                visible_trees += 1;
            }
        }
    }
    println!("{visible_trees}");
    */

    // chall 2
    let mut max = 0;
    for i in 1..grid_size-1 {
        for j in 1..grid_size-1 {
            let v = scenic(&grid, i, j);
            if v > max {
                max = v;
            }
        }
    }
    println!("{max}");

}


/// please don't judge me, my brain was melting. I spent so much time here because I can't read
fn scenic(grid: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut score = vec![];
    let size = grid.len();
    let current_tree = get(grid, i, j);
    // north
    let mut tmp_vec = vec![];
    let mut range = (0..i).collect::<Vec<usize>>();
    range.reverse();
    println!("{range:?}");
    for k in range {
        let v = get(grid, k, j);
        if v < current_tree {
            tmp_vec.push(v);
            continue;
        }
        if v >= current_tree {
            tmp_vec.push(v);
            break;
        }
    }
    score.push(tmp_vec.len());

    // south
    let mut tmp_vec = vec![];
    let range = (i+1..size).collect::<Vec<usize>>();
    for k in range {
        let v = get(grid, k, j);
        if v < current_tree {
            tmp_vec.push(v);
            continue;
        }
        if v >= current_tree {
            tmp_vec.push(v);
            break;
        }
    }
    score.push(tmp_vec.len());

    // west
    let mut tmp_vec = vec![];
    let mut range = (0..j).collect::<Vec<usize>>();
    range.reverse();
    for k in range {
        let v = get(grid, i, k);
        if v < current_tree {
            tmp_vec.push(v);
            continue;
        }
        if v >= current_tree {
            tmp_vec.push(v);
            break;
        }
    }
    score.push(tmp_vec.len());

    // east
    let mut tmp_vec = vec![];
    let range = (j+1..size).collect::<Vec<usize>>();
    for k in range {
        let v = get(grid, i, k);
        if v < current_tree {
            tmp_vec.push(v);
            continue;
        }
        if v >= current_tree {
            tmp_vec.push(v);
            break;
        }
    }
    score.push(tmp_vec.len());

    let s = score.get(0).unwrap() * score.get(1).unwrap() * score.get(2).unwrap() * score.get(3).unwrap();
    println!("G({i},{j}) = {current_tree} {score:?} {s}");
    s
}

fn visible(grid: &Vec<Vec<usize>>, i: usize, j: usize) -> bool {
    let size = grid.len();
    let value = grid.get(i).unwrap().get(j).unwrap();

    let mut north = true;
    for k in 0..i {
        north &= grid.get(k).unwrap().get(j).unwrap() < value;
    }

    let mut south = true;
    for k in i+1..size {
        south &= grid.get(k).unwrap().get(j).unwrap() < value;
    }

    let mut west = true;
    for k in 0..j {
        west &= grid.get(i).unwrap().get(k).unwrap() < value;
    }

    let mut east = true;
    for k in j+1..size {
        east &= grid.get(i).unwrap().get(k).unwrap() < value;
    }
    north | south | west | east
}

fn get(grid: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    grid.get(i).unwrap().get(j).unwrap().clone()
}