enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Sparkling"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Fruity => println!("Fruity"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let sparkling = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 2.2,
    };
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.2,
    };
    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 3.9,
    };
    print_drink(sparkling);
    print_drink(fruity);
    print_drink(sweet);
}
