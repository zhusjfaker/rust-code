#[cfg(test)]
mod tests {
    extern crate ratel;

    use ratel::parser;

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

    }
}
