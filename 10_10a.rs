fn print_msg(gt_100: bool) {
    match gt_100 {
        true => println!("true"),
        false => println!("false"),
    }
}

fn main() {
    let value = 122;
    let is_gt_100 = value > 100;
    print_msg(is_gt_100);
}
