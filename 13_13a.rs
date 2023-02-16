fn main() {
    let number = vec![10, 20, 30, 40];

    for &num in &number {
        if num == 30 {
            println!("thirty");
        } else {
            println!("{:?}", num);
        }
    }
    println!("the length : {:?}", number.len());
}
