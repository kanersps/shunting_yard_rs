#[cfg(test)]
mod test {
    use crate::Algorithm;

    #[test]
    fn simple_expression() {
        run_test("10 * 2 + 100", 120);
    }

    #[test]
    fn advanced_expression() {
        run_test("((100 + 1) * 2) + 500 + (300 * 2 / 15) * 10", 1102);
    }

    fn run_test(input: &str, expected: i32) {
        let mut algorithm = Algorithm::default();

        let result = algorithm.execute(input);

        if let Err(e) = result {
            panic!("{:?}", e)
        } else if let Ok(returned) = result {
            assert_eq!(expected, returned);
        }
    }
}
