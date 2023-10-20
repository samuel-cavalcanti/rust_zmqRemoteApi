use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    let project_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let sim_const_file_name = "sim_const.rs";
    let sim_api_file_name = "sim_api.rs";
    let sim_ik_api_file_name = "sim_ik_api.rs";

    let sim_dir = Path::new(&project_dir)
        .join("src")
        .join("remote_api_objects")
        .join("sim");

    let assets = Path::new(&project_dir).join("c_transpiler").join("assets");

    let files = [sim_ik_api_file_name, sim_const_file_name, sim_api_file_name];

    for file in files {
        let origin = assets.join(file);
        let dest = sim_dir.join(file);

        println!("cargo:rerun-if-changed={}", origin.to_string_lossy());

        let content =
            fs::read_to_string(origin.clone()).unwrap_or_else(|_| panic!("read: {origin:?}"));

        fs::write(dest.clone(), content).unwrap_or_else(|_| panic!("write: {dest:?}"));
    }

    println!("cargo:rerun-if-changed=build.rs");
}
