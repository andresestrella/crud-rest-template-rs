//loads configs from ENV
use config::{ConfigError}; 
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig{
    pub host:String,
    pub port: i32
}

#[derive(Deserialize)]
pub struct DbConfig{
    pub database:String, // MARIADB.DATABASE
    //pub root_user:String,  //problema con esta variable MARIADB.ROOT.USER, a cual nombre se mapearia?
    //pub root_password:String,
    pub host:String,
    pub port:i32,
    //pub pool_max:i32,
}

#[derive(Deserialize)]
pub struct Config{
    pub server: ServerConfig,
    pub mariadb: DbConfig
}

impl Config{
    pub fn from_env() -> Result<Self, ConfigError>{
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}