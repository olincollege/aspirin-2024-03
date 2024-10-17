use std::ops::Add;

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    // add p1 and p2 using the MyAdd trait
    dbg!(<Point as MyAdd<Point, Point>>::my_add(&p1, &p2));
    dbg!(<Point as MyAdd<Point, i32>>::my_add(&p1, &p2));

    // add p1 and p2 using associated types
    dbg!(p1 + p2);
    dbg!(p1 + 1);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

trait MyAdd<In, Out> {
    fn my_add(&self, other: &In) -> Out;
}

impl MyAdd<Point, Point> for Point {
    fn my_add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl MyAdd<Point, i32> for Point {
    fn my_add(&self, other: &Point) -> i32 {
        self.x + other.x + self.y + other.y
    }
}

impl Add<Point> for Point {
    // I can't do this again!
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, other: i32) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}
