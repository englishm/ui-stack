#![feature(macro_metavar_expr)]

use ui_stack::{
    pop_statement,
    ustack::Stack,
    utype,
    utype::{Cons, Nil, UType},
};

fn main() {
    // let (): utype!(&str, u8, u16);
    pop_statement!(String, u8, u16);
}
