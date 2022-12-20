use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use std::ops::Bound::{Excluded, Included};
use unbounded_interval_tree::interval_tree::IntervalTree;

fn main() {
    // init
    let content = include_str!("../input");
    let sensor_regex = Regex::new(r"at x=(-?\d+), y=(-?\d+):.*at x=(-?\d+), y=(-?\d+)").unwrap();
    let mut sensors = vec![];
    for cap in sensor_regex.captures_iter(content) {
        sensors.push(Sensor::new(
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
            cap[3].parse::<i64>().unwrap(),
            cap[4].parse::<i64>().unwrap(),
        ));
    }
    /* PART 1
    let beacons: Vec<Coord> = sensors.iter().map(|s| s.beacon.clone()).collect();
    let target_layer = 2000000;
    let mut count = 0;
    let mut counted = HashSet::<Coord>::new();
    // explore all sensors
    for sensor in sensors {
        let (min_y, max_y) = sensor.vertical_range();
        // if sensor detects stuff around y = 2000000
        if target_layer >= min_y || target_layer <= max_y {
            // inspect all cells in its horizontal range
            let (min_x, max_x) = sensor.horizontal_range();
            for x in min_x..=max_x {
                let coord = Coord::new(x, target_layer);
                // if distance with this coord is within sensor range
                if sensor.coord.distance(&coord) <= sensor.distance()
                    // and not already explored
                    && !counted.contains(&coord)
                    // and not a beacon
                    && !beacons.contains(&coord) {
                    count += 1;
                    counted.insert(coord);
                }
            }
        }
    }
    println!("{count}");
    */
    // PART 2
    let (min_layer, max_layer) = (0, 4000000);
    for layer in min_layer..=max_layer {
        // filling counted with all possible coords
        let mut signals = Signal::new();
        let first_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        for sensor in &sensors {
            if sensor.coord.x == 2 && sensor.coord.y == 18 {}
            let (min_y, max_y) = sensor.vertical_range();
            // if sensor detects stuff on this layer
            if layer <= max_y && min_y <= layer {
                // inspect cells around it on this layer
                // finding the relevant x value at this layer
                let offset = sensor.distance() - (sensor.coord.y - layer).abs();
                let (min_x, max_x) = (sensor.coord.x - offset, sensor.coord.x + offset);
                // cells between [min_x; max_x] at y=layer are not candidates
                // for all cells within [0;4000000] (optimized a little)
                signals.push((min_x, max_x));
            }
        }
        // check unique number of Signal cells. if it is exactly 1, jackpot!
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let available = signals.available_between(min_layer, max_layer);
        let new_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if layer % 250000 == 0 {
            println!(
                "#{layer} {:?}, {} ({}) in {} secs",
                available,
                available.len(),
                (new_time - time).as_millis(),
                (new_time - first_time).as_millis()
            );
        }
        if available.len() == 1 {
            println!("{}", layer + 4000000 * available.first().unwrap());
            break;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }

    pub fn distance(&self, other: &Coord) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    coord: Coord,
    beacon: Coord,
}

impl Sensor {
    pub fn new(x: i64, y: i64, x_b: i64, y_b: i64) -> Sensor {
        let coord = Coord::new(x, y);
        let beacon = Coord::new(x_b, y_b);
        Sensor { coord, beacon }
    }

    pub fn distance(&self) -> i64 {
        self.coord.distance(&self.beacon)
    }

    pub fn vertical_range(&self) -> (i64, i64) {
        let distance = self.distance();
        (self.coord.y - distance, self.coord.y + distance)
    }

    pub fn horizontal_range(&self) -> (i64, i64) {
        let distance = self.distance();
        (self.coord.x - distance, self.coord.x + distance)
    }
}

struct Signal {
    signals: Vec<(i64, i64)>,
}

impl Signal {
    pub fn new() -> Signal {
        Signal {
            signals: Vec::new(),
        }
    }

    pub fn available_between(&self, min: i64, max: i64) -> Vec<i64> {
        // filling tree
        let mut tree = IntervalTree::default();
        for (s_min, s_max) in &self.signals {
            tree.insert(*s_min..=*s_max);
        }
        let range = min..=max;
        let diff = tree.get_interval_difference(&range);
        let mut candidates = vec![];
        // there may be a better way but I'm tired
        for (x_min, x_max) in diff {
            match (x_min, x_max) {
                (Included(min), Included(max)) => {
                    for i in *min..=*max {
                        candidates.push(i);
                    }
                }
                (Excluded(min), Excluded(max)) => {
                    for i in *min + 1..*max {
                        candidates.push(i);
                    }
                }
                (Included(min), Excluded(max)) => {
                    for i in *min..*max {
                        candidates.push(i);
                    }
                }
                (Excluded(min), Included(max)) => {
                    for i in *min + 1..=*max {
                        candidates.push(i);
                    }
                }
                _ => {
                    panic!("jpp");
                }
            }
        }
        candidates
    }

    pub fn push(&mut self, range: (i64, i64)) {
        self.signals.push(range);
    }
}
