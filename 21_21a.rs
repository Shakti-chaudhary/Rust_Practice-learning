use std::collections::HashMap;
fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chairs".to_owned(), 5);
    stock.insert("Beds".to_owned(), 3);
    stock.insert("Tables".to_owned(), 2);
    stock.insert("Couches".to_owned(), 0);

    let mut total_stock = 0;
    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;
        match qty {
            0 => println!("the name is: {:?}, Out of stock", item),
            _ => println!("the name is: {:?}, and quantity is {:?}", item, qty),
        }
    }
    println!("{:?} total items", total_stock);
}
