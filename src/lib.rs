use anyhow::{Context, Result};
use log::{error, warn};
use mlua::{IntoLua, Lua};
use pest::Parser;
use pest_derive::Parser;
use std::{fs, path::PathBuf};

const HEADER_SRC: &str = include_str!("header.lua");
const FOOTER_SRC: &str = include_str!("footer.lua");

#[derive(Parser)]
#[grammar = "src/plua.pest"]
struct PluaParser;

pub struct PluaProgram {
    pub name: String,
    pub metaprogram: String,
}

pub struct Plua {
    lua: Lua,
}

impl Plua {
    pub fn new() -> Result<Self> {
        let lua = unsafe { Lua::unsafe_new() };
        let globals = lua.globals();

        // Plua compile time library
        let plua_lib = lua.create_table()?;
        // Plua.warn
        let plua_warn = lua.create_function(|lua, msg: String| {
            let debug = lua.inspect_stack(1).context("Unable to retrieve stack")?;
            warn!("Warning on line {}: {}", debug.curr_line(), msg);
            Ok(())
        })?;
        plua_lib.set("warn", plua_warn)?;
        // Plua.error
        let plua_error = lua.create_function::<_, String, Result<()>>(|lua, err: String| {
            let debug = lua.inspect_stack(1).context("Unable to retrieve stack")?;
            error!("Error on line {}: {}", debug.curr_line(), err);
            Err(mlua::Error::RuntimeError(err))
        })?;
        plua_lib.set("error", plua_error)?;
        globals.set("Plua", plua_lib)?;

        Ok(Self { lua })
    }

    pub fn set_global(&mut self, name: &str, value: impl IntoLua) -> Result<()> {
        self.lua.globals().set(name, value)?;
        Ok(())
    }

    pub fn compile(name: &str, source: &str) -> Result<PluaProgram> {
        let metaprogram =
            Self::parse(name, source).with_context(|| format!("Error parsing {}", name))?;
        Ok(PluaProgram {
            name: name.to_string(),
            metaprogram: format!("{}{}{}", HEADER_SRC, &metaprogram, FOOTER_SRC),
        })
    }

    pub fn exec(&self, program: &PluaProgram) -> Result<String> {
        let output: String = self
            .lua
            .load(&program.metaprogram)
            .set_name(&program.name)
            .eval()
            .with_context(|| format!("Error compiling {}", &program.name))?;
        Ok(output)
    }

    fn parse(name: &str, source: &str) -> Result<String> {
        let mut metaprogram = vec![];
        let pairs = PluaParser::parse(Rule::Program, source)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::Lua => {
                    metaprogram.push(format!("Plua.emit(\"{}\")", Self::escape(pair.as_str())))
                }
                Rule::MetaIncludeFile => {
                    let include_filename = pair.as_str().to_string();
                    let include_path = {
                        let mut buf = PathBuf::new();
                        buf.push(name);
                        buf.pop();
                        buf.push(&include_filename);
                        buf.set_extension("plua");
                        buf.to_str().unwrap().to_owned()
                    };
                    let include_source = fs::read_to_string(&include_path)
                        .with_context(|| format!("Error reading include {}", &include_path))?;
                    metaprogram.push(
                        Self::parse(&include_path, &include_source)
                            .with_context(|| format!("Error parsing {}", &include_path))?,
                    );
                }
                Rule::MetaCodeSingleBody | Rule::MetaCodeMultiBody => {
                    let mut content = pair.as_str().to_string();
                    let maybe_inner = pair.into_inner().next();
                    if let Some(inner) = maybe_inner {
                        if matches!(inner.as_rule(), Rule::MetaCodeInterpolateBody) {
                            content = format!("\"{}\"", Self::escape(inner.as_str()));
                        }
                    }
                    metaprogram.push(content);
                }
                Rule::MetaCodeInterpolateBody => {
                    metaprogram.push(format!("\"{}\"", Self::escape(pair.as_str())));
                }
                Rule::MetaValueInterpolateBody => {
                    metaprogram.push(format!(
                        "Plua.emit(Plua.format_value({}))",
                        Self::escape(pair.as_str())
                    ));
                }
                Rule::MetaExpressionInterpolateBody => {
                    metaprogram.push(format!("Plua.emit({})", pair.as_str()));
                }
                Rule::End => {
                    metaprogram.push("\n".to_string());
                }
                _ => {}
            }
        }
        Result::<String>::Ok(metaprogram.join("\n"))
    }

    fn escape(s: &str) -> String {
        s.replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .to_string()
    }
}
