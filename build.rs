use dotenv::{dotenv, var};
use std::{fs::File, io::Write};
use wlrs_auth::hash_password;

fn pub_const_fmt(key: &str, value: &str) -> String {
    format!(
        "pub const {}: &str = \"{}\";\n",
        key,
        value.replace('"', "\\\"")
    )
}

fn get_var_fmt(key: &str) -> String {
    let value = var(key).unwrap();
    pub_const_fmt(key, &value)
}

fn main() {
    println!("cargo:rerun-if-changed=.env");
    dotenv().ok();

    let mut server = File::create("wlrs-server/src/env.rs").unwrap();
    let mut wasm = File::create("wlrs-wasm/src/env.rs").unwrap();

    let server_addr = get_var_fmt("SERVER_ADDR");
    let rcon_pass = get_var_fmt("RCON_PASS");
    let websocket_addr = get_var_fmt("WEBSOCKET_ADDR");

    let server_pass_key = "SERVER_PASS";
    let server_pass_value = var(server_pass_key).unwrap();

    let (salt_value, hash_value) = hash_password(server_pass_value.as_bytes()).unwrap();
    let salt_key = format!("{}_SALT", server_pass_key);
    let hash_key = format!("{}_HASH", server_pass_key);

    server.write_all(server_addr.as_bytes()).unwrap();
    server.write_all(rcon_pass.as_bytes()).unwrap();

    server
        .write_all(pub_const_fmt(&salt_key, &salt_value).as_bytes())
        .unwrap();
    server
        .write_all(pub_const_fmt(&hash_key, &hash_value).as_bytes())
        .unwrap();

    wasm.write_all(websocket_addr.as_bytes()).unwrap();
}
