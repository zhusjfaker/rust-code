#![feature(allocator_api)]
#![feature(get_mut_unchecked)]
#![feature(new_uninit)]

use std::string::String;

mod less;


#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::Path;
    use std::rc::Rc;

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

    /// [测试] 返回 str 某一个字符的 unicode 编码
    #[test]
    fn test_char() {
        fn charCodeAt(txt: &str, index: usize) -> u32 {
            let charlist: Vec<char> = txt.chars().collect();
            let val = charlist.get(index).unwrap();
            return *val as u32;
        }
        let char_val = charCodeAt("abc", 0);
        println!("char is {}", char_val);
    }

    #[test]
    fn pointer() {
        let mut c = Rc::new(5);
        *Rc::get_mut(&mut c).unwrap() = 4;
        println!("num is {}", *c);
        assert_eq!(*c, 4);
        let mut y = Rc::clone(&c);
        // *Rc::make_mut(&mut y) = 6;
        *Rc::make_mut(&mut c) = 5;
        println!("....");
    }
}
