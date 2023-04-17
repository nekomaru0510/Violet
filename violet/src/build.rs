//extern crate cc;
use std::error::Error;
//use cc::Build;

use std::fs;
use std::fs::File;
use std::io::Write;
/*
use toml;
use toml::Value;
*/
fn main() -> Result<(), Box<dyn Error>> {
    
    println!("build");
    let contents = fs::read_to_string("../config/setting/setting.toml").expect("Failed to read file");
    println!("{:?}", contents);
    //let value = contents.parse::<Value>().unwrap();
    /*
    let a = fs::read_to_string("../config/setting/setting.toml");
    match a {
        None => println!("None"),
        _ => println!("exist"),
    }
    */
    //println!("env: {:?}", value["env"].as_str().unwrap());

    let header = "//!setting.rs \n// Don't edit this file \n";

    // 生成したコードをファイルに書き込む
    let mut file = File::create("src/setting.rs").unwrap();
    file.write_all(header.as_bytes()).unwrap();
    
    Ok(())
}
