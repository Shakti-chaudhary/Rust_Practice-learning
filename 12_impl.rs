struct Temperature {
    degree_f: f64,
}
impl Temperature {
    fn show_temp(temp: Temperature) {
        {
            println!("temp = {:?}", temp.degree_f);
        }
    }
}
fn main() {
    let hot = Temperature { degree_f: 99.9 };
    Temperature::show_temp(hot)
}
