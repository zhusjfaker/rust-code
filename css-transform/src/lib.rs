#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::Path;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    ///
    /// 计算项目的全局目录
    /// # Arguments
    ///
    /// * `path`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn path_resolve(path: String) -> String {
        let work_cwd = env!("CARGO_MANIFEST_DIR");
        let os_work_cwd = OsString::from(work_cwd);
        return Path::new(&os_work_cwd)
            .join(path)
            .into_os_string()
            .into_string()
            .unwrap();
    }

    #[test]
    fn css_parser() {
        let lessfile_path = path_resolve("assets/index.less".to_string());
        let content = std::fs::read_to_string(lessfile_path.as_str()).unwrap();
        println!("content is {}", content);
        let css_content = sass_rs::compile_string(content.as_str(), Default::default()).unwrap();
        println!("content is {}", css_content);
    }
}
