use std::collections::HashMap;

use utils;

pub fn run() {
    for line in utils::read_input(3) {
        let number = line.parse::<u32>().unwrap();
        let point = compute_spiral_point_up_to(number);
        let distance = point.x.abs() + point.y.abs();
        println!("{}", distance);

        let first_value_greater_than_number = compute_first_value_greater_than_number(number);
        println!("{}", first_value_greater_than_number);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn to_string(&self) -> String {
        return [self.x.to_string(), self.y.to_string()].join(",");
    }
}

struct Bounds {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Bounds {
    fn left_blocked(&self, point: &Point) -> bool {
        self.left == point.x
    }

    fn right_blocked(&self, point: &Point) -> bool {
        self.right == point.x
    }

    fn top_blocked(&self, point: &Point) -> bool {
        self.top == point.y
    }

    fn bottom_blocked(&self, point: &Point) -> bool {
        self.bottom == point.y
    }

    fn expand(&mut self) {
        self.left -= 1;
        self.right += 1;
        self.top -= 1;
        self.bottom += 1;
    }
}

fn compute_spiral_point_up_to(number: u32) -> Point {
    let mut point = Point { x: 0, y: 0 };
    let mut bounds = Bounds {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };

    for _ in 2..number + 1 {
        iterate_spiral(&mut point, &mut bounds);
    }

    return point;
}

fn compute_first_value_greater_than_number(number: u32) -> u32 {
    let mut point = Point { x: 0, y: 0 };
    let mut bounds = Bounds {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };

    let mut spiral = HashMap::new();

    for _ in 2..number + 1 {
        let mut value = sum_adjacents(&point, &spiral);
        if value == 0 {
            value = 1;
        }
        if value > number {
            return value;
        }
        spiral.insert(point.to_string(), value);

        iterate_spiral(&mut point, &mut bounds);
    }

    return 0;
}

fn iterate_spiral(point: &mut Point, bounds: &mut Bounds) {
    if bounds.bottom_blocked(&point) && bounds.right_blocked(&point) {
        bounds.expand();
    }
    if bounds.bottom_blocked(&point) {
        point.x += 1;
    } else if bounds.left_blocked(&point) {
        point.y += 1;
    } else if bounds.top_blocked(&point) {
        point.x -= 1;
    } else if bounds.right_blocked(&point) {
        point.y -= 1;
    } else {
        point.x += 1;
    }
}

fn sum_adjacents(point: &Point, spiral: &HashMap<String, u32>) -> u32 {
    let adjacent_points: [Point; 9] = [
        Point { x: point.x - 1, y: point.y - 1},
        Point { x: point.x - 1, y: point.y },
        Point { x: point.x - 1, y: point.y + 1},
        Point { x: point.x, y: point.y - 1},
        Point { x: point.x, y: point.y },
        Point { x: point.x, y: point.y + 1},
        Point { x: point.x + 1, y: point.y - 1},
        Point { x: point.x + 1, y: point.y },
        Point { x: point.x + 1, y: point.y + 1},
    ];

    let mut sum : u32 = 0;
    for point in adjacent_points.iter() {
        match spiral.get(&point.to_string()) {
            Some(value) => sum += value,
            None => {},
        }
    }

    return sum;
}
