#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}
#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct ShirtsColor(Color);
impl ShirtsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoes_color(color: ShoesColor) {
    println!("shoes color = {:?}", color);
}
fn print_shirt_color(color: ShirtsColor) {
    println!("shirts color = {:?}", color);
}
fn print_pant_color(color: PantsColor) {
    println!("pants color = {:?}", color);
}

fn main() {
    let shirt_color = ShirtsColor::new(Color::Gray);
    let pants_color = PantsColor::new(Color::Blue);
    let shoes_color = ShoesColor::new(Color::White);

    print_shirt_color(shirt_color);
    print_pant_color(pants_color);
    print_shoes_color(shoes_color);
}
