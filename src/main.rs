trait Node {}

#[derive(Debug)]
struct Stack<T: Node> {
    _phantom: std::marker::PhantomData<T>,
    data: Vec<u8>,
}

impl Stack<Empty> {
    fn new() -> Stack<Empty> {
        Stack {
            _phantom: std::marker::PhantomData,
            data: Vec::new(),
        }
    }
}
impl<T: Node> Stack<T> {
    fn push<H>(mut self: Stack<T>, h: H) -> Stack<NonEmpty<H, T>>
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
impl<H, R: Node> Stack<NonEmpty<H, R>> {
    fn pop(mut self: Stack<NonEmpty<H, R>>) -> (H, Stack<R>)
    where
        R: Node,
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
}

impl<H1, H2> From<Stack<NonEmpty<H1, NonEmpty<H2, Empty>>>> for (H1, H2) {
    fn from(value: Stack<NonEmpty<H1, NonEmpty<H2, Empty>>>) -> (H1, H2) {
        let (h1, s) = value.pop();
        let (h2, _) = s.pop();
        (h1, h2)
    }
}
impl<H1, H2, H3> From<Stack<NonEmpty<H1, NonEmpty<H2, NonEmpty<H3, Empty>>>>> for (H1, H2, H3) {
    fn from(value: Stack<NonEmpty<H1, NonEmpty<H2, NonEmpty<H3, Empty>>>>) -> (H1, H2, H3) {
        let (h1, s) = value.pop();
        let (h2, s) = s.pop();
        let (h3, _) = s.pop();
        (h1, h2, h3)
    }
}

trait PopChain<H, R> {
    fn pop(self) -> (H, Stack<R>)
    where
        R: Node,
        H: Sized;
}

impl<H1, H2, R: Node> PopChain<H2, R> for (H1, Stack<NonEmpty<H2, R>>) {
    fn pop(self) -> (H2, Stack<R>)
    where
        H1: Sized,
        H2: Sized,
        R: Node,
    {
        let (_h, s) = self;
        s.pop()
    }
}

#[derive(Debug)]
struct Empty;
impl Node for Empty {}

#[derive(Debug)]
struct NonEmpty<H, T: Node> {
    _phantom: std::marker::PhantomData<(H, T)>,
}

impl<H, T> Node for NonEmpty<H, T>
where
    H: Sized,
    T: Node,
{
}

fn main() {
    let s = Stack::new();
    let s = s.push(1u8).push(2u8).push(3u32).push("foo");
    dbg!(&s);
    let (h, s) = s.pop();
    dbg!(&h, &s);
    let (h1, h2, h3) = s.into();
    dbg!(&h1, &h2, &h3);
}
