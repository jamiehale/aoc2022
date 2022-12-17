use lazy_static::lazy_static;
use regex::Regex;
use std::{
    io::{self, BufRead},
    vec,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, p: &Position) -> u64 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}

struct Sensor {
    position: Position,
    beacon: Position,
    range: u64,
}

impl Sensor {
    fn new(position: Position, beacon: Position) -> Self {
        Sensor {
            position,
            beacon,
            range: position.distance_to(&beacon),
        }
    }

    fn can_see(&self, p: &Position) -> bool {
        self.position.distance_to(p) <= self.range
    }
}

fn to_positions(line: String) -> (Position, Position) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }
    let ns: Vec<i64> = RE
        .find_iter(line.as_str())
        .map(|s| s.as_str().parse::<i64>().unwrap())
        .collect();
    (Position::new(ns[0], ns[1]), Position::new(ns[2], ns[3]))
}

fn main() {
    let facts: Vec<(Position, Position)> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_positions)
        .collect();
    let mut sensors: Vec<Sensor> = vec![];
    let mut beacons: Vec<Position> = vec![];
    for (sensor_location, beacon_location) in &facts {
        let sensor = Sensor::new(*sensor_location, *beacon_location);
        println!(
            "Sensor at {:?} range={} beacon at {:?}",
            sensor.position, sensor.range, sensor.beacon
        );
        sensors.push(sensor);
        if !beacons.contains(&beacon_location) {
            beacons.push(*beacon_location);
        }
    }
    let min_x = *sensors
        .iter()
        .map(|sensor| sensor.position.x - sensor.range as i64)
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap();
    let max_x = *sensors
        .iter()
        .map(|sensor| sensor.position.x + sensor.range as i64)
        .collect::<Vec<i64>>()
        .iter()
        .max()
        .unwrap();
    println!("{}..{}", min_x, max_x);
    let mut count = 0;
    for x in min_x..max_x {
        let p = Position::new(x, 2000000);
        // print!("{:?}", p);
        if beacons.contains(&p) {
            // println!(" is a beacon");
            continue;
        }
        for sensor in &sensors {
            if sensor.can_see(&p) {
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}
