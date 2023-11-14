macro_rules! all_the_tuples {
    ($name:ty) => {
        $name!(H1,);
        $name!(H1, H2);
        $name!(H1, H2, H3);
        $name!(H1, H2, H3, H4);
        $name!(H1, H2, H3, H4, H5);
        $name!(H1, H2, H3, H4, H5, H6);
        $name!(H1, H2, H3, H4, H5, H6, H7);
        $name!(H1, H2, H3, H4, H5, H6, H7, H8);
        $name!(H1, H2, H3, H4, H5, H6, H7, H8, H9);
        $name!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10);
    };
}

macro_rules! utype {
    () => {crate::utype::Nil};
    ($H1:ty) => {crate::utype::Cons<$H1, crate::utype::Nil>};
    ($H1:ty, $($HN:ty),*) => { crate::utype::Cons< $H1, utype!($($HN),*) > };
}

#[cfg(test)]
mod tests {
    #[test]
    fn utype() {
        assert_eq!(
            "ui_stack::utype::Cons<&str, ui_stack::utype::Cons<u8, ui_stack::utype::Cons<u16, ui_stack::utype::Nil>>>",
            std::any::type_name::<utype!(&str, u8, u16)>()
        );
    }
}
