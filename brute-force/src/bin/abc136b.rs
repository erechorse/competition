use proconio;

fn main() {
    proconio::input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 1..n+1 {
        let digits = i.ilog10() + 1;
        if digits % 2 == 1 {
            ans = ans + 1;
        }
    }
    println!("{}", ans);
}
