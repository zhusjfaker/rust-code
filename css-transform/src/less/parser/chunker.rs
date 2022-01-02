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


pub fn chunker(input: &str) -> Result<Vec<&str>, String> {
  let len = input.len();
  let mut level = 0;
  let mut parenLevel = 0;
  let mut lastOpening: usize = 0;
  let mut lastOpeningParen: usize = 0;
  let mut lastMultiComment: usize = 0;
  let mut lastMultiCommentEndBrace: usize = 0;
  let mut chunks = vec![];
  let mut emitFrom = 0;
  let mut chunkerCurrentIndex = 0;
  let mut currentChunkStartIndex: usize;
  let mut cc: u32 = 0;
  let mut cc2: u32;
  let mut matched;

  let mut emitChunk = |emitFrom: &mut usize, chunkerCurrentIndex: &usize, force: Option<bool>| {
    let len = chunkerCurrentIndex - *emitFrom;
    if ((len < 512) && !force.unwrap_or(true)) || len == 0 {
      return;
    }
    chunks.push(&input[*emitFrom..chunkerCurrentIndex + 1]);
    *emitFrom = chunkerCurrentIndex + 1;
  };

  loop {
    if chunkerCurrentIndex < len {
      chunkerCurrentIndex = chunkerCurrentIndex.clone() + 1;
      cc = input.charCodeAt(chunkerCurrentIndex.clone()).unwrap();
      if (cc >= 97 && cc <= 122) || cc < 34 {
        continue;
      }
      match cc {
        40 => {
          parenLevel += 1;
          lastOpeningParen = chunkerCurrentIndex.clone();
          continue;
        }
        41 => {
          parenLevel -= 1;
          if parenLevel < 0 {
            return Err(format!("missing opening `(` {}", &chunkerCurrentIndex));
          }
          continue;
        }
        59 => {
          if parenLevel != 0 {
            emitChunk(&mut emitFrom, &chunkerCurrentIndex, None);
          }
          continue;
        }
        123 => {
          level += 1;
          lastOpening = chunkerCurrentIndex.clone();
          continue;
        }
        125 => {
          level -= 1;
          if level < 0 {
            return Err(format!("missing opening `{}` {}", "{", &chunkerCurrentIndex));
          }
          if level == 0 && parenLevel == 0 {
            emitChunk(&mut emitFrom, &chunkerCurrentIndex, None);
          }
          continue;
        }
        92 => {
          if chunkerCurrentIndex < len - 1 {
            chunkerCurrentIndex += 1;
            continue;
          }
          return Err(format!("unescaped `\\` {}", chunkerCurrentIndex));
        }
        34 | 39 | 96 => {
          matched = 0;
          currentChunkStartIndex = chunkerCurrentIndex;
          chunkerCurrentIndex += 1;
          loop {
            if chunkerCurrentIndex < len {
              cc2 = input.charCodeAt(chunkerCurrentIndex).unwrap();
              if cc2 > 96 {
                continue;
              }
              if cc2 == cc {
                matched = 1;
                break;
              }
              if cc == 92 {
                if chunkerCurrentIndex == len - 1 {
                  return Err(format!("unescaped `\\` {}", chunkerCurrentIndex));
                }
                chunkerCurrentIndex += 1;
              }
            } else {
              break;
            }
            chunkerCurrentIndex += 1;
          }
          if matched == 0 {
            continue;
          }
          return Err(format!("unmatched {} {}", cc.to_string(), currentChunkStartIndex));
        }
        47 => {
          if parenLevel == 0 || chunkerCurrentIndex == len - 1 {
            continue;
          }
          cc2 = input.charCodeAt(chunkerCurrentIndex + 1).unwrap();
          if cc2 == 47 {
            chunkerCurrentIndex += 2;
            loop {
              if chunkerCurrentIndex < len {
                cc2 = input.charCodeAt(chunkerCurrentIndex).unwrap();
                if cc2 <= 13 && (cc2 == 10 || cc2 == 13) {
                  break;
                }
              } else {
                break;
              }
              chunkerCurrentIndex += 1;
            }
          } else if cc2 == 42 {
            lastMultiComment = chunkerCurrentIndex;
            chunkerCurrentIndex = chunkerCurrentIndex;
            chunkerCurrentIndex += 2;
            loop {
              if chunkerCurrentIndex < len - 1 {
                cc2 = input.charCodeAt(chunkerCurrentIndex).unwrap();
                if cc2 == 125 {
                  lastMultiCommentEndBrace = chunkerCurrentIndex;
                }
                if cc2 != 42 {
                  continue;
                }
                if input.charCodeAt(chunkerCurrentIndex + 1).unwrap() == 47 {
                  break;
                }
              } else {
                break;
              }
              chunkerCurrentIndex += 1;
            }
            if chunkerCurrentIndex == len - 1 {
              return Err(format!("missing closing `*/` {}", chunkerCurrentIndex));
            }
            chunkerCurrentIndex += 1;
          }
          continue;
        }
        42 => {
          if chunkerCurrentIndex < len - 1 && input.charCodeAt(chunkerCurrentIndex + 1).unwrap() == 47 {
            return Err(format!("unmatched `/*` {}", chunkerCurrentIndex));
          }
          continue;
        }
        _ => {}
      }
    } else {
      break;
    }
  }

  if level != 0 {
    return if lastMultiComment > lastOpening && lastMultiCommentEndBrace > lastMultiComment {
      Err(format!("missing closing `{}` or `*/` {}", "}", lastOpening))
    } else {
      Err(format!("missing closing `{}` {}", "}", lastOpening))
    };
  } else if parenLevel != 0 {
    return Err(format!("missing closing `)` {}", lastOpeningParen));
  }

  emitChunk(&mut emitFrom, &chunkerCurrentIndex, Some(true));
  Ok(chunks)
}