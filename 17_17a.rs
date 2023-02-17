struct Student {
    name: String,
    loker: Option<i32>,
}

fn main() {
    let mary = Student {
        name: "Mary".to_owned(),
        loker: Some(3),
    };

    println!("student: {:?}", mary.name);
    match mary.loker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assigned"),
    }
}
