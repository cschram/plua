use anyhow::{anyhow, Result};
use mlua::Lua;
use pest::Parser;
use pest_derive::Parser;

const HEADER_SRC: &str = include_str!("header.lua");
const FOOTER_SRC: &str = include_str!("footer.lua");

#[derive(Parser)]
#[grammar = "src/plua.pest"]
pub struct PluaParser;

pub struct Plua {
    name: String,
    source: String,
    metaprogram: String,
    lua: Lua,
}

impl Plua {
    pub fn new(name: &str, source: &str) -> Result<Self> {
        Ok(Self {
            name: name.to_owned(),
            source: source.to_owned(),
            metaprogram: Self::parse(source)?,
            lua: Lua::new(),
        })
    }

    fn parse(source: &str) -> Result<String> {
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
                            Rule::MetaInterpolate => {
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
                    let filename = pair.into_inner().next().expect("Expected Filename");
                    // TODO: Inline included plua
                }
                _ => {}
            }
        }
        metaprogram.push(FOOTER_SRC.to_owned());
        Ok(metaprogram.join("\n"))
    }

    fn escape(s: &str) -> String {
        s.replace("\"", "\\\"").to_string()
    }

    pub fn metaprogram(&self) -> String {
        self.metaprogram.clone()
    }

    pub fn exec(&self) -> Result<String> {
        let output: String = self
            .lua
            .load(&self.metaprogram)
            .set_name(&self.name)
            .eval()?;
        Ok(output)
    }
}
