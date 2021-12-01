#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::stderr;
    use swc::Compiler;
    use swc::config::SourceMapsConfig;
    use swc::ecmascript::ast::{EsVersion, ImportDecl, ImportNamedSpecifier, ImportStarAsSpecifier, ModuleDecl};
    use swc_common::sync::Lrc;
    use swc_common::{DUMMY_SP, errors::{ColorConfig, Handler}, FileName, FilePathMapping, SourceMap};
    use swc_ecma_parser::{EsConfig, lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
    use swc::ecmascript::ast::{Decl, Module, ModuleItem, Pat, Stmt};
    use swc_atoms::{js_word, JsWord};
    use swc_ecma_visit::FoldWith;
    use swc_ecma_ast::{ImportSpecifier, Str};
    use swc_ecma_transforms::const_modules;
    use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith};
    use swc_bundler::Bundler;

    #[test]
    fn mut_change() {
        let mut list = vec![1, 2, 3, 4];
        let mut temp_list: Vec<i32> = vec![];
        for num in list.iter_mut() {
            let c = (*num) % 2;
            if c == 0 {
                *num = *num * 4;
                temp_list.push(*num * 4);
            }
        }
        list.extend(temp_list);
        let str = list.iter().map(|x| { return x.to_string(); }).collect::<Vec<std::string::String>>().join("\n");
        println!("{}", str);
    }

    #[test]
    fn transform() {
        let source = "
import React from \"react\";
import ReactDOM from \"react-dom\";
import {Button, Input} from \"antd\";
import Child from \"./component/Child\";

class Page extends React.Component {
    render() {
        return (
            <div className={\"test\"}>
                <div>Page</div>
                <Child/>
                <Button>click me</Button>
                <Input/>
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById(\"root\"));
";
        let source2 = "\
        function abc(){\
            console.log(123);
        }";


        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.js".into()),
            source.into(),
        );

        let compiler = Compiler::new(cm.clone());
        let handler = Handler::with_emitter_writer(Box::new(stderr()), Some(compiler.cm.clone()));

        let lexer = Lexer::new(
            // We want to parse ecmascript
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                dynamic_import: true,
                dts: false,
                no_early_errors: false,
                import_assertions: false,
            }),
            // EsVersion defaults to es5
            EsVersion::Es2016,
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        let list_error = parser.take_errors();
        if list_error.iter().len() > 0 {
            let mut err_msg = "".to_owned();
            for err in list_error {
                let msg = err.into_kind().msg().to_string();
                err_msg.push_str(msg.as_str());
            }
        }

        let mut module = parser
            .parse_module().unwrap();

        println!("parser success");

        let s = serde_json::to_string_pretty(&module).expect("failed to serialize");
        println!("ast json is \n {}", s);

        let mut specifiers = vec![];

        for item in &mut module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
                let source = &*var.src.value;
                if source == "antd" {
                    for specifier in &var.specifiers {
                        match specifier {
                            ImportSpecifier::Named(ref s) => {
                                let ident = format!("{}", s.local.sym);
                                specifiers.push(format!("antd/es/{}/style/index.css", ident.to_lowercase()));
                            }
                            ImportSpecifier::Default(ref s) => {}
                            ImportSpecifier::Namespace(ref ns) => {}
                        }
                    }
                }
            }
        }

        for css_source in specifiers.clone() {
            let css_source_ref = css_source.as_str();
            let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![],
                src: Str {
                    span: DUMMY_SP,
                    value: JsWord::from(css_source_ref),
                    has_escape: false,
                    kind: Default::default(),
                },
                type_only: false,
                asserts: None,
            }));
            module.body.insert(0, dec);
        }

        let new_res = compiler.print(&module,
                                     None,
                                     None,
                                     false,
                                     EsVersion::Es2020,
                                     SourceMapsConfig::Bool(false),
                                     &Default::default(),
                                     None,
                                     false,
                                     None, ).unwrap();


        println!("gen success");
    }
}

