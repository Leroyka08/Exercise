use rand::Rng;


pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return -1; 
    }

    let average = (total / n as u32) as i32;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments.iter() {
        balance += weight as i32 - average;
        moves += balance.abs();
    }

    moves as isize
}


pub fn count_permutation_safe(shipments: &Vec<u32>) -> Result<usize, &'static str> {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return Err("Impossible to balance shipments equally");
    }

    let average = (total / n as u32) as i32;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments.iter() {
        balance += weight as i32 - average;
        moves += balance.abs();
    }

    Ok(moves as usize)
}


pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(1..=1000);
    let mut shipments: Vec<u32> = (0..n).map(|_| avg).collect();

    for i in 0..(n / 2) {
        let delta = rng.gen_range(1..=avg.min(100));
        shipments[i] += delta;
        shipments[n - i - 1] -= delta;
    }

    shipments
}

fn main() {
    let example1 = vec![1, 1, 1, 1, 6];
    println!("Example 1: {:?}", example1);
    println!("Moves: {}\n", count_permutation(&example1)); 

    let example2 = vec![9, 3, 7, 2, 9];
    println!("Example 2: {:?}", example2);
    println!("Moves: {}\n", count_permutation(&example2)); 
    let impossible = vec![1, 2, 3];
    println!("Impossible case: {:?}", impossible);
    println!("Moves: {}\n", count_permutation(&impossible)); 

    let generated = gen_shipments(10);
    println!("Generated: {:?}", generated);
    println!("Moves: {}\n", count_permutation(&generated)); 

}
