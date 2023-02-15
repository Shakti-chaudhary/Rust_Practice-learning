fn coordinate() -> (i32, i32) {
    (7, 9)
}

fn main() {
    let (y_value, x_value) = coordinate();

    if y_value < 5 {
        println!("y_value is less than 5");
    } else if y_value == 5 {
        println!("y_value is equal to  5");
    } else {
        println!("y_value is greater than 5");
    }
}
