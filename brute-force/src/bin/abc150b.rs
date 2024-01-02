use proconio;

fn main() {
    proconio::input! {
        n: usize,
        i: String,
    }
    println!("{}", i.match_indices("ABC").count());
}
