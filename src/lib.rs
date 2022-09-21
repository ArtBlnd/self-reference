#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod utils;
pub(crate) use utils::*;

mod refs;
pub use refs::*;

use core::marker::PhantomPinned;
use core::ops::DerefMut;
use core::pin::Pin;

use pin_project::{pin_project, UnsafeUnpin};
use stable_deref_trait::StableDeref;

/// A Self-Referential Helper.
#[pin_project(UnsafeUnpin)]
pub struct SelfReference<'a, T, R>
where
    for<'this> R: RefDef<'this>,
{
    // SAFETY-NOTE: 'static lifetime is only for placeholder because there is no like 'this or 'phantom lifetime on rust.
    //              using referential object as 'static lifetime is unsound! NEVER use it.
    #[pin]
    referential: <R as RefDef<'a>>::Type,
    #[pin]
    object: T,

    __private: PhantomPinned,
}

impl<'a, T, R> SelfReference<'a, T, R>
where
    for<'this> R: RefDef<'this>,
{
    /// You will "never" able to hold object before its pinned.
    /// try initializing as empty static object. (using Option, NonNull or Empty enum field)
    pub fn new<F>(object: T, init: F) -> Self
    where
        F: FnOnce() -> <R as RefDef<'static>>::Type,
    {
        let referential = unsafe { detach_lifetime_ref::<R>((init)()) };

        Self {
            object,
            referential,
            __private: PhantomPinned,
        }
    }

    pub fn new_stable<F>(mut object: T, init: F) -> Self
    where
        T: StableDeref + DerefMut,
        F: FnOnce(&mut T::Target) -> <R as RefDef<'_>>::Type,
    {
        let referential = unsafe { detach_lifetime_ref::<R>((init)(object.deref_mut())) };

        Self {
            object,
            referential,
            __private: PhantomPinned,
        }
    }

    /// reset referenceial object using object.
    /// object is now pinned so initializing referential type is safe.
    /// This is also useful when you consumed your own reference. (like in AsyncIterator)
    pub fn reset<'s, F>(self: Pin<&'s mut Self>, f: F)
    where
        F: FnOnce(Pin<&'s mut T>) -> <R as RefDef<'s>>::Type,
    {
        let mut proj = self.project();

        let value = unsafe { detach_lifetime_ref::<R>((f)(proj.object)) };
        proj.referential.set(value);
    }

    /// get pinned mutable referencial object that has self lifetime.
    pub fn pin_mut<'s>(self: Pin<&'s mut Self>) -> Pin<&'s mut <R as RefDef<'s>>::Type> {
        let referential = self.project().referential;
        unsafe { detach_lifetime_pin_mut::<R>(referential) }
    }

    /// get pinned referencial object that has self lifetime.
    pub fn pin_ref<'s>(self: Pin<&'s Self>) -> Pin<&'s <R as RefDef<'s>>::Type> {
        let referential = self.project_ref().referential;
        unsafe { detach_lifetime_pin_ref::<R>(referential) }
    }
}

impl<'a, T, R> SelfReference<'a, T, R>
where
    for<'this> R: RefDef<'this>,
    for<'this> <R as RefDef<'this>>::Type: Unpin,
{
    /// get mutable referencial object that has self lifetime.
    pub fn get_mut<'s>(self: Pin<&'s mut Self>) -> &'s mut <R as RefDef<'s>>::Type {
        self.pin_mut().get_mut()
    }

    /// get referencial object that has self lifetime.
    pub fn get_ref<'s>(self: Pin<&'s Self>) -> &'s <R as RefDef<'s>>::Type {
        self.pin_ref().get_ref()
    }
}

impl<'a, T, R> SelfReference<'a, T, R>
where
    for<'this> R: RefDef<'this>,
    T: Unpin,
{
    /// reset referenceial object using unpinned object.
    /// object is now pinned so initializing referential type is safe.
    /// This is also useful when you consumed your own reference. (like in AsyncIterator)
    pub fn reset_unpin<'s, F>(self: Pin<&'s mut Self>, f: F)
    where
        F: FnOnce(&'s mut T) -> <R as RefDef<'s>>::Type,
    {
        let mut proj = self.project();

        let value = unsafe { detach_lifetime_ref::<R>((f)(proj.object.get_mut())) };
        proj.referential.set(value);
    }
}

unsafe impl<'a, T, R> UnsafeUnpin for SelfReference<'a, T, R>
where
    for<'this> R: RefDef<'this>,
    T: StableDeref,
{
}
