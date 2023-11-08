use crate::utype::{Cons, Nil, UType};

#[derive(Debug)]
pub struct Stack<T: UType> {
    _phantom: std::marker::PhantomData<T>,
    data: Vec<u8>,
}

impl Stack<Nil> {
    pub fn new() -> Stack<Nil> {
        Stack {
            _phantom: std::marker::PhantomData,
            data: Vec::new(),
        }
    }
}

impl<T: UType> Stack<T> {
    pub fn push<H>(mut self: Stack<T>, h: H) -> Stack<Cons<H, T>>
    where
        Self: Sized,
        H: Sized,
    {
        let h_as_bytes = unsafe {
            std::slice::from_raw_parts(&h as *const H as *const u8, std::mem::size_of::<H>())
        };
        self.data.extend(h_as_bytes);
        unsafe { std::mem::transmute(self) }
    }
}

impl<H, R: UType> Stack<Cons<H, R>> {
    pub fn pop(mut self: Stack<Cons<H, R>>) -> (H, Stack<R>)
    where
        R: UType,
        H: Sized,
    {
        let size_of_h = std::mem::size_of::<H>();
        let h = unsafe {
            self.data
                .drain(self.data.len() - size_of_h..)
                .collect::<Vec<u8>>()
                .as_ptr()
                .cast::<H>()
                .read()
        };
        (h, unsafe { std::mem::transmute(self) })
    }

    pub fn discard(mut self: Stack<Cons<H, R>>) -> Stack<R> {
        let size_of_h = std::mem::size_of::<H>();
        self.data.drain(self.data.len() - size_of_h..);
        unsafe { std::mem::transmute(self) }
    }
}

impl From<Stack<Nil>> for () {
    fn from(_value: Stack<Nil>) -> () {
        ()
    }
}

impl<H1> From<Stack<Cons<H1, Nil>>> for (H1,) {
    fn from(value: Stack<Cons<H1, Nil>>) -> (H1,) {
        let (h1, _s) = value.pop();
        (h1,)
    }
}

impl<H1, H2> From<Stack<Cons<H1, Cons<H2, Nil>>>> for (H1, H2) {
    fn from(value: Stack<Cons<H1, Cons<H2, Nil>>>) -> (H1, H2) {
        let (h1, s) = value.pop();
        let (h2, _) = s.pop();
        (h1, h2)
    }
}

impl<H1, H2, H3> From<Stack<Cons<H1, Cons<H2, Cons<H3, Nil>>>>> for (H1, H2, H3) {
    fn from(value: Stack<Cons<H1, Cons<H2, Cons<H3, Nil>>>>) -> (H1, H2, H3) {
        let (h1, s) = value.pop();
        let (h2, s) = s.pop();
        let (h3, _) = s.pop();
        (h1, h2, h3)
    }
}

trait PopChain<H, R> {
    fn pop(self) -> (H, Stack<R>)
    where
        R: UType,
        H: Sized;
}

impl<H1, H2, R: UType> PopChain<H2, R> for (H1, Stack<Cons<H2, R>>) {
    fn pop(self) -> (H2, Stack<R>)
    where
        H1: Sized,
        H2: Sized,
        R: UType,
    {
        let (_h, s) = self;
        s.pop()
    }
}
