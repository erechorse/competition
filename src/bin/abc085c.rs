use proconio;

fn main() {
    proconio::input! {
        n: i32,
        y: i32,
    }
    let mut ans = (-1, -1, -1);
    for i in 0..n + 1 {
        for j in 0..n + 1 - i {
            let sum = 10000 * i + 5000 * j + 1000 * (n - i - j);
            if y == sum {
                ans = (i, j, n - i - j);
                break;
            }
        }
    }

    println!("{} {} {}", ans.0, ans.1, ans.2);
}
