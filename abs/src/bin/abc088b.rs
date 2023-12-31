use proconio;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize;n],
    }
    let mut alice = 0;
    let mut bob = 0;
    let mut vec = a;

    for i in 0..n {
        let (index, max) = vec.iter()
            .enumerate()
            .fold((usize::MIN, usize::MIN), |(i_a, a), (i_b, &b)| {
                if b > a {
                    (i_b, b)
                } else {
                    (i_a, a)
                }
            });
        vec.remove(index);
        if i % 2 == 0 {
            alice = alice + max;
        } else {
            bob = bob + max;
        }
    }
    println!("{}", alice - bob);
}
