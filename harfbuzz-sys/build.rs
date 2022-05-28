#[cfg(feature = "bindgen")]
extern crate bindgen_;
#[cfg(feature = "vendored")]
extern crate cc;
extern crate pkg_config;

use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn targetted_env_var(var_base: &str) -> Option<String> {
    match (env::var("TARGET"), env::var("HOST")) {
        (Ok(target), Ok(host)) => {
            let kind = if host == target { "HOST" } else { "TARGET" };
            let target_u = target.replace("-", "_");

            env::var(&format!("{}_{}", var_base, target))
                .or_else(|_| env::var(&format!("{}_{}", var_base, target_u)))
                .or_else(|_| env::var(&format!("{}_{}", kind, var_base)))
                .or_else(|_| env::var(var_base))
                .ok()
        }
        (Err(env::VarError::NotPresent), _) | (_, Err(env::VarError::NotPresent)) => {
            env::var(var_base).ok()
        }
        (Err(env::VarError::NotUnicode(s)), _) | (_, Err(env::VarError::NotUnicode(s))) => {
            panic!(
                "HOST or TARGET environment variable is not valid unicode: {:?}",
                s
            )
        }
    }
}

#[cfg(feature = "bindgen")]
mod bindings {
    use std::env;
    use std::fs;
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};

    static HEADER: &'static str = r#"#include "hb.h"
#include "hb-ot.h"
#include "hb-aat.h""#;

    struct BindingsWriter {
        file: fs::File,
        ignore: regex::Regex,
        replacer: regex::Regex,
    }
    impl BindingsWriter {
        fn new(path: &Path) -> BindingsWriter {
            BindingsWriter {
                file: fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path)
                    .unwrap(),
                ignore: regex::Regex::new("__u?int(8|16|32)_t ").unwrap(),
                replacer: regex::Regex::new("(i|u)(8|16|32)_").unwrap(),
            }
        }
    }

    impl Write for BindingsWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let c = unsafe { std::str::from_utf8_unchecked(buf) };
            for line in c.lines() {
                if !self.ignore.is_match(&line) {
                    let rep = self.replacer.replacen(&line, 0, "${1}${2}");
                    self.file.write(rep.as_bytes())?;
                    self.file.write("\n".as_bytes())?;
                }
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.file.flush()
        }
    }

    pub(super) fn gen(include_dirs: &[PathBuf]) {
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        let mut builder = bindgen_::builder()
            .no_convert_floats()
            .prepend_enum_name(false)
            .allowlist_type("hb_.*")
            .allowlist_function("hb_.*")
            .header_contents("wrapper.h", HEADER);

        for include_dir in include_dirs {
            builder = builder.clang_arg(format!("-I{}", include_dir.to_str().unwrap()));
        }

        let writer = BindingsWriter::new(&out_dir.join("bindings.rs"));
        let bindings = builder.generate().unwrap();
        bindings.write(Box::new(writer)).unwrap();
        println!("cargo:bindings={}", out_dir.join("bindings.rs").display());
    }
}

#[cfg(feature = "vendored")]
fn crossfile<P: AsRef<Path>>(out_dir: P) -> PathBuf {
    use std::ffi::OsString;

    let mut cfg = cc::Build::new();
    let cc = cfg.cpp(false).warnings(false).get_compiler();
    let cxx = cfg
        .cpp(true)
        .flag_if_supported("-std=c++11") // for unix
        .get_compiler();

    let cpu_family = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let system = if cfg!(target_vendor = "apple") {
        "darwin".to_string()
    } else if cfg!(all(windows, target_env = "gnu")) {
        "cygwin".to_string()
    } else {
        env::var("CARGO_CFG_TARGET_OS").unwrap()
    };
    let endian = env::var("CARGO_CFG_TARGET_ENDIAN").unwrap();

    let file = out_dir.as_ref().join("crossfile");
    println!("crossfile = {}", &file.display());

    let mut cross = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .truncate(true)
        .create(true)
        .open(&file)
        .unwrap();

    writeln!(cross, "[binaries]").unwrap();
    writeln!(cross, "c = '{}'", cc.path().display()).unwrap();
    writeln!(cross, "cpp = '{}'", cxx.path().display()).unwrap();

    if let Some(pkgconfig) = targetted_env_var("PKG_CONFIG") {
        writeln!(cross, "pkgconfig = '{}'", pkgconfig).unwrap();
    }

    writeln!(cross, "[host_machine]").unwrap();
    writeln!(cross, "system = '{}'", system).unwrap();
    writeln!(cross, "cpu = '{}'", cpu_family).unwrap();
    writeln!(cross, "cpu_family = '{}'", cpu_family).unwrap();
    writeln!(cross, "endian = '{}'", endian).unwrap();

    writeln!(cross, "[built-in options]").unwrap();
    let mut c_args = OsString::new();
    for (i, arg) in cc.args().iter().enumerate() {
        if i > 0 {
            c_args.push(",");
        }
        c_args.push("'");
        c_args.push(arg);
        c_args.push("'");
    }
    let mut cxx_args = OsString::new();
    for (i, arg) in cxx.args().iter().enumerate() {
        if i > 0 {
            cxx_args.push(",");
        }
        cxx_args.push("'");
        cxx_args.push(arg);
        cxx_args.push("'");
    }
    writeln!(cross, "c_args = [{}]", c_args.to_str().unwrap()).unwrap();
    writeln!(cross, "cpp_args = [{}]", cxx_args.to_str().unwrap()).unwrap();

    writeln!(cross, "[properties]").unwrap();
    if let Some(sysroot) = targetted_env_var("SYSROOT") {
        writeln!(cross, "sys_root = '{}'", sysroot).unwrap();
    }

    file
}

