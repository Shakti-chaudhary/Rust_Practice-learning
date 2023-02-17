fn to_uppercase(str: &str) {
    println!("{:?}", str.to_uppercase());
}
fn to_lowercase(str: &str) {
    println!("{:?}", str.to_lowercase());
}
fn main() {
    let name = "Rohan Singh";
    to_lowercase(&name);
    to_uppercase(&name);
}
