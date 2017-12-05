use utils;

pub fn run() {
    for line in utils::read_input(3) {
        let number = line.parse::<u32>().unwrap();
        let point = get_spiral_coords(number);
        let distance = point.x.abs() + point.y.abs();
        println!("{}", distance);
    }
}

struct Point {
    x: i32,
    y: i32,
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

fn get_spiral_coords(number: u32) -> Point {
    let mut point = Point { x: 0, y: 0 };
    let mut bounds = Bounds {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    for _ in 2..number + 1 {
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

    return point;
}
