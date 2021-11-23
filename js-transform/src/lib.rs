#[cfg(test)]
mod tests {
    extern crate swc_common;
    extern crate swc_ecma_parser;

    use swc::ecmascript::ast::EsVersion;
    use swc_common::sync::Lrc;
    use swc_common::{
        errors::{ColorConfig, Handler},
        FileName, FilePathMapping, SourceMap,
    };
    use swc_ecma_parser::{EsConfig, lexer::Lexer, Parser, StringInput, Syntax};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
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
                private_in_object: true
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

        let _module = parser
            .parse_module().expect("parser fail");

        println!("parser success");
    }
}
