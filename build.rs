fn main() {
    let bindings = bindgen::Builder::default()
        .header("/usr/local/include/librdkafka/rdkafka.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    println!("cargo:rustc-link-lib=rdkafka");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Could not write bindings!")
}
