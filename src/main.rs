use anyhow::{anyhow, Context, Error, Result};
use clap::Parser;
use glob::glob;
use log::{error, info};
use plua::Plua;
use simple_logger::SimpleLogger;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(name = "Plua")]
#[command(version = "0.1-alpha")]
#[command(about = "Lua preprocessor")]
pub struct PluaCli {
    /// Source plua file.
    pub source: String,

    /// Output directory. If omitted, the source directory will be used.
    #[arg(short, long)]
    pub output: Option<String>,

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

fn write_lua(filename: &str, source: &str) -> Result<()> {
    match stylua_lib::format_code(
        source,
        stylua_lib::Config::new(),
        None,
        stylua_lib::OutputVerification::None,
    )
    .context("Failed to format lua")
    {
        Ok(formatted) => {
            fs::write(filename, formatted)?;
        }
        Err(err) => {
            report_error(err);
            // Fail gracefull and write the unformatted code so it can be debugged.
            fs::write(filename, source)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .unwrap();

    let cli = PluaCli::parse();
    let mut plua = Plua::new()?;

    for file in glob(&cli.source)? {
        let path = file?.to_str().unwrap().to_string();
        println!("{}", path);
        let source = fs::read_to_string(&path)?;
        let source_filename = Path::new(&path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let dest_dir = match &cli.output {
            Some(dir) => dir,
            None => Path::new(&path).parent().unwrap().to_str().unwrap(),
        };
        let dest_file = {
            let mut p = PathBuf::new();
            p.push(&dest_dir);
            p.push(&source_filename);
            p.set_extension("lua");
            p.to_str().unwrap().to_owned()
        };
        let meta_file = {
            let mut p = PathBuf::new();
            p.push(&dest_dir);
            p.push(&source_filename);
            p.set_extension("meta.lua");
            p.to_str().unwrap().to_owned()
        };
        fs::create_dir_all(&dest_dir)?;

        for env in cli.parse_env()? {
            match &env.value {
                PluaEnvValue::String(s) => plua.set_global(&env.name, s.clone())?,
                PluaEnvValue::Boolean(b) => plua.set_global(&env.name, *b)?,
                PluaEnvValue::Number(n) => plua.set_global(&env.name, *n)?,
            }
        }

        match Plua::compile(&path, &source) {
            Ok(program) => {
                if cli.debug {
                    write_lua(&meta_file, &program.metaprogram)?;
                    if !cli.quiet {
                        info!("Wrote metaprogram {}", &meta_file);
                    }
                }

                match plua.exec(&program) {
                    Ok(output) => {
                        write_lua(&dest_file, &output)?;
                        if !cli.quiet {
                            info!("Wrote lua {}", &dest_file);
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
    }
    Ok(())
}
