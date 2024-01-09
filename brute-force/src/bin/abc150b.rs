use proconio;

fn main() {
    proconio::input! {
        _: usize,
        i: String,
    }
    println!("{}", i.match_indices("ABC").count());
}
