use ui_stack_macros::pop_statement;

fn main() {
    // let (): utype!(&str, u8, u16);
    pop_statement!(let foo = "foo";);
}
