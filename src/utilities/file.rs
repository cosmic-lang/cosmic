use std::{path::PathBuf, env, rc::Rc};

use anyhow::{Result, anyhow};

pub fn read_file(path: &PathBuf, ext: &str) -> Result<Rc<str>> {
    let path = env::current_dir()?.join(path);
    let extension = path.extension().unwrap().to_str().unwrap();

    if extension != ext {
      return Err(anyhow!(format!("Invalid extension: {extension}, expected: {ext}")))
    }

    let source: Rc<str>;
    match path.to_str() {
      Some(path) => match std::fs::read_to_string(path) {
        Ok(src) => source = src.into(),
        Err(msg) => return Err(anyhow!(msg))
      },
      None => return Err(anyhow!("Could not convert path to string")) 
    }

    Ok(source)
}
