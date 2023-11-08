use ui_stack::ustack::Stack;

fn main() {
    let s = Stack::new();
    let s = s.push(1u8).push(2u8).push(3u32).push("foo");
    let s = s.discard().discard().discard().discard();
    dbg!(s);
}
