trait Nil: Node {}

trait Cons<H, T: Node>: Node {}

trait Node {
    // fn push<H, C>(self, h: H) -> C
    // where
    //     Self: Sized,
    //     C: Cons<H, Self>;
}

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

#[derive(Debug)]
struct Empty;

impl Node for Empty {}
impl Nil for Empty {}

fn main() {
    let s = Stack::new();
    dbg!(s);
}
