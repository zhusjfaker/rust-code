struct SaveStack {
  current: String,
  i: usize,
  j: usize,
}

pub struct ParserInput {
  finished: bool,
  autoCommentAbsorb: bool,
  commentStore: Vec<std::string::String>,
  i: usize,

  //无意义计算引用
  current: Box<String>,
  currentPos: Box<usize>,
  saveStack: Box<Vec<SaveStack>>,
  furthest: Box<usize>,
  j: Box<usize>,
}

impl ParserInput {
  fn save(&mut self) {
    *self.currentPos = self.i;
    (*self.saveStack).push(SaveStack {
      current: (*self.current).to_string(),
      i: self.i,
      j: *self.j,
    });
  }
}

pub fn parser_input() -> ParserInput {
  let CHARCODE_SPACE = 32;
  let CHARCODE_TAB = 9;
  let CHARCODE_LF = 10;
  let CHARCODE_CR = 13;
  let CHARCODE_PLUS = 43;
  let CHARCODE_COMMA = 44;
  let CHARCODE_FORWARD_SLASH = 47;
  let CHARCODE_9 = 57;

  let parserInput = ParserInput {
    finished: false,
    autoCommentAbsorb: true,
    commentStore: vec![],
    i: 0,
    //无意义计算引用
    current: Box::new("".to_string()),
    currentPos: Box::new(0),
    saveStack: Box::new(vec![]),
    furthest: Box::new(0),
    j: Box::new(0),
  };

  let skipWhitespace = |length: usize| {};


  parserInput
}