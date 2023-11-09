pub trait UType: Default {
    fn const_size() -> usize;
    fn const_len() -> usize;
}

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

impl Default for Nil {
    fn default() -> Self {
        Self {}
    }
}

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

impl<H: Sized, T: UType> Default for Cons<H, T> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
