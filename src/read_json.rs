use serde::{de::DeserializeOwned, Deserialize};
use std::{error::Error, fmt::Debug, fs::File, path::Path};

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub user: String,
}

pub fn read_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<Vec<T>, Box<dyn Error>> {
    let file = File::open(path)?;

    let user: Vec<T> = serde_json::from_reader(file)?;

    Ok(user)
}
