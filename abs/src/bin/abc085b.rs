use proconio;

fn main() {
    proconio::input! {
        n: usize,
        d: [usize;n],
    }
    let mut vec = d;
    vec.sort();
    vec.dedup();
    println!("{}", vec.len());
}
