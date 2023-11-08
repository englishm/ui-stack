use ui_stack::ustack::Stack;

fn main() {
    let s = Stack::new();
    let s = s.push(1u8).push(2u16).push(3u32);
    dbg!(s.const_size());
    let s = s.push("foo");
    dbg!(s.const_size());
}
