#![no_std]

mod utils;
pub(crate) use utils::*;
mod refs;
pub use refs::*;

use core::pin::Pin;


/// A Self-Referential Helper.
#[pin_project::pin_project]
pub struct SelfReference<T, R>
where
    for<'this> R: RefDef<'this>
{
    // SAFETY-NOTE: 'static lifetime is only for placeholder because there is no like 'this or 'phantom lifetime on rust. 
    //              using referential object as 'static lifetime is unsound! NEVER use it.
    #[pin]
    referential: <R as RefDef<'static>>::Type,
    #[pin]
    object: T,
}

impl<T, R> SelfReference<T, R>
where
    for<'this> R: RefDef<'this>
{
    /// You are never able to "hold" object before its pinned.
    /// try initializing as empty static object. (using Option, NonNull or Empty enum field)
    pub fn new<F>(object: T, init: F) -> Self 
    where
        F: FnOnce() -> <R as RefDef<'static>>::Type
    {
        Self {
            object,
            referential: (init)()
        }
    }

    /// get referencial object that has self lifetime.
    /// We are returning pinned object. which is safe.
    pub fn projection<'s>(self: Pin<&'s mut Self>) -> Pin<&'s mut <R as RefDef<'s>>::Type> {
        let referential = self.project().referential;
        unsafe { detach_lifetime_pin::<R>(referential) }
    }

    /// reset referenceial object using object.
    /// object is now pinned so initializing referential type is safe.
    /// This can be useful when you consumed your own reference. (like in AsyncIterator)
    pub fn reset<'s, F>(self: Pin<&'s mut Self>, f: F)
    where
        F: FnOnce(Pin<&'s mut T>) -> <R as RefDef<'s>>::Type
    {
        let mut proj = self.project();

        let value = unsafe { detach_lifetime_ref::<R>((f)(proj.object)) };
        proj.referential.set(value);
    }
}
