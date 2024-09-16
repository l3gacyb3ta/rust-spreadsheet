use super::parse_to_cell;

#[test]
fn parse_test() {
    parse_to_cell("Sum(A1)".to_owned());
}