#[cfg(feature = "vendored")]
fn failed(output: &std::process::Output) {
    match std::str::from_utf8(&output.stdout) {
        Ok(s) => {
            println!("{}", s);
        }
        Err(_) => {
            println!("{:?}", &output.stdout);
        }
    }
    match std::str::from_utf8(&output.stderr) {
        Ok(s) => {
            println!("{}", s);
        }
        Err(_) => {
            println!("{:?}", &output.stdout);
        }
    }
    panic!("build harfbuzz failed");
}

#[cfg(feature = "vendored")]
fn vendored() {
    let target = env::var("TARGET").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);

    let pkg_config_path = format!("PKG_CONFIG_PATH_{}", &target);

    let build_dir = out_dir.join("build");
    let mut meson = Command::new("meson");
    meson
        .arg("setup")
        .arg(&build_dir)
        .arg(env::current_dir().unwrap().join("harfbuzz"))
        .arg("--prefix")
        .arg(&out_dir)
        .arg("--libdir")
        .arg("lib")
        .arg("--default-library")
        .arg("static")
        .arg("--buildtype")
        .arg("custom"); // that will read from CXXFLAGS

    if target != env::var("HOST").unwrap() {
        meson.arg("--cross-file").arg(&crossfile(&out_dir));
    }

    meson.arg("-Dglib=disabled");
    meson.arg("-Dgobject=disabled");

    #[cfg(target_vendor = "apple")]
    meson.arg("-Dcoretext=enabled");

    // #[cfg(windows)] {
    //     meson.arg("-Ddirectwrite=enabled");
    //     meson.arg("-Ddirectwrite=enabled");
    // }

    meson.arg("-Dtests=disabled");

    #[cfg(any(target_os = "android", all(unix, not(target_vendor = "apple"))))]
    meson.arg("-Dfreetype=enabled");

    meson.arg("-Dchafa=disabled");
    meson.arg("-Dcairo=disabled");

    #[cfg(feature = "graphite2")]
    meson.arg("-Dgraphite=enabled");

    // #[cfg(feature = "icu")]
    meson.arg("-Dicu=disabled");
    meson.arg("-Dicu_builtin=true");

    if let Ok(freetype_dir) = env::var("DEP_FREETYPE_ROOT") {
        let libdir = targetted_env_var("PKG_CONFIG_PATH").unwrap_or_default();
        let mut libdir: Vec<_> = env::split_paths(&libdir).collect();
        libdir.push(PathBuf::from(freetype_dir).join("lib").join("pkgconfig"));
        let libdir = env::join_paths(&libdir).unwrap_or_default();
        meson.env("PKG_CONFIG_PATH", &libdir);
    }

    let cfgresult = meson.output().unwrap();
    if !cfgresult.status.success() {
        failed(&cfgresult);
    }

    let mut meson = Command::new("meson");
    let output = meson
        .args(&["compile", "-C"])
        .arg(&build_dir)
        .output()
        .unwrap();
    if !output.status.success() {
        failed(&output);
    }

    let mut meson = Command::new("meson");
    let output = meson
        .args(&["install", "-C"])
        .arg(&build_dir)
        .output()
        .unwrap();
    if !output.status.success() {
        failed(&output);
    }

    if target != env::var("HOST").unwrap() {
        env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
    }
    env::set_var(&pkg_config_path, out_dir.join("lib").join("pkgconfig"));
}

#[allow(dead_code)]
/// System libraries should only be linked dynamically
fn is_static_available(name: &str, dir: &Path) -> bool {
    let libname = format!("lib{}.a", name);
    dir.join(&libname).exists()
}

fn probe() -> pkg_config::Library {
    let mut config = pkg_config::Config::new();
    config.statik(true);
    // #[cfg(all(not(feature = "bindgen"), not(feature = "vendored")))]
    #[cfg(not(feature = "bindgen"))]
    config.range_version("4.3".."5.0");
    config.probe("harfbuzz").unwrap()
}

fn main() {
    #[cfg(feature = "vendored")]
    vendored();

    #[allow(unused_variables)]
    let library = probe();

    #[cfg(feature = "bindgen")]
    bindings::gen(&library.include_paths);

    for include_dir in &library.include_paths {
        if !include_dir.ends_with("harfbuzz") {
            continue;
        }
        println!("cargo:include={}", include_dir.display());
        break;
    }
}
