use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum Status {
    Available,
    Unavailable,
    Rented,
    Maintenance,
}
#[derive(Debug)]
struct Rental {
    status: Status,
    vehicle: Vehicle,
    vin: String,
}
struct Corporate(Rc<RefCell<Vec<Rental>>>);

struct StoreFront(Rc<RefCell<Vec<Rental>>>);

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                status: Status::Available,
                vehicle: Vehicle::Car,
                vin: "123".to_owned(),
            },
            Rental {
                status: Status::Maintenance,
                vehicle: Vehicle::Truck,
                vin: "abc".to_owned(),
            },
        ];
        let vehicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate(Rc::clone(&vehicles));
        let storefront = StoreFront(Rc::clone(&vehicles));
        {
            let mut rentals = storefront.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Available);
                car.status = Status::Rented;
            }
        }
        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Rented);
                car.status = Status::Available;
            }
        }

        let rentals = storefront.0.borrow_mut();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, Status::Available);
        }
    }
}
