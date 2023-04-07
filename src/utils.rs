use crate::refs::RefDef;

use core::mem;
use core::pin::Pin;

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

#[inline]
pub unsafe fn detach_lifetime_get_ref<'x, 'y, 'z: 'y, R: ?Sized>(
    v: &'x R::Type<'y>,
) -> &'x R::Type<'z>
where
    R: RefDef,
{
    // I don't know why rustc indicates that different lifetime with same type has different size.
    // we are just using trasmute_copy and forget instead of transmute.
    let value = mem::transmute_copy(&v);
    mem::forget(v);

    return value;
}

#[inline]
pub unsafe fn detach_lifetime_get_mut<'x, 'y, 'z: 'y, R: ?Sized>(
    v: &'x mut R::Type<'y>,
) -> &'x mut R::Type<'z>
where
    R: RefDef,
{
    // I don't know why rustc indicates that different lifetime with same type has different size.
    // we are just using trasmute_copy and forget instead of transmute.
    let value = mem::transmute_copy(&v);
    mem::forget(v);

    return value;
}

#[inline]
pub unsafe fn detach_lifetime_pin_mut<'x, 'y, 'z: 'y, R: ?Sized>(
    v: Pin<&'x mut R::Type<'y>>,
) -> Pin<&'x mut R::Type<'z>>
where
    R: RefDef,
{
    mem::transmute(v)
}

#[inline]
pub unsafe fn detach_lifetime_pin_ref<'x, 'y, 'z: 'y, R: ?Sized>(
    v: Pin<&'x R::Type<'y>>,
) -> Pin<&'x R::Type<'z>>
where
    R: RefDef,
{
    mem::transmute(v)
}
