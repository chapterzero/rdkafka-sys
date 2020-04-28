fn main() {
    let bindings = bindgen::Builder::default()
        .header("/usr/local/include/librdkafka/rdkafka.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    println!("cargo:rustc-link-lib=rdkafka");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Could not write bindings!")
}
