use anyhow::{Result, anyhow};
use clap::Parser;

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
