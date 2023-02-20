trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn square_meter(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.square_meter()
    }
}

struct Carpet(f64);
impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.0
    }
    fn square_meter(&self) -> f64 {
        self.0
    }
}
struct Tile(f64);
impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.0
    }
    fn square_meter(&self) -> f64 {
        self.0
    }
}
struct Wood(f64);
impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.0
    }
    fn square_meter(&self) -> f64 {
        self.0
    }
}

fn total_cost(material: &Vec<Box<dyn Material>>) -> f64 {
    material.iter().map(|mat| mat.total_cost()).sum()
}

fn main() {
    let meter = 20.0;
    let carpet = Box::new(Carpet(meter));
    let tile = Box::new(Tile(meter));
    let wood = Box::new(Wood(meter));

    let costs: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    let amount = total_cost(&costs);
    println!("Total amount = ${:?}", amount);
}
