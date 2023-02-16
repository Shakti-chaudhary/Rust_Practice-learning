struct Grocery {
    item: String,
    id: i32,
    quantity: i32,
}
fn display_item(item: &Grocery) {
    println!("name = {}", item.item);
}
fn display_quantity(item: &Grocery) {
    println!("quantity = {:?}", item.quantity);
}
fn display_id(item: &Grocery) {
    println!("id = {:?}", item.id);
}

fn main() {
    let item1 = Grocery {
        item: String::from("Mango"),
        id: 53522,
        quantity: 12,
    };
    display_item(&item1);
    display_id(&item1);
    display_quantity(&item1)
}
