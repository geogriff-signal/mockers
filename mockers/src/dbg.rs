use std::fmt::{Debug, Formatter, Result};

pub struct MaybeDebugWrapper<'a, T: ?Sized>(&'a T);

trait MaybeDebug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
#[cfg(unstable)]
impl<T> MaybeDebug for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "???")
    }
}
impl<T: Debug> MaybeDebug for T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt(f)
    }
}
#[cfg(unstable)]
impl<'t, T: ?Sized> Debug for MaybeDebugWrapper<'t, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}
impl<'t, T: ?Sized> Debug for MaybeDebugWrapper<'t, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "???")
    }
}

pub fn dbg<T: ?Sized>(t: &T) -> MaybeDebugWrapper<'_, T> {
    MaybeDebugWrapper(t)
}
