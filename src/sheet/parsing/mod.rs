#[cfg(test)]
mod tests;

use super::types::{CellPrint, Coord, Sheet};
use crate::graph::types::{Id, SumNode};
use pest::Parser;
use pest_derive::Parser;
use std::error::Error;

#[derive(Parser)]
#[grammar = "sheet/parsing/grammar.pest"]
struct CellParser;

pub fn spreadsheet_to_coords(s: &str) -> Coord<usize> {
    let mut chars = s.chars();
    let mut x = 0;
    let mut rest = vec![];
    while let Some(c) = chars.next() {
        if c.is_ascii_alphabetic() {
            x = x * 26 + (c as usize - 'A' as usize + 1);
        } else {
            rest.push(c);
        }
    }

    let y = rest
        .clone()
        .into_iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
        - 1;

    Coord { x: x - 1, y }
}

pub fn coords_to_spreadsheet(coords: Coord<usize>) -> String {
    let mut result = String::new();

    let mut x = coords.x + 1;

    while x > 0 {
        let c = (x - 1) % 26;
        result.push((c as u8 + b'A') as char);
        x = (x - 1) / 26;
    }

    result.push_str(&format!("{}", coords.y + 1));

    result
}
impl<const WIDTH: usize, const HEIGHT: usize> Sheet<{ WIDTH }, { HEIGHT }> {
    pub fn parse_to_cell(
        &mut self,
        raw: String,
    ) -> Result<Id, Box<dyn Error>> {
        let result = CellParser::parse(Rule::Function, &raw)?;
        let first = result
            .into_iter()
            .next()
            .ok_or("Error getting first match")?;

        // match first.as_rule() {
        //     Rule::Function => todo!(),
        //     Rule::FunctionIdent => todo!(),
        //     Rule::Identifier => todo!(),
        //     Rule::digit => todo!(),
        //     Rule::alpha => todo!(),
        // }

        let mut inner = first.into_inner().into_iter();
        let identifier = inner.next().ok_or("No Identifier")?;

        let thing = match identifier.as_str() {
            "Sum" => {
                let mut rest: Vec<Id> = vec![];
                while let Some(ident) = inner.next() {
                    if ident.as_rule() == Rule::Identifier {
                        let coords = spreadsheet_to_coords(ident.as_str());
                        let node = self.get_node(coords);
                        if node.is_some() {
                            rest.push(node.unwrap())
                        }
                    }
                }

                let new_node = self.graph.add_node(Box::new(SumNode::new()));

                rest.into_iter().for_each(|x| {
                    self.graph.connect(x, new_node);
                });
                new_node
            }
            otherwise => return Err(format!("Unimplmented Function {}", otherwise).into()),
        };

        return Ok(thing)
    }
}
