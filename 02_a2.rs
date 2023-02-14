fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let sum_result = sum(8, 5);
    display_result(sum_result)
}
