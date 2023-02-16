struct Temperature {
    degree_f: f64,
}
impl Temperature {
    fn freezing() -> Self {
        Self { degree_f: 32.0 }
    }
    fn boiling() -> Self {
        Self { degree_f: 212.0 }
    }

    fn show_temp(&self) {
        {
            println!("temp = {:?}", self.degree_f);
        }
    }
}
fn main() {
    let hot = Temperature { degree_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boil = Temperature::boiling();
    boil.show_temp()
}
