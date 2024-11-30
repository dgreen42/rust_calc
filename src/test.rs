#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_simple_addition() {
        let test = String::from("1+2");
        let result = expression(test).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_simple_subtraction() {
        let test = String::from("5-2");
        let result = expression(test).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_simple_multiplication() {
        let test = String::from("2*2");
        let result = expression(test).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_simple_division() {
        let test = String::from("9/3");
        let result = expression(test).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_pemdas_1() {
        let test = String::from("2+8/4");
        let result = expression(test).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_pemdas_2() {
        let test = String::from("2*8-4");
        let result = expression(test).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_pemdas_3() {
        let test = String::from("2*8-4/2");
        let result = expression(test).unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    fn test_paren_beginning() {
        let test = String::from("(2*8)-4/2");
        let result = expression(test).unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    fn test_paren_middle() {
        let test = String::from("2*(8-4)/2");
        let result = expression(test).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_paren_end() {
        let test = String::from("2*8-(4/2)");
        let result = expression(test).unwrap();
        assert_eq!(result, 14);
    }
}
