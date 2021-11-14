/* Kumark
 * Fiathux Su 2021-11-13
 */

// element position
pub struct ElementPos {
    ln_b: i32,   // start line position
    ln_e: i32,   // stop line position
    col_b: i32,  // start column position
    col_e: i32,  // stop column position
    offset: i32, // document content offset
}

// element data
pub struct ElementData {
    raw: String,
    pos: ElementPos,
}

pub trait MetaElement {
    fn data(&self) -> &ElementData;
}

// basic element
pub trait Element: MetaElement {
    // get formatted string
    fn formating(&self) -> String;
    //get class-name
    fn class(&self) -> String;
}
