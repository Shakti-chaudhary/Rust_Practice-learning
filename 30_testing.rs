// copy all the code to main.rs in src/  to run tests

fn all_caps(world: &str) -> String {
    world.to_uppercase()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}
