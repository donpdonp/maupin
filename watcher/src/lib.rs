pub mod compound;
pub mod config;
pub mod update;

use once_cell::sync::Lazy;
pub static CONFIG: Lazy<config::Config> = Lazy::new(|| config::load());
