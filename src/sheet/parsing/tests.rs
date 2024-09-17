use crate::{graph::types::ValueNode, sheet::{parsing::{coords_to_spreadsheet, spreadsheet_to_coords}, types::{Coord, Sheet}}};

#[test]
fn parse_test() {
    let mut sheet = Sheet::<10,10>::new();
    let a = sheet.add_node(Coord::<usize> {x: 0, y: 0}, Box::new(ValueNode::new(1.0)));
    let b = sheet.add_node(Coord::<usize> {x: 0, y: 1}, Box::new(ValueNode::new(2.0)));

    let ab = sheet.parse_to_cell("Sum(A1, A2)".to_owned()).unwrap();

    assert_eq!(sheet.graph.get_node_value(ab), Some(3.0))
}

#[test]
fn thing_to_coord() {
    assert_eq!(spreadsheet_to_coords("A1").x, 0);
    assert_eq!(spreadsheet_to_coords("A1").y, 0);
    assert_eq!(spreadsheet_to_coords("AA1").x, 26);
    assert_eq!(spreadsheet_to_coords("AA1").y, 0);
    assert_eq!(spreadsheet_to_coords("A13").y, 12);
}

#[test]
fn coord_to_thing() {
    assert_eq!(coords_to_spreadsheet(Coord::<usize> {x: 0, y: 0}), "A1");
    assert_eq!(coords_to_spreadsheet(Coord::<usize> {x: 3, y: 3}), "D4");
    assert_eq!(coords_to_spreadsheet(Coord::<usize> {x: 26, y: 0}), "AA1");
    assert_eq!(coords_to_spreadsheet(Coord::<usize> {x: 1, y: 51}), "B52");

}