const WIDTH: usize = 15;
const HEIGHT: usize = 15;

fn draw_envelope() {
    let mut result = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let is_border = x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1;
            let is_diagonal1 = x == y;
            let is_diagonal2 = x + y == WIDTH - 1;
            
            if is_border || is_diagonal1 || is_diagonal2 {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
}

fn main() {
    draw_envelope();
}
 