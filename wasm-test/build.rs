use std::ffi::OsString;
use std::path::Path;

/**
 * 计算项目的全局目录
 */
fn path_resolve(path: String) -> String {
    let work_cwd = env!("CARGO_MANIFEST_DIR");
    let os_work_cwd = OsString::from(work_cwd);
    return Path::new(&os_work_cwd)
        .join(path)
        .into_os_string()
        .into_string()
        .unwrap();
}


/**
 * 批量返回所有 项目 文件里 的绝对路径
 */
fn calc_project_path(static_files: Vec<String>) -> Vec<String> {
    let mut final_paths: Vec<String> = std::vec::Vec::new();
    for static_path in static_files.iter() {
        let static_file = path_resolve(String::from(static_path));
        final_paths.push(static_file);
    }
    return final_paths;
}

/**
 * 批量增加 rust 构建中 查询静态库的 路径 include
 */
fn build_include_path(static_files: Vec<String>) {
    for static_path in static_files.iter() {
        println!("cargo:rustc-link-search=native={}", static_path);
    }
}

/**
 * 批量制定找寻的静态库 库名 libxx 则 -> xx 忽略前缀 lib
 */
fn build_include_dll(static_libs: Vec<String>) {
    for static_lib in static_libs.iter() {
        println!("cargo:rustc-link-lib=static={}", static_lib);
    }
}


fn build() {
    let include_static_paths = vec!["clib"];
    build_include_path(calc_project_path(
        include_static_paths
            .into_iter()
            .map(|s| s.to_owned())
            .collect(),
    ));
    let include_static_libs = vec!["clib/libindex.a"];
    build_include_dll(calc_project_path(
        include_static_libs
            .into_iter()
            .map(|s| s.to_owned())
            .collect(),
    ));
    println!("build successful!")
}

fn main() {
    build();
}
