use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

macro_rules! warning {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let is_debug_mode = cfg!(debug_assertions);
    let build_mode = if is_debug_mode { "Debug" } else { "Release" };

    // [1] vcpkg init
    let vcpkg_root = "./vcpkg";
    let vcpkg_exe: String;
    let vcpkg_sh: String;
    match std::env::consts::OS {
        "windows" => {
            warning!("当前系统是 Windows {build_mode}");
            vcpkg_exe = format!("{vcpkg_root}/vcpkg.exe");
            vcpkg_sh = format!("{vcpkg_root}/bootstrap-vcpkg.bat");
        }
        "macos" => {
            warning!("当前系统是 macOS {build_mode}");
            vcpkg_exe = format!("{vcpkg_root}/vcpkg");
            vcpkg_sh = format!("{vcpkg_root}/bootstrap-vcpkg.sh");
        }
        "linux" => {
            warning!("当前系统是 Linux {build_mode}");
            vcpkg_exe = format!("{vcpkg_root}/vcpkg");
            vcpkg_sh = format!("{vcpkg_root}/bootstrap-vcpkg.sh");
        }
        _ => panic!("未知操作系统"),
    }
    let vcpkg_exe_path = std::path::Path::new(&vcpkg_exe);
    let vcpkg_sh_path = std::path::Path::new(&vcpkg_sh);

    match fs::metadata(vcpkg_exe_path) {
        Ok(_metadata) => {}
        Err(_) => {
            warning!("vcpkg 首次初始化，请确保网络访问正常...");
            let status = Command::new(vcpkg_sh_path)
                .status()
                .expect(&format!("vcpkg 初始化失败：{vcpkg_sh}"));
            if !status.success() {
                panic!("vcpkg 初始化失败：{status}");
            }
        }
    }

    // [2] cmake build vcpkg
    let build_dir = "./build";
    let build_dir_path = std::path::Path::new(&build_dir);
    let build_cache = format!("{build_dir}/CMakeCache.txt");
    let build_cache_path = std::path::Path::new(&build_cache);
    warning!("cmake vcpkg 依赖需要下载和编译，请确保网络访问正常...");
    fn cmake_build() {
        let status = Command::new("cmake")
            .arg("--build")
            .arg("build")
            .status()
            .expect("cmake 编译失败");
        if !status.success() {
            panic!("cmake 编译失败：{status}");
        }
    }
    match fs::metadata(build_cache_path) {
        Ok(_metadata) => {
            cmake_build();
        }
        Err(_) => {
            let status = Command::new("cmake")
                .arg("--no-warn-unused-cli")
                .arg(format!("-DCMAKE_BUILD_TYPE:STRING={build_mode}"))
                .arg("-B")
                .arg(build_dir_path)
                .arg("-S")
                .arg(".")
                .status()
                .expect("cmake 初始化失败");
            if !status.success() {
                panic!("cmake 初始化失败：{status}");
            }
            cmake_build();
        }
    }

    // [3] bindgen
    warning!("bindgen 正在生成头文件...");
    let lib_dir;
    let head_dir;
    let zlib_lib;
    match std::env::consts::OS {
        "windows" => {
            if is_debug_mode {
                lib_dir = "./build/vcpkg_installed/x64-windows-static/debug/lib";
                zlib_lib = "zlibd"; // zlibd.lib
            } else {
                lib_dir = "./build/vcpkg_installed/x64-windows-static/lib";
                zlib_lib = "zlib"; // zlib.lib
            };
            head_dir = "./build/vcpkg_installed/x64-windows-static/include";
        }
        "macos" => {
            lib_dir = if is_debug_mode {
                "./build/vcpkg_installed/arm64-osx/debug/lib"
            } else {
                "./build/vcpkg_installed/arm64-osx/lib"
            };
            head_dir = "./build/vcpkg_installed/arm64-osx/include";
            zlib_lib = "z"; // libz.a
        }
        "linux" => {
            lib_dir = if is_debug_mode {
                "./build/vcpkg_installed/x64-linux/debug/lib"
            } else {
                "./build/vcpkg_installed/x64-linux/lib"
            };
            head_dir = "./build/vcpkg_installed/x64-linux/include";
            zlib_lib = "z"; // libz.a
        }
        _ => panic!("未知操作系统"),
    }
    let lib_dir_path = std::path::Path::new(lib_dir).to_str().unwrap();
    let zlib_head = "zlib.h";
    let zlib_head_path = format!("{head_dir}/{zlib_head}");
    let zlib_head_path = std::path::Path::new(&zlib_head_path).to_str().unwrap();
    println!("cargo:rustc-link-search={lib_dir_path}");
    println!("cargo:rustc-link-lib={zlib_lib}");
    println!("cargo:rerun-if-changed={zlib_head_path}");
    let bindings = bindgen::Builder::default()
        .header(zlib_head_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("bindgen zlib_head_path 生成失败");

    let bindgen_dir = "./src/bindgen";
    let bindgen_dir_path = PathBuf::from(bindgen_dir);
    let zlib_rs = "zlib.rs";
    let zlib_rs_path = bindgen_dir_path.join(zlib_rs);

    let mut zlib_rs_code = bindings.to_string();
    let code_start = "#![allow(warnings)]\r\n";
    zlib_rs_code.insert_str(0, code_start);

    let mut zlib_rs_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(zlib_rs_path)
        .expect("bindgen zlib_rs 打开失败");
    zlib_rs_file
        .write_all(zlib_rs_code.as_bytes())
        .expect("bindgen zlib_rs 写入失败");
}
