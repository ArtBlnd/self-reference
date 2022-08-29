use std::rc::Rc;
use std::sync::Arc;
use std::boxed::Box;


/// Type marker for stable pointers.
pub unsafe trait StablePtr { }

unsafe impl<T> StablePtr for Rc<T> { }
unsafe impl<T> StablePtr for Arc<T> { }
unsafe impl<T> StablePtr for Box<T> { }