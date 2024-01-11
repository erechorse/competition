use proconio;

fn main() {
    proconio::input! {
        _: usize,
        s: String,
    }
    let mut counter = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                if let Some(index) = s.find(&i.to_string()) {
                    let pass = &s[index+1..];
                    if let Some(index) = pass.find(&j.to_string()) {
                        let pass = &pass[index+1..];
                        if pass.find(&k.to_string()).is_some() {
                            counter += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", counter);
}
