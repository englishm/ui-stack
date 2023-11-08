pub trait UType {
    fn const_size() -> usize;
    fn const_len() -> usize;
}
trait NilT: UType {}
trait ConsT: UType {}

#[derive(Debug)]
pub struct Nil;
impl UType for Nil {
    fn const_size() -> usize {
        0
    }

    fn const_len() -> usize {
        0
    }
}
impl NilT for Nil {}

#[derive(Debug)]
pub struct Cons<H, T: UType> {
    _phantom: std::marker::PhantomData<(H, T)>,
}

impl<H, T> UType for Cons<H, T>
where
    H: Sized,
    T: UType,
{
    fn const_size() -> usize {
        std::mem::size_of::<H>() + T::const_size()
    }

    fn const_len() -> usize {
        1 + T::const_len()
    }
}

impl<H, T> ConsT for Cons<H, T>
where
    H: Sized,
    T: UType,
{
}
