extern crate bindgen;

pub mod gen;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::ffi::OsString;
    use std::iter::FromIterator;
    use std::ops::Deref;
    use std::path::Path;
    use std::process::Command;

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
     * 计算项目内部关于 ffmpeg 的静态库 文件
     */
    fn static_ffmpeg_lib() -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        let path = path_resolve("ff-output/lib".into());
        let mut files = std::fs::read_dir(path).unwrap();
        while let Some(file) = files.next() {
            let name = file.unwrap().file_name().into_string().unwrap();
            let extname = std::path::Path::new(&name)
                .extension()
                .and_then(std::ffi::OsStr::to_str);
            if extname.unwrap_or("") == "a" {
                list.push(name.replace(".a", "").replace("lib", ""));
            }
        }
        return list;
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
     * 批量制定找寻的静态库 库名 libxx 则 -> xx 忽略前缀 lib
     */
    fn build_include_dll(static_libs: Vec<String>) {
        for static_lib in static_libs.iter() {
            println!("cargo:rustc-link-lib=static={}", static_lib);
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn env() {
        for (key, val) in std::env::vars() {
            println!("{} is {} ....", key, val);
        }
    }

    /**
     * 生成 C 对应 FFI 的 Rust 头文件方法
     *
     */
    pub fn generate(headerpath: Vec<&str>, outpath: &str, include_path: Option<Vec<&str>>) {
        let mut bindings = bindgen::Builder::default()
            .enable_cxx_namespaces()
            .generate_inline_functions(true)
            .derive_default(true);

        // bindings = bindings.clang_arg("-arch").clang_arg("arm64");

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

    fn recursivefolder(ffoutput: &str, rsoutput: &str) {
        let ff_header_paths = path_resolve(ffoutput.into());

        fn calc(path: String) -> Vec<String> {
            let mut filelist: Vec<String> = Vec::new();
            let mut files = std::fs::read_dir(path.clone()).unwrap();
            while let Some(file) = files.next() {
                let dir_entry = file.unwrap();
                let filename: String = dir_entry.file_name().into_string().unwrap();
                let name = format!("{}/{}", path.clone(), filename);
                let abs_name = path_resolve(name);
                if dir_entry.file_type().unwrap().is_dir() {
                    let temp_list = calc(abs_name);
                    filelist = [filelist.as_slice(), temp_list.as_slice()].concat();
                } else {
                    filelist.push(abs_name);
                }
            }
            return filelist.clone();
        }

        fn filter(list: Vec<String>, blocklist: Vec<String>) -> Vec<String> {
            let mut filelist: Vec<String> = Vec::new();
            list.iter().for_each(|path| {
                let path_val = path.to_string();

                let filter_list = blocklist
                    .iter()
                    .filter(|block| {
                        let b_val = block.to_string();
                        return path_val.contains(&b_val);
                    })
                    .collect::<Vec<&String>>();

                if filter_list.len() == 0 {
                    filelist.push(path_val);
                } else {
                    // println!("命中屏蔽路径:{}...", path_val);
                }
            });
            return filelist;
        }

        let block_words = vec![
            "dxva2.h",
            "d3d11va.h",
            "qsv.h",
            "vdpau.h",
            "videotoolbox.h",
            "xvmc.h",
            "hwcontext_cuda.h",
            "hwcontext_d3d11va.h",
            "hwcontext_dxva2.h",
            "hwcontext_opencl.h",
            "hwcontext_qsv.h",
            "hwcontext_vaapi.h",
            "hwcontext_vdpau.h",
            "hwcontext_videotoolbox.h",
            "hwcontext_vulkan.h",
        ]
            .iter()
            .map(|i| {
                return i.to_string();
            })
            .collect::<Vec<String>>();

        let origin_list = calc(ff_header_paths);
        let list_ptr = filter(origin_list, block_words);
        let gen_path_str = path_resolve(rsoutput.to_string());
        let gen_path = std::path::Path::new(gen_path_str.as_str());

        if gen_path.exists() {
            std::fs::remove_file(gen_path).unwrap();
        }

        let header_path = path_resolve(ffoutput.to_string());
        let include_path = vec![header_path.as_str()];

        let input = list_ptr
            .iter()
            .map(|header| {
                println!(".....{}", header);
                return header.deref();
            })
            .collect::<Vec<&str>>();
        generate(input, &gen_path_str, Some(include_path.clone()));

        return;
    }

    #[test]
    fn build() {
        // 下载FFmpeg 内容
        let lib_path = path_resolve("source/FFmpeg".into());
        if Path::new(&lib_path).exists() == false {
            let git_url = "git@github.com:FFmpeg/FFmpeg.git";
            let workspace = "source";
            let cwd = path_resolve(workspace.into());
            let mut task_clone_ffmpeg_lib = Command::new("git");
            task_clone_ffmpeg_lib.arg("clone").arg(git_url);
            task_clone_ffmpeg_lib.current_dir(cwd.clone());
            task_clone_ffmpeg_lib
                .status()
                .expect(&format!("git clone {} --> failed", git_url));
        } else {
            let cwd = path_resolve(lib_path);
            let mut task_clone_ffmpeg_lib = Command::new("git");
            task_clone_ffmpeg_lib.arg("pull");
            task_clone_ffmpeg_lib.current_dir(cwd.clone());
            task_clone_ffmpeg_lib
                .status()
                .expect("source/FFmpeg 下执行 git pull 失败");
        }
        // 尝试编译 FFmpeg 根据当前 OS 和 arch
        let complier_lib_path = path_resolve(String::from("ff-output"));
        if !Path::new(&complier_lib_path).exists() {
            let mut task_complie_ffmpeg_lib = Command::new("sh");
            if cfg!(target_arch = "aarch64") {
                task_complie_ffmpeg_lib.arg("./build.arm.sh");
            } else {
                task_complie_ffmpeg_lib.arg("./build.sh");
            }
            let complie_cwd_path = "script";
            let complie_cwd = path_resolve(complie_cwd_path.into());
            task_complie_ffmpeg_lib.current_dir(complie_cwd.clone());
            task_complie_ffmpeg_lib
                .status()
                .expect("执行 ffmpeg 构建编译失败!");
        }
        // 尝试生成 rust 定义
        let rs_lib_dir = "src/gen";
        let rs_lib_file = format!("{}/ffmpeg.rs", rs_lib_dir.clone());
        let abs_rs_lib_dir = path_resolve(rs_lib_dir.to_string());
        if !Path::new(abs_rs_lib_dir.clone().as_str()).exists() {
            std::fs::create_dir_all(abs_rs_lib_dir.clone().as_str()).unwrap();
        }
        let ffoutput_path = format!("{}/include/", complier_lib_path);
        if !Path::new(&rs_lib_file.to_string()).exists() {
            recursivefolder(ffoutput_path.as_ref(), rs_lib_file.as_ref());
        }
        // 尝试 编译绑定生成的静态文件
        let include_static_paths = vec!["ff-output/lib"];
        build_include_path(calc_project_path(
            include_static_paths
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        ));
        let lib_name_list = static_ffmpeg_lib();
        build_include_dll(lib_name_list);
    }
}


