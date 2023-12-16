use std::fs;
use std::process::Command;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    // [1] vcpkg init
    const VCPKG_ROOT: &str = "./vcpkg";
    let vcpkg_exe: String;
    let vcpkg_sh: String;
    match std::env::consts::OS {
        "linux" => {
            p!("当前系统是 Linux");
            vcpkg_exe = format!("{VCPKG_ROOT}/vcpkg");
            vcpkg_sh = format!("{VCPKG_ROOT}/bootstrap-vcpkg.sh");
        }
        "macos" => {
            p!("当前系统是 macOS");
            vcpkg_exe = format!("{VCPKG_ROOT}/vcpkg");
            vcpkg_sh = format!("{VCPKG_ROOT}/bootstrap-vcpkg.sh");
        }
        "windows" => {
            p!("当前系统是 Windows");
            vcpkg_exe = format!("{VCPKG_ROOT}/vcpkg.exe");
            vcpkg_sh = format!("{VCPKG_ROOT}/bootstrap-vcpkg.bat");
        }
        _ => panic!("未知操作系统"),
    }
    let vcpkg_exe_path = std::path::Path::new(&vcpkg_exe);
    let vcpkg_sh_path = std::path::Path::new(&vcpkg_sh);

    match fs::metadata(vcpkg_exe_path) {
        Ok(_metadata) => {}
        Err(_) => {
            p!("vcpkg 首次初始化，请确保网络访问正常...");
            let status = Command::new(vcpkg_sh_path)
                .status()
                .expect(&format!("Failed to run {vcpkg_sh}"));
            if !status.success() {
                panic!("Failed to init vcpkg");
            }
        }
    }

    // [2] cmake build
    const CMAKE_BUILD: &str = "./build";
    let build = format!("{CMAKE_BUILD}");
    let build_cache = format!("{CMAKE_BUILD}/CMakeCache.txt");
    let build_cache_path = std::path::Path::new(&build_cache);
    let build_mode = if cfg!(debug_assertions) {
        "Debug"
    } else {
        "Release"
    };
    p!("vcpkg 依赖需要下载和编译，请确保网络访问正常...");
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
                .arg(build)
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
}
