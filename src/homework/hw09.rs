pub fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    // Зсув вправо: обертаємо n праворуч, з урахуванням негативних значень
    let n = ((n % len as isize + len as isize) % len as isize) as usize;

    let chars: Vec<char> = s.chars().collect();
    let rotated: Vec<char> = chars[len - n..]
        .iter()
        .chain(&chars[..len - n])
        .cloned()
        .collect();

    rotated.into_iter().collect()
}
