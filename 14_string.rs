struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name is {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];
    for item in &receipt {
        println!("name : {:?}, count : {:?}", item.name, item.count);
    }
    print_name(&receipt[1].name);
}
