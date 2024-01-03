use proconio;

fn main() {
    proconio::input! {
        s: String,
    }
    let mut count = 0;
    let mut tmp = 0;
    for c in s.chars() {
        if c == 'A' || c == 'C' || c == 'G' || c == 'T' {
            tmp = tmp + 1;
            if count < tmp {
                count = tmp;
            }
        } else {
            tmp = 0;
        }
    }
    println!("{}", count);
}
