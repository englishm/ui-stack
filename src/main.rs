trait Nil: Node {}

trait Cons<H, T: Node>: Node {}

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
    fn push<H>(self: Stack<T>, h: H) -> Stack<NonEmpty<H, T>>
    where
        Self: Sized,
        H: Sized,
    {
        let mut data = self.data;
        let h_as_bytes = unsafe {
            std::slice::from_raw_parts(&h as *const H as *const u8, std::mem::size_of::<H>())
        };
        data.extend(h_as_bytes);
        Stack {
            _phantom: std::marker::PhantomData,
            data: data,
        }
    }
}

#[derive(Debug)]
struct Empty;

impl Node for Empty {}
impl Nil for Empty {}

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
impl<H, T> Cons<H, T> for NonEmpty<H, T>
where
    H: Sized,
    T: Node,
{
}

fn main() {
    let s = Stack::new();
    let s = s.push(1u8);
    dbg!(s);
}
