use proconio;
use std::cmp;

fn main() {
    proconio::input! {
        a: isize,
        b: isize,
        c: isize,
        x: isize,
        y: isize,
    }
    let mut pmin = a * x + b * y;
    for i in (0..=(2 * cmp::max(x, y))).step_by(2) {
        let price = cmp::max(0, x-i/2) * a + cmp::max(0, y-i/2) * b + i * c;
        println!("{}", price);
        pmin = cmp::min(pmin, price);
    }
    println!("{}",  pmin);

}
