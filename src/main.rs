mod cli;

use crate::cli::{PluaCli, PluaEnvValue};
use anyhow::{Error, Result};
use log::{error, info};
use plua::Plua;
use simple_logger::SimpleLogger;
use std::{fs, path::PathBuf};

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
