fn main() {
    let mut args = std::env::args();
    args.next();
    let out_dir = args.next()
        .expect("Require 2nd argument: rdkafka lib ");
    // generate bindings
    let bindings = bindgen::Builder::default()
        .header(&format!("{}/include/librdkafka/rdkafka.h", out_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Could not write bindings!");
}
