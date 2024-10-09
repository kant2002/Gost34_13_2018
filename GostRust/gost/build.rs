use serde::{Deserialize, Serialize};
use simple_error::simple_error;
use std::{env, path::PathBuf, process::ExitCode};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default = "default_lib_clang_path")]
    lib_clang_path: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            lib_clang_path: default_lib_clang_path(),
        }
    }

    pub fn load<S: AsRef<std::path::Path>>(cp: S) -> Result<Self, Box<dyn std::error::Error>> {
        let config = std::fs::read_to_string(cp.as_ref()).map_err(|e| {
            simple_error!(
                "can't read config from `{}`. err: {}",
                cp.as_ref().to_string_lossy(),
                e
            )
        })?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }
}

pub fn config_file_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    match env_var("CARGO_MANIFEST_DIR") {
        Ok(s) => Ok(std::path::Path::new(&s).join(".vscode").join("build.toml")),
        Err(e) => simple_error::bail!("[build] could not determine repository root: {}", e),
    }
}

fn config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = config_file_path()?;
    if config_path.exists() {
        Config::load(config_path)
    } else {
        Ok(Config::new())
    }
}

fn default_lib_clang_path() -> String {
    // https://download.qt.io/development_releases/prebuilt/libclang/qt/?C=M;O=D
    let p = vec![
        "C:/Users/alex3/bin/libclang/libclang-release_16.0.0-based-windows-vs2019_64/libclang/bin",
        "d:/bin/clang+llvm-18.1.8-x86_64-pc-windows-msvc/bin",
    ];
    let pp = *p
        .iter()
        .find(|x| {
            let px = std::path::PathBuf::from(x);
            px.exists()
        })
        .unwrap_or(&"");
    pp.to_string()
}

fn main() -> ExitCode {
    if let Err(e) = build() {
        eprintln!("Error: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn env_var(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    match env::var(name) {
        Ok(val) => Ok(val),
        Err(e) => simple_error::bail!("can't get env var `{}`. {}", name, e),
    }
}

fn libgostcc() -> Result<(), Box<dyn std::error::Error>> {
    let target = env_var("TARGET")?;

    let mut build_pp = cc::Build::new();
    build_pp
        .file("./csrc/_shim.cpp")
        .file("./csrc/PRNG.cpp")
        .file("./csrc/Qalqan.cpp")
        .file("./csrc/TestVectors.cpp")
        .include("./csrc");

    if target.contains("linux") {
        build_pp.compile("libgostcc.a")
    } else {
        build_pp.compile("gostcc")
    }
    Ok(())
}

fn bindgen() -> Result<(), Box<dyn std::error::Error>> {
    let out = &PathBuf::from(env_var("OUT_DIR")?);

    let conf = config()?;
    std::env::set_var("LIBCLANG_PATH", conf.lib_clang_path);

    let bindings = bindgen::Builder::default()
        .header("./csrc/_shim.hpp")
        .clang_args(["-x", "c++", "-std=c++17"])
        .clang_arg("-I./c-src")
        .ctypes_prefix("cty")
        .generate()
        .map_err(|e| simple_error::simple_error!("can't generate bindings. {}", e))?;

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    libgostcc()?;
    bindgen()?;
    Ok(())
}
