use configparser::ini::Ini;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Ini = {
        let mut config_parser = Ini::new();

        config_parser
            .load("./config.ini")
            .expect("Failed to load config.ini file");

        config_parser
    };
}
