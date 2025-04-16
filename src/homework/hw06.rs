pub fn draw_tree(triangles: usize) {
    let total_height: usize = (1..=triangles).sum();
    let max_width = total_height * 2 - 1;

    for t in 1..=triangles {
        for i in 0..t {
            let stars = i * 2 + 1;
            let spaces = (max_width - stars) / 2;
            println!(
                "{}{}",
                " ".repeat(spaces),
                "*".repeat(stars)
            );
        }
    }
}

fn main() {
    let triangle_count = 6; 
    draw_tree(triangle_count);
}
