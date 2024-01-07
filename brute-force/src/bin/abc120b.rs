use proconio;

fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        k: usize,
    }
    let small = if a < b { a } else { b };
    let mut count = 0;
    let mut ans = 0;
    for i in (1..=small).rev() {
        if a % i == 0 && b % i == 0 {
            count = count + 1;
            if count == k {
                ans = i;
                break;
            }
        }
    }
    println!("{}", ans);
}
