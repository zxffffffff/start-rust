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
    // [1] vcpkg init
    let vcpkg_root = "./vcpkg";
    let vcpkg_exe: String;
    let vcpkg_sh: String;
    match std::env::consts::OS {
        "linux" => {
            warning!("当前系统是 Linux");
            vcpkg_exe = format!("{vcpkg_root}/vcpkg");
            vcpkg_sh = format!("{vcpkg_root}/bootstrap-vcpkg.sh");
        }
        "macos" => {
            warning!("当前系统是 macOS");
            vcpkg_exe = format!("{vcpkg_root}/vcpkg");
            vcpkg_sh = format!("{vcpkg_root}/bootstrap-vcpkg.sh");
        }
        "windows" => {
            warning!("当前系统是 Windows");
            vcpkg_exe = format!("{vcpkg_root}/vcpkg.exe");
            vcpkg_sh = format!("{vcpkg_root}/bootstrap-vcpkg.bat");
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
                .expect(&format!("Failed to run {vcpkg_sh}"));
            if !status.success() {
                panic!("Failed to init vcpkg");
            }
        }
    }

    // [2] cmake build vcpkg
    let build_dir = "./build";
    let build_dir_path = std::path::Path::new(&build_dir);
    let build_cache = format!("{build_dir}/CMakeCache.txt");
    let build_cache_path = std::path::Path::new(&build_cache);
    let build_mode = if cfg!(debug_assertions) {
        "Debug"
    } else {
        "Release"
    };
    warning!("vcpkg 依赖需要下载和编译，请确保网络访问正常...");
    fn cmake_build() {
        let status = Command::new("cmake")
            .arg("--build")
            .arg("build")
            .status()
            .expect("Failed to run cmake command");
        if !status.success() {
            panic!("Failed to run cmake build");
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
                .expect("Failed to run cmake command");
            if !status.success() {
                panic!("Failed to run cmake");
            }
            cmake_build();
        }
    }

    // [3] bindgen
    warning!("bindgen 正在生成头文件...");
    let lib_dir = "./build/vcpkg_installed/arm64-osx/debug/lib"; // todo debug
    let lib_dir_path = std::path::Path::new(lib_dir).to_str().unwrap();
    let head_dir = "./build/vcpkg_installed/arm64-osx/include";
    let zlib_lib = "z"; // libz
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
        .expect("Failed to generate bindings");

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
        .expect("Failed to open zlib_rs_code");
    zlib_rs_file
        .write_all(zlib_rs_code.as_bytes())
        .expect("Failed to write zlib_rs_code");
}
