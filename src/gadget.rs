use std::convert::TryFrom;
use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use directories::BaseDirs;
use regex::Regex;

#[derive(Debug)]
pub struct GadgetResult {
    pub icon: String,
    pub data: Result<String>,
}

impl GadgetResult {
    pub fn print(&self, space_size: usize, placeholder: &str) -> String {
        let space = str::repeat(" ", space_size);
        let realdata = match &self.data {
            Ok(d) => d.trim_start().trim_end(),
            _ => placeholder,
        };
        format!("{}{}{}", self.icon, space, realdata)
    }
}

#[derive(Debug)]
pub enum Gadget {
    Synchronization,
    Git,
    Thunderbird,
    Memory,
    Temperature,
}

impl Gadget {
    pub fn run(&self) -> GadgetResult {
        match self {
            Gadget::Synchronization => GadgetResult {
                icon: "".to_string(),
                data: run_synchronization(),
            },
            Gadget::Git => GadgetResult {
                icon: "".to_string(),
                data: run_git(),
            },
            Gadget::Thunderbird => GadgetResult {
                icon: "".to_string(),
                data: run_thunderbird(),
            },
            Gadget::Memory => GadgetResult {
                icon: "".to_string(),
                data: run_memory(),
            },
            Gadget::Temperature => GadgetResult {
                icon: "".to_string(),
                data: run_temperature(),
            },
        }
    }
}

impl TryFrom<&str> for Gadget {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "synchronization" => Ok(Gadget::Synchronization),
            "git" => Ok(Gadget::Git),
            "thunderbird" => Ok(Gadget::Thunderbird),
            "memory" => Ok(Gadget::Memory),
            "temperature" => Ok(Gadget::Temperature),
            _ => Err("invalid gadget".to_string()),
        }
    }
}

fn run_synchronization() -> Result<String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let sync_dir = base_dirs.data_dir().join("sync-*.lock").display().to_string();
        let glob_iter = glob::glob(&sync_dir).context("Unable to iterate data directory")?;
        let count = glob_iter.filter(|p| p.is_ok()).count();

        return Ok(count.to_string());
    }

    bail!("Unable to get data directory");
}

fn run_git() -> Result<String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let sync_dir = base_dirs.home_dir().join("git").join(".statistics");
        let contents = fs::read_to_string(sync_dir).context("Unable to read statistics file of Git repositories")?;

        let split = contents.split(',').collect::<Vec<&str>>();

        if split.len() != 3 {
            bail!("Statistics file of Git repositories is malformed");
        }

        return Ok(split.join(" "));
    }

    bail!("Unable to get data directory");
}

fn run_thunderbird() -> Result<String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let sync_dir = base_dirs.data_dir().join("tbunread").join("count");
        let contents = fs::read_to_string(sync_dir).context("Unable to read count file of tbunread")?;

        return Ok(contents);
    }

    bail!("Unable to get data directory");
}

fn run_memory() -> Result<String> {
    let path = Path::new("/proc/meminfo").display().to_string();
    let contents = fs::read_to_string(path).context("Unable to read meminfo file")?;

    let re = Regex::new(r"^MemTotal: \s+(\d+) kB").unwrap();
    let cap = re.captures(&contents).unwrap();
    let total: f64 = cap[1].parse().context("meminfo file is malformed")?;

    let re = Regex::new(r"MemAvailable: \s+(\d+) kB").unwrap();
    let cap = re.captures(&contents).unwrap();
    let available: f64 = cap[1].parse().context("meminfo file is malformed")?;

    let percent = (available / total) * 100.;
    let percent = 100 - percent as usize;

    Ok(format!("{:02}%", percent))
}

fn run_temperature() -> Result<String> {
    let glob_pattern = "/sys/class/hwmon/hwmon*/temp*_input";
    let glob_iter = glob::glob(glob_pattern).context("Unable to iterate hwmon files")?;

    let mut temperatures = vec![];

    for file in glob_iter {
        match file {
            Ok(path) => {
                let contents =
                    fs::read_to_string(&path).with_context(|| format!("Failed to read file: {}", path.display()))?;

                let temp: i32 = contents
                    .trim()
                    .parse()
                    .with_context(|| format!("Failed to parse temperature in file: {}", path.display()))?;

                let temp_in_celsius = temp / 1000;

                if (10..=150).contains(&temp_in_celsius) {
                    temperatures.push(temp_in_celsius);
                }
            }
            Err(e) => {
                eprintln!("Error accessing file: {:?}", e);
            }
        }
    }

    if temperatures.is_empty() {
        bail!("No valid temperatures found");
    }

    let max_temperature = temperatures.into_iter().max().unwrap();

    Ok(format!("{:02}°", max_temperature))
}
