/* Kumark
 * Fiathux Su 2021-11-14
 */

use super::context::Symbol;
use super::elements::Element;
use super::elements::ElementPos;

pub enum ParseResult {
    Break,
    Continue,
    Parsed,
}

pub trait SyntaxParser {
    // parse a symbol
    fn parse(&mut self, s: &Symbol) -> ParseResult;
    // parse a element
    fn subparse(&mut self, elem: &dyn Element) -> ParseResult;
    // create element from parsed string
    fn get_element(&mut self, raw: String, pos: ElementPos) -> dyn Element;
}

pub struct SyntaxLayer {
    layers: Vec<Box<dyn SyntaxParser>>,
}
