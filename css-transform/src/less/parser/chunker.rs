pub fn charCodeAt(txt: &str, index: usize) -> u32 {
    let charlist: Vec<char> = txt.chars().collect();
    let val = charlist.get(index).unwrap();
    return *val as u32;
}


fn emitChunk(force: bool, chunkerCurrentIndex: i32, emitFrom: &mut i32) {
    let len = chunkerCurrentIndex - *emitFrom;
    if ((len < 512) && !force) || len == 0 {
        return;
    }
    // chunks.push(input.slice(emitFrom, chunkerCurrentIndex + 1));
    *emitFrom = chunkerCurrentIndex + 1;
    return;
}

pub fn chunker(input: &str) {
    let len = input.len();
    let mut level = 0;
    let mut parenLevel = 0;
    let mut emitFrom = 0;
    let mut chunkerCurrentIndex = 0;
    let mut cc: u32 = 0;


    loop {
        if chunkerCurrentIndex < len {
            chunkerCurrentIndex += 1;
            cc = charCodeAt(input, chunkerCurrentIndex);
            match cc {
                40 => {}
                41 => {}
                59 => {}
                123 => {}
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