use crate::refs::RefDef;

use core::mem;

// SAFETY-NOTE: we are trasmutting same type with different lifetime, unless lifetime
//              based generic speialization is on stable this is always safe.
#[inline]
pub unsafe fn detach_lifetime_ref<'this, R: ?Sized>(v: R::Type<'_>) -> R::Type<'this>
where
    R: RefDef,
{
    // I don't know why rustc indicates that different lifetime with same type has different size.
    // we are just using trasmute_copy and forget instead of transmute.
    let value = mem::transmute_copy(&v);
    mem::forget(v);

    return value;
}
