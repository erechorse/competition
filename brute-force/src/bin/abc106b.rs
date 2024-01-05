use proconio;

fn main() {
    proconio::input! {
        n: usize,
    }
    let mut s: Vec<usize> = (1..n+1).collect();
    s.retain(|&a| a % 2 == 1);
    let mut ans = 0;
    for i in s {
        let mut count = 2;
        for j in 2..i/2+1 {
            if i % j == 0 {
                count = count + 1;
            }
        }
        if count == 8 {
            ans = ans + 1;
        }
    }
    println!("{}", ans);
}
