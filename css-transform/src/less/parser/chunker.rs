trait StringExtend {
  fn charCodeAt(&self, index: usize) -> Option<u32>;
}

impl StringExtend for &str {
  fn charCodeAt(&self, index: usize) -> Option<u32> {
    let charlist: Vec<char> = self.chars().collect::<Vec<char>>();
    match charlist.get(index) {
      Some(val) => Some(*val as u32),
      None => None
    }
  }
}


pub fn chunker(input: &str) {
  let len = input.len();
  let mut level = 0;
  let mut parenLevel = 0;
  let mut lastOpening: usize;
  let mut lastOpeningParen: usize;
  let mut emitFrom = 0;
  let mut chunkerCurrentIndex = 0;
  // let mut chunks = vec![];
  let mut cc: u32 = 0;

  // let mut emitChunk = |force: Option<bool>| {
  //   let len = chunkerCurrentIndex - emitFrom;
  //   if ((len < 512) && !force.unwrap_or(true)) || len == 0 {
  //     return;
  //   }
  //   chunks.push(&input[emitFrom..chunkerCurrentIndex + 1]);
  //   emitFrom = chunkerCurrentIndex + 1;
  // };

  loop {
    if chunkerCurrentIndex < len {
      chunkerCurrentIndex = chunkerCurrentIndex.clone() + 1;
      cc = input.charCodeAt(chunkerCurrentIndex.clone()).unwrap();
      match cc {
        40 => {
          parenLevel += 1;
          lastOpeningParen = chunkerCurrentIndex.clone();
          continue;
        }
        41 => {}
        59 => {
          if parenLevel != 0 {
            // emitChunk(None);
          }
          continue;
        }
        123 => {
          level += 1;
          lastOpening = chunkerCurrentIndex.clone();
        }
        125 => {}
        92 => {}
        34 | 39 | 96 => {}
        47 => {}
        42 => {}
        _ => continue
      }
    } else {
      break;
    }
  }
}