extern crate bindgen;

use std::collections::HashSet;
use std::ffi::OsString;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug, Clone)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

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

/*
 * 生成定义文件
 */
fn generate(headerpath: Vec<&str>, outpath: &str, include_path: Option<Vec<&str>>) {
    let mut bindings = bindgen::Builder::default()
        .enable_cxx_namespaces()
        .generate_inline_functions(true)
        .derive_default(true);

    bindings = bindings.clang_arg("-arch").clang_arg("arm64");

    for header in headerpath.iter() {
        bindings = bindings.header(header.to_string());
    }

    let ignored_macros = IgnoreMacros(HashSet::from_iter(vec![
        String::from("FP_INFINITE"),
        String::from("FP_NAN"),
        String::from("FP_NORMAL"),
        String::from("FP_SUBNORMAL"),
        String::from("FP_ZERO"),
        String::from("IPPORT_RESERVED"),
    ]));

    if !include_path.is_none() {
        let include_path_list = include_path.unwrap();
        for path in include_path_list.iter() {
            bindings = bindings.clang_arg("-I").clang_arg(path.to_string());
        }
    }

    let res = bindings
        .parse_callbacks(Box::new(ignored_macros.clone()))
        .layout_tests(false)
        .rustfmt_bindings(true)
        .detect_include_paths(true)
        .generate_comments(true)
        .generate()
        .unwrap();

    res.write_to_file(outpath).unwrap();
}

/*
 * 构建项目方法
 */
fn build() {
    /** 生成 *.rs 定义文件 */
    let value_headerpaths = calc_project_path(vec!["clib/index.h"].into_iter().map(|s| s.to_owned()).collect());
    let headerpaths = value_headerpaths.iter().map(|x| x.deref()).collect::<Vec<&str>>();
    let value_includepaths = calc_project_path(vec!["clib"].into_iter()
        .map(|s| s.to_owned())
        .collect());
    let includepaths = value_includepaths.iter().map(|x| x.deref()).collect::<Vec<&str>>();
    generate(headerpaths, path_resolve(String::from("src/clib.rs")).as_str(), Some(includepaths));
    /*
     * 编译携带 静态库过程
     */
    let include_static_paths = vec!["clib"];
    build_include_path(calc_project_path(
        include_static_paths
            .into_iter()
            .map(|s| s.to_owned())
            .collect(),
    ));
    let include_static_libs = vec!["index"];
    build_include_dll(include_static_libs.into_iter().map(|s| s.to_owned()).collect());
    println!("build successful!")
}

fn main() {
    build();
}
