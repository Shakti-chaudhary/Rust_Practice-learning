enum Color {
    White,
    Red,
    Blue,
    Pink,
    Yellow,
    Green,
}
fn print_color(my_color: Color) {
    match my_color {
        Color::White => println!("White color"),
        Color::Red => println!("Red color"),
        Color::Blue => println!("Blue color"),
        Color::Pink => println!("Pink color"),
        Color::Yellow => println!("Yellow color"),
        Color::Green => println!("Green color"),
    }
}
fn main() {
    let color = Color::Red;
    print_color(color);
}
