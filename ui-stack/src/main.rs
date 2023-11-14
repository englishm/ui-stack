use ui_stack::{
    ustack::Stack,
    utype::{Cons, Nil, UType},
};

fn main() {
    let s = Stack::new();
    let s = s.push(1u8).push(2u16).push(3u32);
    dbg!(s.const_size());
    let s = s.push("foo");
    dbg!(s.const_size());
    let (foo, s) = s.pop();
    assert_eq!(foo, "foo");
    let s = s.discard().discard().discard();
    dbg!(s.const_size());
    dbg!(<Cons<u64, Cons<u8, Nil>>>::const_size());
    dbg!(std::mem::size_of::<Cons<u64, Cons<u8, Nil>>>());
}
