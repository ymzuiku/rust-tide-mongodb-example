use std::{fs::File, io::Read, path::Path};

use serde_json;
use std::collections::HashMap;

static mut APP_ENV: Option<&'static HashMap<String, String>> = None;

fn first_load() -> HashMap<String, String> {
    let path = Path::new("./env.json");
    if !path.is_file() {
        panic!("Not find env.json");
    }

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    // let data: &'static str = ;
    let s = Box::leak(data.into_boxed_str());
    let a: HashMap<String, String> = serde_json::from_str(s).unwrap();

    return a;
}

pub fn load() -> &'static HashMap<String, String> {
    unsafe {
        match APP_ENV {
            Some(e) => e,
            None => {
                APP_ENV = Some(Box::leak(Box::new(first_load())));
                return APP_ENV.unwrap();
            }
        }
    }
}
