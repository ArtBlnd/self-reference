use core::marker::PhantomData;

// Reference gateway
pub trait RefDef<'this> {
    type Type;
}

pub struct MutRef<T>(PhantomData<T>);
impl<'this, T> RefDef<'this> for MutRef<T>
where
    T: 'this,
{
    type Type = &'this mut T;
}

pub struct OptionMutRef<T>(PhantomData<T>);
impl<'this, T> RefDef<'this> for OptionMutRef<T>
where
    T: 'this,
{
    type Type = Option<&'this mut T>;
}
