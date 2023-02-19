// copy all the code to main.rs in src/  to run tests

fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_lower() {
        let result = clamp(10, 100, 100);
        let expected = 100;
        assert_eq!(result, expected, "should be 100");
    }
    #[test]
    fn clamp_upper() {
        let result = clamp(5000, 100, 1000);
        let expected = 1000;
        assert_eq!(result, expected, "should be 1000");
    }
    #[test]
    fn clamp_div() {
        let result = divide(1, 1);
        let expected = Some(1);
        assert_eq!(result, expected, "should be 1");
    }
    #[test]
    fn check_concat() {
        let result = concat("a", "b");
        let expected = String::from("ab");
        assert_eq!(result, expected, "should be ab");
    }
}
