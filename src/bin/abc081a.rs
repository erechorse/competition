use proconio;

fn main() {
    proconio::input! {
        s: String
    }
    let mut ans = 0;
    for c in s.chars() {
        let num = c as i32 - 48;
        ans = ans + num;
    }
    println!("{}", ans);
}
