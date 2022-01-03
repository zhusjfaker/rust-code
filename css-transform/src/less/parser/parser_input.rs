struct SaveStack {
  current: String,
  i: usize,
  j: usize,
}

struct ParserInput<'a> {
  finished: bool,
  autoCommentAbsorb: bool,
  commentStore: Vec<std::string::String>,
  i: usize,
}

impl ParserInput<'static> {
  fn save(&mut self) {}
}

pub fn parser_input() -> ParserInput<'static> {
  let CHARCODE_SPACE = 32;
  let CHARCODE_TAB = 9;
  let CHARCODE_LF = 10;
  let CHARCODE_CR = 13;
  let CHARCODE_PLUS = 43;
  let CHARCODE_COMMA = 44;
  let CHARCODE_FORWARD_SLASH = 47;
  let CHARCODE_9 = 57;

  let input: &str;
  let j: usize;
  let current: &str;
  let mut currentPos: usize = 0;

  let parserInput = ParserInput {
    finished: false,
    autoCommentAbsorb: true,
    commentStore: vec![],
    i: 0,
  };

  let skipWhitespace = |length: usize| {};


  parserInput
}