use anyhow::{anyhow, Result};
use mlua::{IntoLua, Lua};
use pest::Parser;
use pest_derive::Parser;

const HEADER_SRC: &str = include_str!("header.lua");
const FOOTER_SRC: &str = include_str!("footer.lua");

#[derive(Parser)]
#[grammar = "src/plua.pest"]
pub struct PluaParser;

pub struct PluaProgram {
    pub name: String,
    pub metaprogram: String,
}

pub struct Plua {
    lua: Lua,
}

impl Plua {
    pub fn new() -> Result<Self> {
        Ok(Self { lua: Lua::new() })
    }

    pub fn set_global(&mut self, name: &str, value: impl IntoLua) -> Result<()> {
        self.lua.globals().set(name, value)?;
        Ok(())
    }

    pub fn parse(name: &str, source: &str) -> Result<PluaProgram> {
        let mut metaprogram = vec![HEADER_SRC.to_owned()];
        let pairs = PluaParser::parse(Rule::Program, source)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::LuaLine => {
                    let mut line = vec![];
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::Lua => {
                                line.push(format!("\"{}\"", Self::escape(inner_pair.as_str())));
                            }
                            Rule::MetaValueInterpolate => {
                                let inner_lua =
                                    inner_pair.into_inner().next().expect("Expected Lua");
                                line.push(format!("__format_value({})", inner_lua.as_str()));
                            }
                            Rule::MetaCodeInterpolate => {
                                let inner_lua =
                                    inner_pair.into_inner().next().expect("Expected Lua");
                                line.push(format!("({})", inner_lua.as_str()));
                            }
                            _ => {
                                return Err(anyhow!(format!(
                                    "Unexpected rule \"{:?}\"",
                                    inner_pair.as_rule()
                                )))
                            }
                        }
                    }
                    metaprogram.push(format!("__emit({})", line.join(" .. ")));
                }
                Rule::MetaLine => {
                    let metaline_content =
                        pair.into_inner().next().expect("Expected MetalineContent");
                    metaprogram.push(metaline_content.as_str().to_string());
                }
                Rule::MetaInclude => {
                    let _filename = pair.into_inner().next().expect("Expected Filename");
                    // TODO: Inline included plua
                }
                _ => {}
            }
        }
        metaprogram.push(FOOTER_SRC.to_owned());
        Ok(PluaProgram {
            name: name.to_string(),
            metaprogram: metaprogram.join("\n"),
        })
    }

    fn escape(s: &str) -> String {
        s.replace("\"", "\\\"").to_string()
    }

    pub fn exec(&self, program: &PluaProgram) -> Result<String> {
        let output: String = self
            .lua
            .load(&program.metaprogram)
            .set_name(&program.name)
            .eval()?;
        Ok(output)
    }
}
