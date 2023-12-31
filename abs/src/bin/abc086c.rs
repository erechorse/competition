use proconio;

fn main() {
    proconio::input! {
        n: usize,
        m: [[usize; 3]; n],
    }
    let mut flag = true;
    let mut x = 0;
    let mut y = 0;
    let mut t = 0;
    for p in m { 
        let need_step = (((p[1] as isize - x as isize)).abs() + ((p[2] as isize - y as isize)).abs()) as usize;
        let actual_step = p[0] - t;
        if need_step == actual_step {
            (t, x, y) = (p[0], p[1], p[2]);
        } else if actual_step < need_step {
            flag = false;
            break;
        } else if ((actual_step - need_step) % 2) == 1 {
            flag = false;
            break;
        } else {
            (t, x, y) = (p[0], p[1], p[2]);
        }
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
