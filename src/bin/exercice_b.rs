struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance_from(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Game {
    circles: Vec<Circle>,
}

impl Game {
    fn new() -> Game {
        Game {
            circles: Vec::new()
        }
    }
    
    fn add_circle(&mut self, x: f64, y: f64) {
        let c = Circle {
            x, 
            y, 
            radius : 50.0
        };

        self.circles.push(c);
    }
    
    fn last_circle(&self) -> Option<&Circle> {
        self.circles.last()
    }
    
    fn is_click_on_last(&self, click_x: f64, click_y: f64) -> bool {
        if let Some(last_circle) = self.last_circle() {
            let p1 = Point::new(click_x, click_y);
            let p2 = Point::new(last_circle.x, last_circle.y);

            p1.distance_from(&p2) < last_circle.radius
        } else {
            false
        }
    }
}

fn main() {
    println!("=== Test Étape 1.1 - Point ===");
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("Distance: {}", p1.distance_from(&p2));
    
    println!("\n=== Test Étape 1.2 - Game ===");
    let mut game = Game::new();
    println!("Cercles au début: {}", game.circles.len());
    
    game.add_circle(100.0, 100.0);
    game.add_circle(200.0, 200.0);
    println!("Cercles après ajouts: {}", game.circles.len());
    
    // Test de last_circle
    match game.last_circle() {
        Some(c) => println!("Dernier cercle à: ({}, {})", c.x, c.y),
        None => println!("Pas de cercle"),
    }
    
    // Test de is_click_on_last
    println!("Clic sur (200, 200): {}", game.is_click_on_last(200.0, 200.0));
    println!("Clic sur (500, 500): {}", game.is_click_on_last(500.0, 500.0));
}