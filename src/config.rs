use std::env;

use configparser::ini::Ini;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Ini = {
        let mut args = env::args();

        let config_path = args.nth(1).unwrap();

        let mut config_parser = Ini::new();

        config_parser.load(config_path).unwrap();

        config_parser
    };
}
