use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_sum)
}

fn print_data(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:    [");
    for (i, val) in data.iter().enumerate() {
        if i != data.len() - 1 {
            print!("{:>2}, ", val);
        } else {
            print!("{:>2}", val);
        }
    }
    println!("]");

    let (min_idx, min_sum) = min_adjacent_sum(data);

    print!("indexes: ");
    for i in 0..data.len() {
        if i == min_idx {
            print!(" \\__");
        } else if i == min_idx + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_idx],
        data[min_idx + 1],
        min_sum,
        min_idx,
        min_idx + 1
    );
}

fn main() {
    for _ in 0..4 {
        let data = gen_random_vector(20);
        print_data(&data);
        println!();
    }
}
