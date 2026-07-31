use crate::input::{Input, Utf8};

#[derive(Debug)]
pub struct Utf8Location {
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Utf8SourceMap {
    line_starts: Vec<usize>,
}

impl Utf8SourceMap {
    pub fn new<'a>(input: &'a Utf8<'a>) -> Self {
        let mut line_starts = vec![0];
        let mut offset = 0;
        while let Some((symbol, new_offset)) = input.read(offset) {
            offset = new_offset;
            if symbol == '\n' {
                line_starts.push(offset);
            }
        }

        Self { line_starts }
    }

    pub fn location(&self, offset: usize) -> Option<Utf8Location> {
        let mut current_line = 0;
        let mut current_start = 0;

        for (line, start) in self.line_starts.iter().enumerate() {
            if *start > offset {
                break;
            }

            current_line = line;
            current_start = *start;
        }

        Some(Utf8Location {
            line: current_line + 1,
            column: offset - current_start,
        })
    }
}
