use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    let ver = env::var("CARGO_PKG_VERSION").unwrap();
    let out_dir = format!("{}/rdkafka", env::var("OUT_DIR").unwrap());

    // clone and compile librdkafka
    let archive = clone_librdkafka("temp", &ver);
    let libdir = unpack_archive(&archive, "temp", &ver);
    compile_librdkafka(&libdir, &out_dir);

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

    // linking
    println!("cargo:rustc-link-search={}/lib", out_dir);
    println!("cargo:rustc-link-lib=rdkafka");
}

#[cfg(feature = "gssapi")]
const GSSAPI_FLAG: &'static str = "--enable-gssapi";
#[cfg(not(feature = "gssapi"))]
const GSSAPI_FLAG: &'static str = "--disable-gssapi";

#[cfg(feature = "ssl")]
const SSL_FLAG: &'static str = "--enable-ssl";
#[cfg(not(feature = "ssl"))]
const SSL_FLAG: &'static str = "--disable-ssl";

#[cfg(feature = "zstd")]
const ZSTD_FLAG: &'static str = "--enable-zstd";
#[cfg(not(feature = "zstd"))]
const ZSTD_FLAG: &'static str = "--disable-zstd";

#[cfg(feature = "lz4")]
const LZ4_FLAG: &'static str = "--enable-lz4-ext";
#[cfg(not(feature = "lz4"))]
const LZ4_FLAG: &'static str = "--disable-lz4-ext";

fn get_librdkafka_configure_flag() -> Vec<&'static str> {
    vec![GSSAPI_FLAG, SSL_FLAG, ZSTD_FLAG, LZ4_FLAG]
}

use std::path::Path;
use std::process::Command;
fn clone_librdkafka(dir: &str, ver: &str) -> String {
    let target_file = format!("{}/librdkafka.zip", dir);
    if Path::new(&target_file).exists() {
        return target_file;
    }

    let url = format!(
        "https://github.com/edenhill/librdkafka/archive/v{}.zip",
        ver
    );
    eprintln!("Downloading librdkafka ver {}: {}", ver, url);
    Command::new("curl")
        .args(&["-L", "-o", &target_file, &url])
        .output()
        .expect("Error downloading librdkafka");
    return target_file;
}

fn unpack_archive(a: &str, dir: &str, ver: &str) -> String {
    eprintln!("Unpacking {}", a);
    Command::new("unzip")
        .args(&[a, "-d", dir])
        .output()
        .expect("Error when unpacking");
    return format!("{}/librdkafka-{}", dir, ver);
}

fn compile_librdkafka(libdir: &str, targetdir: &str) {
    let prefix = format!("--prefix={}", targetdir);
    let mut flags = vec![
        prefix.as_str(),
    ];
    let configure_flags = get_librdkafka_configure_flag();
    flags.extend(configure_flags);
    eprintln!(
        "Compiling librdkafka dir {}, with flags {:?}",
        libdir, flags
    );
    //configure
    let res = Command::new("./configure")
        .current_dir(libdir)
        .args(flags)
        .output()
        .expect("Error when configure");
    if !res.status.success() {
        panic!(
            "Configure Got non 0 status code: {:?}, {}",
            res.status.code(),
            std::str::from_utf8(&res.stderr).unwrap()
        );
    }

    // make
    let res = Command::new("make")
        .current_dir(libdir)
        .output()
        .expect("Error when make");
    if !res.status.success() {
        panic!(
            "Make Got non 0 status code: {:?}, {}",
            res.status.code(),
            std::str::from_utf8(&res.stderr).unwrap()
        );
    }

    // make install
    let res = Command::new("make")
        .current_dir(libdir)
        .args(&["install"])
        .output()
        .expect("Error when make install");
    if !res.status.success() {
        panic!(
            "Make install Got non 0 status code: {:?}, {}",
            res.status.code(),
            std::str::from_utf8(&res.stderr).unwrap()
        );
    }
}
