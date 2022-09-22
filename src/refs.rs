use core::marker::PhantomData;
use core::pin::Pin;

// Reference gateway
pub trait RefDef {
    type Type<'this>
    where
        Self: 'this;
}

pub struct Mut<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for Mut<T> {
    type Type<'this> = &'this mut T where T: 'this;
}

pub struct OptionMut<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for OptionMut<T> {
    type Type<'this> = Option<&'this mut T> where T: 'this;
}

pub struct Ref<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for Ref<T> {
    type Type<'this> = &'this T where T: 'this;
}

pub struct OptionRef<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for OptionRef<T> {
    type Type<'this> = Option<&'this T> where T: 'this;
}

pub struct PinMut<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for PinMut<T> {
    type Type<'this> = Pin<&'this mut T> where T: 'this;
}

pub struct OptionPinMut<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for OptionPinMut<T> {
    type Type<'this> = Option<Pin<&'this mut T>> where T: 'this;
}

pub struct PinRef<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for PinRef<T> {
    type Type<'this> = Pin<&'this T> where T: 'this;
}

pub struct OptionPinRef<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> RefDef for OptionPinRef<T> {
    type Type<'this> = Option<Pin<&'this T>> where T: 'this;
}
