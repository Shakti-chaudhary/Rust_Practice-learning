enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Billy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Anna".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!(
                    "Backstage ticket holder {:?}, and price is {:?}",
                    holder, price
                )
            }
            Ticket::Standard(price) => println!("The price of standard {:?}", price),
            Ticket::Vip(holder, price) => {
                println!("Vip ticket holder {:?}, and price is {:?}", price, holder)
            }
        }
    }
}
