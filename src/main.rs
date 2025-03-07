use anyhow::{Error, Result, anyhow};
use clap::Parser;
use log::{error, info};
use plua::Plua;
use simple_logger::SimpleLogger;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "Plua")]
#[command(version = "0.1")]
#[command(about = "Lua preprocessor")]
pub struct PluaCli {
    /// Input plua file.
    pub input: String,

    /// Output lua file.
    pub output: String,

    /// Pass an environment global in the format name=value.
    #[arg(short, long)]
    pub env: Vec<String>,

    /// Supress stdout logging.
    #[arg(short, long)]
    pub quiet: bool,

    /// Enable debug mode. Metaprograms will be written as a .meta.lua file.
    #[arg(short, long)]
    pub debug: bool,
}

impl PluaCli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn parse_env(&self) -> Result<Vec<PluaEnv>> {
        let mut env = vec![];
        for e in self.env.iter() {
            let parts: Vec<&str> = e.split("=").collect();
            if parts.len() != 2 {
                return Err(anyhow!("Expected env syntax name=value"));
            }
            let name = parts[0].to_string();
            let value = if parts[1] == "true" {
                PluaEnvValue::Boolean(true)
            } else if parts[1] == "false" {
                PluaEnvValue::Boolean(false)
            } else if parts[1].parse::<f32>().is_ok() {
                PluaEnvValue::Number(parts[1].parse()?)
            } else {
                PluaEnvValue::String(parts[1].to_string())
            };
            env.push(PluaEnv { name, value });
        }
        Ok(env)
    }
}

#[derive(Debug)]
pub struct PluaEnv {
    pub name: String,
    pub value: PluaEnvValue,
}

#[derive(Debug)]
pub enum PluaEnvValue {
    String(String),
    Boolean(bool),
    Number(f64),
}

fn report_error(err: Error) {
    for cause in err.chain() {
        error!("{}", cause);
    }
}

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .unwrap();

    let cli = PluaCli::parse_args();
    let source = fs::read_to_string(&cli.input)?;
    let mut plua = Plua::new()?;

    for env in cli.parse_env()? {
        match &env.value {
            PluaEnvValue::String(s) => plua.set_global(&env.name, s.clone())?,
            PluaEnvValue::Boolean(b) => plua.set_global(&env.name, *b)?,
            PluaEnvValue::Number(n) => plua.set_global(&env.name, *n)?,
        }
    }

    match Plua::compile(&cli.input, &source) {
        Ok(program) => {
            if cli.debug {
                let meta_filename = {
                    let mut p = PathBuf::new();
                    p.push(&cli.output);
                    p.set_extension("meta.lua");
                    p.to_str().unwrap().to_owned()
                };
                fs::write(&meta_filename, &program.metaprogram)?;
                if !cli.quiet {
                    info!("Wrote metaprogram {}", &meta_filename);
                }
            }

            match plua.exec(&program) {
                Ok(output) => {
                    fs::write(&cli.output, output)?;
                    if !cli.quiet {
                        info!("Wrote lua {}", &cli.output);
                    }
                }
                Err(e) => {
                    report_error(e);
                }
            }
        }
        Err(e) => {
            report_error(e);
        }
    }
    Ok(())
}
