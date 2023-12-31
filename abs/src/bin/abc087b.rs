use proconio;

fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut ans = 0;
    for i in 0..(a + 1) {
        for j in 0..(b + 1) {
            for k in 0..(c + 1) {
                let codic = 500 * i + 100 * j + 50 * k;
                if codic == x {
                    ans = ans + 1;
                }
            }
        }
    }
    println!("{}", ans);
}
