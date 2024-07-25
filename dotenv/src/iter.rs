use crate::{
    errors::{Error, Result},
    parse,
};
use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader, Read},
};

pub struct Iter<R> {
    lines: QuotedLines<BufReader<R>>,
    substitution_data: HashMap<String, Option<String>>,
}

impl<R: Read> Iter<R> {
    pub fn new(reader: R) -> Self {
        Self {
            lines: QuotedLines {
                buf: BufReader::new(reader),
            },
            substitution_data: HashMap::new(),
        }
    }

    /// Loads all variables found in the `reader` into the environment,
    /// preserving any existing environment variables of the same name.
    ///
    /// If a variable is specified multiple times within the reader's data,
    /// then the first occurrence is applied.
    ///
    /// # Safety
    ///
    /// Even though this function is currently not marked as `unsafe`, it will
    /// be when the Rust 2024 edition is stabalized. This is tracked in
    /// [rust#27970](https://github.com/rust-lang/rust/issues/27970).
    ///
    /// This function is safe to call in a single-threaded program.
    ///
    /// In multi-threaded programs, you must ensure there are no other threads
    /// concurrently writing or *reading*(!) from the environment. You are
    /// responsible for figuring out how to achieve this, but we strongly
    /// suggest only using `dotenvy` before spawning any threads
    /// (this includes the use of `tokio::main`).
    pub fn load(mut self) -> Result<()> {
        self.remove_bom()?;

        for item in self {
            let (key, value) = item?;
            if env::var(&key).is_err() {
                env::set_var(&key, value);
            }
        }

        Ok(())
    }

    /// Loads all variables found in the `reader` into the environment,
    /// overriding any existing environment variables of the same name.
    ///
    /// If a variable is specified multiple times within the reader's data,
    /// then the last occurrence is applied.
    pub fn load_override(mut self) -> Result<()> {
        self.remove_bom()?;

        for item in self {
            let (key, value) = item?;
            env::set_var(key, value);
        }

        Ok(())
    }

    fn remove_bom(&mut self) -> Result<()> {
        let buffer = self.lines.buf.fill_buf().map_err(Error::Io)?;
        // https://www.compart.com/en/unicode/U+FEFF
        if buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
            // remove the BOM from the bufreader
            self.lines.buf.consume(3);
        }
        Ok(())
    }
}

struct QuotedLines<B> {
    buf: B,
}

enum ParseState {
    Complete,
    Escape,
    StrongOpen,
    StrongOpenEscape,
    WeakOpen,
    WeakOpenEscape,
    Comment,
    WhiteSpace,
}

fn eval_end_state(prev_state: ParseState, buf: &str) -> (usize, ParseState) {
    let mut cur_state = prev_state;
    let mut cur_pos: usize = 0;

    for (pos, c) in buf.char_indices() {
        cur_pos = pos;
        cur_state = match cur_state {
            ParseState::WhiteSpace => match c {
                '#' => return (cur_pos, ParseState::Comment),
                '\\' => ParseState::Escape,
                '"' => ParseState::WeakOpen,
                '\'' => ParseState::StrongOpen,
                _ => ParseState::Complete,
            },
            ParseState::Escape => ParseState::Complete,
            ParseState::Complete => match c {
                c if c.is_whitespace() && c != '\n' && c != '\r' => ParseState::WhiteSpace,
                '\\' => ParseState::Escape,
                '"' => ParseState::WeakOpen,
                '\'' => ParseState::StrongOpen,
                _ => ParseState::Complete,
            },
            ParseState::WeakOpen => match c {
                '\\' => ParseState::WeakOpenEscape,
                '"' => ParseState::Complete,
                _ => ParseState::WeakOpen,
            },
            ParseState::WeakOpenEscape => ParseState::WeakOpen,
            ParseState::StrongOpen => match c {
                '\\' => ParseState::StrongOpenEscape,
                '\'' => ParseState::Complete,
                _ => ParseState::StrongOpen,
            },
            ParseState::StrongOpenEscape => ParseState::StrongOpen,
            // Comments last the entire line.
            ParseState::Comment => panic!("should have returned early"),
        };
    }
    (cur_pos, cur_state)
}

impl<B: BufRead> Iterator for QuotedLines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        let mut cur_state = ParseState::Complete;
        let mut buf_pos;
        let mut cur_pos;
        loop {
            buf_pos = buf.len();
            match self.buf.read_line(&mut buf) {
                Ok(0) => {
                    if matches!(cur_state, ParseState::Complete) {
                        return None;
                    }
                    let len = buf.len();
                    return Some(Err(Error::LineParse(buf, len)));
                }
                Ok(_n) => {
                    // Skip lines which start with a # before iteration
                    // This optimizes parsing a bit.
                    if buf.trim_start().starts_with('#') {
                        return Some(Ok(String::with_capacity(0)));
                    }
                    let result = eval_end_state(cur_state, &buf[buf_pos..]);
                    cur_pos = result.0;
                    cur_state = result.1;

                    match cur_state {
                        ParseState::Complete => {
                            if buf.ends_with('\n') {
                                buf.pop();
                                if buf.ends_with('\r') {
                                    buf.pop();
                                }
                            }
                            return Some(Ok(buf));
                        }
                        ParseState::Escape
                        | ParseState::StrongOpen
                        | ParseState::StrongOpenEscape
                        | ParseState::WeakOpen
                        | ParseState::WeakOpenEscape
                        | ParseState::WhiteSpace => {}
                        ParseState::Comment => {
                            buf.truncate(buf_pos + cur_pos);
                            return Some(Ok(buf));
                        }
                    }
                }
                Err(e) => return Some(Err(Error::Io(e))),
            }
        }
    }
}

impl<R: Read> Iterator for Iter<R> {
    type Item = Result<(String, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = match self.lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Some(Err(err)),
                None => return None,
            };

            match parse::parse_line(&line, &mut self.substitution_data) {
                Ok(Some(result)) => return Some(Ok(result)),
                Ok(None) => {}
                Err(err) => return Some(Err(err)),
            }
        }
    }
}
