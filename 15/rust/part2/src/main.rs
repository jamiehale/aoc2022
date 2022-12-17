use lazy_static::lazy_static;
use regex::Regex;
use std::{
    io::{self, BufRead},
    process::exit,
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
    const TEST_RANGE: i64 = 4_000_000;
    for sensor in &sensors {
        println!("Sensor at {:?}", sensor.position);
        for i in 0..sensor.range {
            let points = vec![
                Position::new(
                    sensor.position.x - (sensor.range as i64 + 1 - i as i64),
                    sensor.position.y + i as i64,
                ),
                Position::new(
                    sensor.position.x + i as i64,
                    sensor.position.y + (sensor.range as i64 + 1 - i as i64),
                ),
                Position::new(
                    sensor.position.x + (sensor.range as i64 + 1 - i as i64),
                    sensor.position.y - i as i64,
                ),
                Position::new(
                    sensor.position.x - i as i64,
                    sensor.position.y - (sensor.range as i64 + 1 - i as i64),
                ),
            ];
            for point in &points {
                if point.x < 0 || point.x > TEST_RANGE {
                    continue;
                }
                if point.y < 0 || point.y > TEST_RANGE {
                    continue;
                }
                let mut solution = true;
                for test_sensor in &sensors {
                    if test_sensor.can_see(point) {
                        solution = false;
                        break;
                    }
                }
                if solution {
                    println!("{:?} {}", point, point.x * TEST_RANGE + point.y);
                    exit(0);
                }
            }
        }
    }
}
