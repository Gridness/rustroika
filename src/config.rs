use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use evalexpr::eval_int;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::{cli::args::parse_expression, prelude::*};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub defaults: Option<Defaults>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Defaults {
    #[serde(deserialize_with = "parse_expression_serde")]
    pub trips_per_week: Option<u32>,
    pub monthly_cost: Option<u32>,
    #[serde(deserialize_with = "parse_expression_serde")]
    pub ticket_price: Option<u32>,
}

fn parse_expression_serde<'de, D>(deserializer: D) -> Result<Option<u32>>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(expr) => eval_int(&expr)
            .map(|n| Some(n as u32))
            .map_err(|e| D::Error::custom(format!("Invalid expression: {}", e))),
        None => Ok(None),
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            trips_per_week: None,
            monthly_cost: None,
            ticket_price: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
    }

    fn config_path() -> Result<PathBuf> {
        ProjectDirs::from("", "", "rustroika")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.yaml"))
            .ok_or_else(err)
    }

    fn get_defaults(&self) -> &Defaults {
        self.defaults.as_ref().unwrap_or(&Defaults::default())
    }
}

pub fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "rustroika")
        .map(|proj_dirs| proj_dirs.config_dir().join("config.yaml"))
}

pub fn load_config() -> Value {
    let config_path = match get_config_path() {
        Some(path) => path,
        None => return Value::Null,
    };

    if !config_path.exists() {
        return Value::Null;
    }

    match fs::read_to_string(&config_path) {
        Ok(contents) => serde_yaml::from_str(&contents).unwrap_or(Value::Null),
        Err(_) => Value::Null,
    }
}

pub fn save_config(config: &Value) -> Result<()> {
    let config_path = get_config_path().ok_or(anyhow::anyhow!("Couldn't get config path"))?;
    let config_dir = config_path.parent().unwrap();
    fs::create_dir_all(config_dir)?;
    fs::write(&config_path, serde_yaml::to_string(config)?)?;
    Ok(())
}

pub fn update_config(key: &str, value: &str) -> Result<()> {
    let mut config = load_config();
    let parsed_value =
        parse_expression(value).map_err(|e| anyhow::anyhow!("Invalid expression: {}", e))?;

    let mut current = &mut config;
    let parts: Vec<&str> = key.split('.').collect();

    for part in &parts[..parts.len() - 1] {
        current = current
            .as_mapping_mut()
            .ok_or(anyhow::anyhow!("Invalid config structure"))?
            .entry(part.to_string().into())
            .or_insert_with(|| Value::Mapping(Default::default()));
    }

    current[parts.last().unwrap()] = Value::Number(parsed_value.into());
    save_config(&config)
}

pub fn remove_config_value(config: &mut Value, path: &[&str]) -> bool {
    if path.is_empty() {
        return false;
    }

    if let Some((last, parts)) = path.split_last() {
        let mut current = config;
        for part in parts {
            current = match current.get_mut(*part) {
                Some(v) => v,
                None => return false,
            };
        }

        if let Some(map) = current.as_mapping_mut() {
            return map.remove(&Value::String(last.to_string())).is_some();
        }
    }
    false
}
