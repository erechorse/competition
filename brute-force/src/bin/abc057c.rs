use proconio;

fn main() {
    proconio::input! {
        n: usize,
    }
    let mut ans = n.ilog10() + 1;
    let s = (n as f64).sqrt().floor() as usize;
    for a in 2..=s {
        if n % a != 0 {
            continue
        } else {
            let b = n / a;
            let f = if a.ilog10()+1 < b.ilog10()+1 { b.ilog10()+1 } else { a.ilog10()+1 };
            if f < ans { 
                ans = f;
            }
        }
    }
    println!("{}", ans);
}
