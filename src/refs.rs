use core::marker::PhantomData;
use core::pin::Pin;

// Reference gateway
pub trait RefDef<'this> {
    type Type;
}

pub struct Mut<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for Mut<T>
where
    T: 'this,
{
    type Type = &'this mut T;
}

pub struct OptionMut<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for OptionMut<T>
where
    T: 'this,
{
    type Type = Option<&'this mut T>;
}

pub struct Ref<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for Ref<T>
where
    T: 'this,
{
    type Type = &'this T;
}

pub struct OptionRef<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for OptionRef<T>
where
    T: 'this,
{
    type Type = Option<&'this T>;
}

pub struct PinMut<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for PinMut<T>
where
    T: 'this,
{
    type Type = Pin<&'this mut T>;
}

pub struct OptionPinMut<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for OptionPinMut<T>
where
    T: 'this,
{
    type Type = Option<Pin<&'this mut T>>;
}

pub struct PinRef<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for PinRef<T>
where
    T: 'this,
{
    type Type = Pin<&'this T>;
}

pub struct OptionPinRef<T: ?Sized>(PhantomData<T>);
impl<'this, T: ?Sized> RefDef<'this> for OptionPinRef<T>
where
    T: 'this,
{
    type Type = Option<Pin<&'this T>>;
}
