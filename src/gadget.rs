use std::convert::TryFrom;
use std::fs;
use std::path::Path;

use directories::BaseDirs;
use regex::Regex;

#[derive(Debug)]
pub struct GadgetResult {
    pub icon: String,
    pub data: Option<String>,
}

impl GadgetResult {
    pub fn print(&self, space_size: usize, placeholder: &str) -> String {
        let space = str::repeat(" ", space_size);
        let realdata = match &self.data {
            Some(d) => d.trim_start().trim_end(),
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

// https://stackoverflow.com/a/51345372
macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return None,
        }
    };
}

fn run_synchronization() -> Option<String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let sync_dir = base_dirs.data_dir().join("sync-*.lock").display().to_string();
        let count = unwrap_or_return!(glob::glob(&sync_dir)).filter(|p| p.is_ok()).count();

        return Some(count.to_string());
    }

    None
}

fn run_git() -> Option<String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let sync_dir = base_dirs.home_dir().join("git").join(".statistics");
        let contents = unwrap_or_return!(fs::read_to_string(sync_dir));

        let split = contents.split(',').collect::<Vec<&str>>();

        if split.len() != 3 {
            return None;
        }

        return Some(split.join(" "));
    }

    None
}

fn run_thunderbird() -> Option<String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let sync_dir = base_dirs.data_dir().join("thunderbird-unread").join("all.count");
        let contents = unwrap_or_return!(fs::read_to_string(sync_dir));

        return Some(contents);
    }

    None
}

fn run_memory() -> Option<String> {
    let path = Path::new("/proc/meminfo").display().to_string();
    let contents = unwrap_or_return!(fs::read_to_string(path));

    let re = Regex::new(r"^MemTotal: \s+(\d+) kB").unwrap();
    let cap = re.captures(&contents).unwrap();
    let total: f64 = unwrap_or_return!(cap[1].parse());

    let re = Regex::new(r"MemAvailable: \s+(\d+) kB").unwrap();
    let cap = re.captures(&contents).unwrap();
    let available: f64 = unwrap_or_return!(cap[1].parse());

    let percent = (available / total) * 100.;
    let percent = 100 - percent as usize;

    Some(format!("{:02}%", percent))
}

fn run_temperature() -> Option<String> {
    let path = Path::new("/sys/devices/platform/coretemp.0/hwmon");
    let path = path.join("*").join("temp*_input").display().to_string();
    let temperature = unwrap_or_return!(glob::glob(&path))
        .map(|file| match file {
            Ok(p) => {
                let contents = fs::read_to_string(p).expect("could not read file");
                let contents = contents.trim_start().trim_end();
                let contents: i32 = contents.parse().expect("temperature is not a number");
                contents / 1000
            }
            Err(_) => std::i32::MIN,
        })
        .max();

    match temperature {
        Some(t) => Some(format!("{:02}°", t)),
        _ => None,
    }
}
