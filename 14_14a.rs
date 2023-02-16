struct Person {
    name: String,
    age: i32,
    color: String,
}

fn main() {
    let persons = vec![
        Person {
            name: String::from("charly"),
            age: 5,
            color: String::from("red"),
        },
        Person {
            name: String::from("anshul"),
            age: 15,
            color: String::from("brown"),
        },
        Person {
            name: String::from("rohan"),
            age: 5,
            color: String::from("green"),
        },
    ];
    for person in &persons {
        if person.age <= 10 {
            println!(" name : {:?}, color : {:?}", person.name, person.color);
        }
    }
}
