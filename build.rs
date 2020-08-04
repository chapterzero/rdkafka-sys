use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    let ver = env::var("CARGO_PKG_VERSION").unwrap();
    let out_dir = format!("{}/rdkafka", env::var("OUT_DIR").unwrap());

    // clone and compile librdkafka
    let clone_dir = format!("{}/temp", out_dir);
    let archive = clone_librdkafka(&clone_dir, &ver);
    let libdir = unpack_archive(&archive, &clone_dir, &ver);
    compile_librdkafka(&libdir, &out_dir);
    copy_librdkafka(&out_dir);

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
        .args(&["-L", "--create-dirs", "-o", &target_file, &url])
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

fn copy_librdkafka(out_dir: &str) -> String {
    let copy_to = format!("{}/../../../..", out_dir);
    let librdkafka_file = format!("{}/lib/librdkafka.so.1", out_dir);
    eprintln!("Copying librdkafka {} to {}", librdkafka_file, copy_to);

    let res = Command::new("cp")
        .args(&[&librdkafka_file, &copy_to])
        .output()
        .expect("Error when copying librdkafka");

    if !res.status.success() {
        panic!(
            "Copy Got non 0 status code: {:?}, {}",
            res.status.code(),
            std::str::from_utf8(&res.stderr).unwrap()
        );
    }

    return copy_to
}
