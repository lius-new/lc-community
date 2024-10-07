#[cfg(test)]
mod config_tests {
    use std::collections::HashMap;

    use config::Config;
    use serde::Deserialize;

    /// test load simple(map) config
    ///
    /// config content:
    /// ```
    /// debug = false
    /// priority = 32
    /// key = "189rjfadoisfj8923fjio"
    /// ````
    #[test]
    fn test_load_simple_config() {
        let config = Config::builder()
            .add_source(config::File::with_name("../config/default"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        println!(
            "{:?}",
            config.try_deserialize::<HashMap<String, String>>().unwrap()
        )
    }

    /// test load struct config
    ///
    /// config content:
    /// ```
    /// [app]
    /// name = "lc"
    /// port = 3000
    ///
    /// [database]
    /// url = "postgres://postgres:password@localhost/test"
    /// max_connections = 5
    /// ````
    #[test]
    fn test_load_struct_config() {
        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct App {
            name: String,
            port: u32,
        }
        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Database {
            url: String,
            max_connections: u32,
        }
        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Config_ {
            app: App,
            database: Database,
        }

        let config = Config::builder()
            .add_source(config::File::with_name("../config/default"))
            .add_source(config::File::with_name("../config/production").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let c = config.try_deserialize::<Config_>().unwrap();
        println!("{:?}", c)
    }

    /// call crates::lcc-uitls::config:;AppConfig::build
    #[test]
    fn test_lc_utils_config_appconfig_build() {
        let c = lc_utils::config::AppConfig::build("../config/default").unwrap();

        println!("{:?}", c)
    }
}
