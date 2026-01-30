struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {x , y}
    }

    fn distance_from(&self, other: &Point) -> f64 {
        let dx = &self.x - other.x;
        let dy = &self.y - other.y;
 
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);

    println!("Distance {}", p1.distance_from(&p2));
}