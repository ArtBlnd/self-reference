use crate::refs::RefDef;

use core::mem;
use core::pin::Pin;

// SAFETY-NOTE: we are trasmutting same type with different lifetime, unless lifetime
//              based generic speialization is on stable this is always safe.
#[inline]
pub unsafe fn detach_lifetime_ref<'this, R: ?Sized>(
    v: <R as RefDef<'_>>::Type,
) -> <R as RefDef<'this>>::Type
where
    for<'a> R: RefDef<'a>,
    for<'a> <R as RefDef<'a>>::Type: Sized,
{
    // I don't know why rustc indicates that different lifetime with same type has different size.
    // we are just using trasmute_copy and forget instead of transmute.
    let value = mem::transmute_copy(&v);
    mem::forget(v);

    return value;
}

#[inline]
pub unsafe fn detach_lifetime_pin<'this, R: ?Sized>(
    v: Pin<&mut <R as RefDef<'_>>::Type>,
) -> Pin<&'this mut <R as RefDef<'this>>::Type>
where
    for<'a> R: RefDef<'a>,
{
    mem::transmute(v)
}
