#[cfg(test)]
mod tests;

use super::types::CellPrint;
use pest::Parser;
use pest_derive::Parser;
use std::error::Error;

#[derive(Parser)]
#[grammar = "sheet/parsing/grammar.pest"]
struct CellParser;

pub fn parse_to_cell(raw: String) -> Result<Box<dyn CellPrint>, Box<dyn Error>> {
    let result = CellParser::parse(Rule::Function, &raw)?;
    // let first = result.into_iter().next().ok_or("Error getting first match")?;

    // match first.as_rule() {
    //     Rule::Function => todo!(),
    //     Rule::FunctionIdent => todo!(),
    //     Rule::Identifier => todo!(),
    //     Rule::digit => todo!(),
    //     Rule::alpha => todo!(),
    // }

    println!("{:#?}", result);

    Err("Parsing not implmented yet".into())
}
