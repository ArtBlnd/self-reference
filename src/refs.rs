use core::marker::PhantomData;

// Reference gateway
pub trait RefDef<'this> {
    type Type;
}

pub struct MutRef<T>(PhantomData<T>);
impl<'this, T> RefDef<'this> for MutRef<T>
where
    T: 'this
{
    type Type = &'this mut T;
}