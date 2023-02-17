struct Customer {
    name: String,
    age: i32,
}

fn is_able(user: &Customer) -> Result<String, String> {
    if user.age >= 21 {
        Ok(user.name.to_owned())
    } else {
        Err("age restricted".to_owned())
    }
}

fn main() {
    let customer = Customer {
        name: "Rohan".to_owned(),
        age: 15,
    };
    let customer1 = Customer {
        name: "Anna".to_owned(),
        age: 22,
    };
    let able = is_able(&customer);
    let able1 = is_able(&customer1);

    match able {
        Ok(able) => println!("{:?} is able to purchase", able),
        Err(e) => println!("{:?}, is {:?}", customer.name, e),
    }
    match able1 {
        Ok(able) => println!("{:?} is able to purchase", able),
        Err(e) => println!("{:?}, is {:?}", customer1.name, e),
    }
}
