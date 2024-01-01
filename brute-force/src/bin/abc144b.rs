use proconio;

fn main() {
    proconio::input! {
        n: usize,
    }
    let mut flag = false;
    for i in 1..10 {
        for j in i..10 {
            if n == i * j { 
                flag = true;
                break;
            }
        }
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
