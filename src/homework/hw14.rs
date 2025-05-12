pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::from("")];
    }

    let prev = gray(n - 1);

    let mut result = Vec::new();

    for code in &prev {
        result.push(format!("0{}", code));
    }

    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}
