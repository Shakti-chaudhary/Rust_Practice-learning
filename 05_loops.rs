fn main() {
    let mut i = 0;
    loop {
        println!("hello world!");
        i += 1;
        if i >= 5 {
            break;
        }
    }
    println!("done! looping ");
}
