struct ParserInput {
  finished: bool,
  autoCommentAbsorb: bool,
  commentStore: Vec<std::string::String>,
  i: usize,
}

impl ParserInput {}

pub fn parser_input() {
  let CHARCODE_SPACE = 32;
  let CHARCODE_TAB = 9;
  let CHARCODE_LF = 10;
  let CHARCODE_CR = 13;
  let CHARCODE_PLUS = 43;
  let CHARCODE_COMMA = 44;
  let CHARCODE_FORWARD_SLASH = 47;
  let CHARCODE_9 = 57;
}