use proconio;

fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    for x in 1..(n + 1) {
        let chars: Vec<usize> = x
            .to_string()
            .chars()
            .into_iter()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect();
        let sum = chars.iter().sum();
        if a <= sum && sum <= b { ans = ans + x}
    }
    println!("{}", ans);
}
