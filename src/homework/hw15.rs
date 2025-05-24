fn check_solution(m: u32, u: u32, x: u32, a: u32, s: u32, l: u32, o: u32, n: u32) -> bool {
    let muxa = m * 1000 + u * 100 + x * 10 + a;
    let xa = x * 10 + a;
    let slon = s * 1000 + l * 100 + o * 10 + n;
    muxa + xa == slon
}

pub fn solve() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut used = [false; 9];
    let mut assignment = [0; 8];
    fn backtrack(pos: usize, used: &mut [bool], assignment: &mut [u32], count: &mut u32) {
        if pos == 8 {
            let (m, u, x, a, s, l, o, n) = (
                assignment[0], assignment[1], assignment[2], assignment[3],
                assignment[4], assignment[5], assignment[6], assignment[7],
            );
            if check_solution(m, u, x, a, s, l, o, n) {
                *count += 1;
                println!("  {}{}{}{}", m, u, x, a);
                println!("{}        {}", x, a);
                println!("  ------");
                println!("    {}{}{}{}", s, l, o, n);
                println!();
            }
            return;
        }

        for digit in 1..=8 {
            if !used[digit as usize] {
                used[digit as usize] = true;
                assignment[pos] = digit;
                backtrack(pos + 1, used, assignment, count);
                used[digit as usize] = false;
            }
        }
    }

    let mut count = 0;
    backtrack(0, &mut used, &mut assignment, &mut count);
    println!("Загальна кількість розв'язків: {}", count);
}

#[test]
fn test_solve() {
    solve();
}
