pub trait UType {}

#[derive(Debug)]
pub struct Nil;
impl UType for Nil {}

#[derive(Debug)]
pub struct Cons<H, T: UType> {
    _phantom: std::marker::PhantomData<(H, T)>,
}

impl<H, T> UType for Cons<H, T>
where
    H: Sized,
    T: UType,
{
}
