enum BoxColor {
    Red,
    Brown,
}

impl BoxColor {
    fn display(&self) {
        match self {
            BoxColor::Brown => println!("brown"),
            BoxColor::Red => println!("red"),
        }
    }
}

struct Dimension {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimension {
    fn create_dimension(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
    fn display(&self) {
        println!(
            "Dimension are width: {:?},height: {:?}, depth: {:?}",
            self.width, self.height, self.depth
        );
    }
}

struct ShippingBox {
    dimension: Dimension,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn create_box(dimension: Dimension, weight: f64, color: BoxColor) -> Self {
        Self {
            dimension,
            weight,
            color,
        }
    }

    fn display(&self) {
        self.color.display();
        self.dimension.display();
        println!("weight are  {:?}", self.weight);
    }
}

fn main() {
    let box1 = ShippingBox::create_box(
        Dimension::create_dimension(22.3, 45.0, 3.54),
        45.0,
        BoxColor::Red,
    );
    box1.display();
}
