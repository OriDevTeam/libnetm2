// Standard Uses
use std::{path::PathBuf, env, fs};

// Crate Uses

// External Uses
extern crate prost_build;


const PROTOS_PATH: &str = "protos/";


fn main() {
    prost_build::compile_protos(&["src/protos/network/packets/login.proto"],
                                &["src/"]).unwrap();
}


pub fn generate() {
    let proto_path = PathBuf::from(PROTOS_PATH);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let out_path = out_dir.join("messages.rs");
    let module_path = src_dir.join("messages").join("generated.rs");

    prost_build::Config::new()
        .compile_protos(
            &[proto_path.to_str().unwrap()],
            &[proto_path.parent().unwrap().to_str().unwrap()],
        )
        .expect("Failed to compile protobuf definitions");

    fs::copy(out_path, module_path).unwrap();
}

