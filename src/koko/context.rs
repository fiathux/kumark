/* Kumark
 * Text context
 * ------
 * Fiathux Su 2021-11-13
 */

use std::fmt;
use std::str::Chars;

// generic symbol object
#[derive(Clone, Copy)]
pub enum MultiSymbol {
    Char(char),
    LineHead,
}

// Symbol object with iterate controller
#[derive(Clone, Copy)]
enum MultiSymbolSys {
    HEAD,
    EOF,
    Sym(MultiSymbol),
}

// symbol position
pub struct SymbolPos {
    pub ln: i32,
    pub col: i32,
    pub offset: i32,
}

// text context iterator
pub struct ContextIter<'a> {
    content: Chars<'a>,
    ln: i32,
    col: i32,
    offset: i32,
    last: MultiSymbolSys,
}

// A symbol in context
pub struct Symbol {
    pub c: MultiSymbol,
    pub pos: SymbolPos,
}

// formatting
impl fmt::Display for MultiSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultiSymbol::Char(s) => write!(f, "'{}'", s),
            MultiSymbol::LineHead => write!(f, "<Ln>"),
        }
    }
}

impl MultiSymbol {
    // check symbol is head of a line
    pub fn ln_head(&self) -> bool {
        match self {
            MultiSymbol::Char(_s) => false,
            MultiSymbol::LineHead => true,
        }
    }

    // retrive symbol as a character
    pub fn as_char(&self) -> char {
        match self {
            MultiSymbol::Char(s) => *s,
            MultiSymbol::LineHead => ' ',
        }
    }
}

//
impl ContextIter<'_> {
    // create new Context
    pub fn new<'a>(content: Chars<'a>) -> ContextIter<'a> {
        ContextIter {
            content: content,
            ln: 0,
            col: 0,
            offset: 0,
            last: MultiSymbolSys::HEAD,
        }
    }

    // get current position
    pub fn pos(&mut self) -> SymbolPos {
        SymbolPos {
            ln: self.ln,
            col: self.col,
            offset: self.offset,
        }
    }

    // move to next character
    fn next_char_status(&mut self) {
        let cx = self.content.next();
        if cx.is_none() {
            self.last = MultiSymbolSys::EOF;
        } else {
            let s = cx.unwrap();
            if s == '\n' {
                self.last = MultiSymbolSys::Sym(MultiSymbol::LineHead);
                self.ln += 1;
                self.col = 0;
            } else {
                self.last = MultiSymbolSys::Sym(MultiSymbol::Char(s));
                self.col += 1;
            }
            self.offset += 1;
        }
    }

    // do iterate with a snapshot.
    // it will recovery to snapshot on error returned
    pub fn tryiter<T, E, F: Fn(&mut ContextIter) -> Result<T, E>>(&mut self, f: F) -> Result<T, E> {
        // create snapshot
        let ctx_snap = ContextIter {
            content: self.content.clone(),
            ln: self.ln,
            col: self.col,
            offset: self.offset,
            last: self.last,
        };
        // do iterate
        let rst = f(self);
        // recovery on failed
        if rst.is_err() {
            self.content = ctx_snap.content;
            self.ln = ctx_snap.ln;
            self.col = ctx_snap.col;
            self.offset = ctx_snap.offset;
            self.last = ctx_snap.last;
        }
        rst
    }
}

// Iterator implemention
impl Iterator for ContextIter<'_> {
    type Item = Symbol;

    //
    fn next(&mut self) -> Option<Self::Item> {
        match self.last {
            MultiSymbolSys::HEAD => {
                let pos = self.pos();
                self.next_char_status();
                Some(Symbol {
                    c: MultiSymbol::LineHead,
                    pos: pos,
                })
            }
            MultiSymbolSys::EOF => None,
            MultiSymbolSys::Sym(s) => {
                let pos = self.pos();
                self.next_char_status();
                Some(Symbol { c: s, pos: pos })
            }
        }
    }
}
