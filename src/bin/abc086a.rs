use proconio;

fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let s = a * b;
    if s % 2 == 1 {
        println!("Odd");
    } else {
        println!("Even")
    }
}
