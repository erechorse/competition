use proconio;

fn main() {
    proconio::input! {
        a: i32
    }
    proconio::input! {
        b: i32,
        c: i32,
    }
    proconio::input! {
        s: String
    }
    let sum = a + b + c;
    println!("{} {}", sum, s);
}
