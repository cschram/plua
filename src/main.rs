mod plua;

use crate::plua::Plua;
use anyhow::Result;

const SYNTAX_EXAMPLE_SRC: &str = include_str!("../examples/syntax.plua");

fn main() -> Result<()> {
    let plua = Plua::new("syntax.plua", SYNTAX_EXAMPLE_SRC)?;
    // println!("{}", plua.metaprogram());
    std::fs::write("metaprogram.lua", plua.metaprogram());
    // println!("{}", plua.exec()?);
    std::fs::write("program.lua", plua.exec()?);
    Ok(())
}
