use std::collections::HashMap;
use std::path::PathBuf;
  
use config::{Config as Cfg, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    // version: Option<String>,
    
    #[serde(default = "default_is_overlay")]
    pub overlay: bool,

    name: Option<String>,

    description: Option<String>,

    target: Option<String>,

    exclude: Option<Vec<String>>,

    git: Option<HashMap<String, String>>,

    install: Option<HashMap<String, Vec<String>>>,
}

fn default_is_overlay() -> bool {true}

// impl std::default::Default for Config {
//     fn default() -> Self {
//         Self {
//             overlay: true,
//             name: None,
//             description: None,
//             target: None,
//             exclude: None,
//             git: None,
//             install: None
//         }
//     }
// }

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self, ConfigError> {
        // println!("path: {}", path.display());
        // let config: Self = Figment::new()
        // let mut f = Figment::new();
        // Figment::from(Serialized::defaults(Config::default()));

        // println!("{:#?}", path.extension());

        // f = match path.extension().and_then(OsStr::to_str) {
        //     Some("yaml" | "yml") => f.merge(Yaml::file(path)),
        //     Some("toml") => f.merge(Toml::file(&path)),
        //     Some("json") => f.merge(Json::file(&path)),
        //     _ => f,
        // };

        let mut s = Cfg::new();

        s.merge(File::with_name(path.to_str().unwrap()))?;
            // Start off by merging in the "default" configuration file
            // .add_source(File::with_name(path))
            // ;
            //     with_name("examples/hierarchical-env/config/default"))
            // // Add in the current environment file
            // // Default to 'development' env
            // // Note that this file is _optional_
            // .add_source(
            //     File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode))
            //         .required(false),
            // )
            // // Add in a local configuration file
            // // This file shouldn't be checked in to git
            // .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
            // // Add in settings from the environment (with a prefix of APP)
            // // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            // .add_source(Environment::with_prefix("app"))
            // // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            // .build()?;

        // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("database.url"));

        s.try_into()
        // You can deserialize (and thus freeze) the entire configuration as
        // s.try_deserialize()

        //     .merge(Toml::file("Cargo.toml"))
        //     // .merge(Env::prefixed("CARGO_"))
        //     // .merge(Env::raw().only(&["RUSTC", "RUSTDOC"]))
        //     .join(Json::file("Cargo.json"))
            // ;

        // Self{}
        // return f.extract().unwrap()/
    }
    
    // pub fn load(path: &PathBuf) -> Self {
    //     // println!("path: {}", path.display());
    //     // let config: Self = Figment::new()
    //     let mut f = Figment::new();
    //     // Figment::from(Serialized::defaults(Config::default()));

    //     println!("{:#?}", path.extension());

    //     f = match path.extension().and_then(OsStr::to_str) {
    //         Some("yaml" | "yml") => f.merge(Yaml::file(path)),
    //         Some("toml") => f.merge(Toml::file(&path)),
    //         Some("json") => f.merge(Json::file(&path)),
    //         _ => f,
    //     };
    //     //     .merge(Toml::file("Cargo.toml"))
    //     //     // .merge(Env::prefixed("CARGO_"))
    //     //     // .merge(Env::raw().only(&["RUSTC", "RUSTDOC"]))
    //     //     .join(Json::file("Cargo.json"))
    //         // ;

    //     // Self{}
    //     return f.extract().unwrap()
    // }
}
