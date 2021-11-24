#[cfg(test)]
mod tests {
    extern crate swc_common;
    extern crate swc_ecma_parser;
    extern crate swc_ecma_visit;

    use std::any::Any;
    use std::borrow::BorrowMut;
    use std::io::stderr;
    use std::ops::Deref;
    use swc::Compiler;
    use swc::config::SourceMapsConfig;
    use swc::ecmascript::ast::{EsVersion, ImportDecl, ImportNamedSpecifier, ImportStarAsSpecifier, ModuleDecl};
    use swc_common::sync::Lrc;
    use swc_common::{
        errors::{ColorConfig, Handler},
        FileName, FilePathMapping, SourceMap,
    };
    use swc_common::comments::SingleThreadedComments;
    use swc_common::util::take::Take;
    use swc_ecma_parser::{EsConfig, lexer::Lexer, Parser, StringInput, Syntax};
    use swc_ecma_transforms::typescript;
    use swc_ecma_transforms_typescript::strip;
    use swc_ecma_visit::FoldWith;
    use swc_ecma_visit::{
        as_folder, noop_visit_mut_type, noop_visit_type, Fold, Node, Visit, VisitMut, VisitMutWith,
        VisitWith,
    };
    use swc::ecmascript::ast::{Decl, Module, ModuleItem, Pat, Stmt};
    use swc_atoms::{js_word, JsWord};
    use swc_ecma_ast::ImportSpecifier;


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
            Syntax::Es(EsConfig {
                jsx: true,
                num_sep: true,
                class_private_props: true,
                class_private_methods: false,
                class_props: true,
                fn_bind: true,
                decorators: true,
                decorators_before_export: true,
                export_default_from: true,
                export_namespace_from: false,
                dynamic_import: true,
                nullish_coalescing: true,
                optional_chaining: true,
                import_meta: false,
                top_level_await: false,
                import_assertions: false,
                static_blocks: true,
                private_in_object: true,
            }),
            // EsVersion defaults to es5
            EsVersion::Es2016,
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        for e in parser.take_errors() {
            println!("parser fail");
        }

        let module = parser
            .parse_module().unwrap();

        println!("parser success");

        let s = serde_json::to_string_pretty(&module).expect("failed to serialize");
        println!("ast json is \n {}", s);

        let mut specifiers = vec![];

        let oldmodule = module.clone();
        for item in module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
                let source = &*var.src.value;
                if source == "antd" {
                    for specifier in &var.specifiers {
                        match specifier {
                            ImportSpecifier::Named(ref s) => {
                                let d = format!("{}", s.local.sym);
                                specifiers.push(d);
                            }
                            ImportSpecifier::Default(ref s) => {}
                            ImportSpecifier::Namespace(ref ns) => {}
                        }
                    }
                }
            }
        }

        let res = compiler.print(&oldmodule,
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

