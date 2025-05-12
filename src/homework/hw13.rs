use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Debug)]
struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rectangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Rectangle {
            x1: x1.min(x2),
            y1: y1.min(y2),
            x2: x1.max(x2),
            y2: y1.max(y2),
        }
    }

    fn covered_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        for x in self.x1..self.x2 {
            for y in self.y1..self.y2 {
                points.insert(Point(x, y));
            }
        }
        points
    }
}

fn total_occupied_area(rectangles: Vec<Rectangle>) -> usize {
    let mut occupied = HashSet::new();
    for rect in rectangles {
        occupied.extend(rect.covered_points());
    }
    occupied.len()
}

fn main() {
    let red = Rectangle::new(2, 3, 5, 9);
    let green = Rectangle::new(3, 5, 9, 7);
    let blue = Rectangle::new(7, 2, 10, 9);

    let area = total_occupied_area(vec![red, green, blue]);
    println!("Total occupied area: {}", area);
}
