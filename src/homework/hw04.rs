// hw04.rs

const HEIGHT: usize = 6; 
const CHAR: char = '*';

fn main() {
    let mut result = String::new();

    
    for i in 0..HEIGHT {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;

        result.push_str(&" ".repeat(spaces));
        result.push_str(&CHAR.to_string().repeat(stars));
        result.push('\n');
    }

    
    for i in (0..HEIGHT - 1).rev() {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;

        result.push_str(&" ".repeat(spaces));
        result.push_str(&CHAR.to_string().repeat(stars));
        result.push('\n');
    }

    print!("{}", result);
}
