mod cli;

use crate::cli::{PluaCli, PluaEnvValue};
use anyhow::Result;
use plua::Plua;
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let cli = PluaCli::parse_args();
    let source = fs::read_to_string(&cli.input)?;
    let mut plua = Plua::new()?;

    for env in cli.parse_env()? {
        match &env.value {
            PluaEnvValue::String(s) => plua.set_global(&env.name, s.clone())?,
            PluaEnvValue::Boolean(b) => plua.set_global(&env.name, b.clone())?,
            PluaEnvValue::Number(n) => plua.set_global(&env.name, n.clone())?,
        }
    }

    let meta_filename = {
        let mut p = PathBuf::new();
        p.push(&cli.output);
        p.set_extension("meta.lua");
        p.to_str().unwrap().to_owned()
    };
    let error_log_filename = {
        let mut p = PathBuf::new();
        p.push(&cli.output);
        p.set_extension("error.log");
        p.to_str().unwrap().to_owned()
    };

    match Plua::parse(&meta_filename, &source) {
        Ok(program) => {
            if cli.meta {
                fs::write(&meta_filename, &program.metaprogram)?;
                if !cli.quiet {
                    println!("Wrote lua output {}", &meta_filename);
                }
            }

            match plua.exec(&program) {
                Ok(output) => {
                    fs::write(&cli.output, output)?;
                    if !cli.quiet {
                        println!("Wrote metaprogram {}", &cli.output);
                    }
                    Ok(())
                }
                Err(e) => {
                    fs::write(&meta_filename, &program.metaprogram)?;
                    fs::write(&error_log_filename, e.to_string())?;
                    if !cli.quiet {
                        println!("Wrote metaprogram {}", &meta_filename);
                        println!("Wrote error log {}", &error_log_filename);
                    }
                    Err(e)
                }
            }
        }
        Err(e) => {
            fs::write(&error_log_filename, e.to_string())?;
            if !cli.quiet {
                println!("Wrote error log {}", &error_log_filename);
            }
            Err(e)
        }
    }
}
