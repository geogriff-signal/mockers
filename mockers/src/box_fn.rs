macro_rules! panicking_fn {
    ($fun:ident ( $($arg:ident),* )) => {
        {
            let mut maybe_fun = Some($fun);
            move |$($arg),*| {
                maybe_fun.take().expect("BoxFn called twice")($($arg),*)
            }
        }
    }
}

pub struct BoxFn0<T>(Box<FnMut() -> T>);
impl<T> BoxFn0<T> {
    pub fn new<F: 'static + FnOnce() -> T>(f: F) -> Self {
        BoxFn0(Box::new(panicking_fn!(f())))
    }
    pub fn call(mut self) -> T {
        self.0()
    }
}

pub struct BoxFn1<A0, T>(Box<FnMut(A0) -> T>);
impl<A0, T> BoxFn1<A0, T> {
    pub fn new<F: 'static + FnOnce(A0) -> T>(f: F) -> Self {
        BoxFn1(Box::new(panicking_fn!(f(a0))))
    }
    pub fn call(mut self, a0: A0) -> T {
        self.0(a0)
    }
}

pub struct BoxFn2<A0, A1, T>(Box<FnMut(A0, A1) -> T>);
impl<A0, A1, T> BoxFn2<A0, A1, T> {
    pub fn new<F: 'static + FnOnce(A0, A1) -> T>(f: F) -> Self {
        BoxFn2(Box::new(panicking_fn!(f(a0,a1))))
    }
    pub fn call(mut self, a0: A0, a1: A1) -> T {
        self.0(a0, a1)
    }
}

pub struct BoxFn3<A0, A1, A2, T>(Box<FnMut(A0, A1, A2) -> T>);
impl<A0, A1, A2, T> BoxFn3<A0, A1, A2, T> {
    pub fn new<F: 'static + FnOnce(A0, A1, A2) -> T>(f: F) -> Self {
        BoxFn3(Box::new(panicking_fn!(f(a0,a1,a2))))
    }
    pub fn call(mut self, a0: A0, a1: A1, a2: A2) -> T {
        self.0(a0, a1, a2)
    }
}

pub struct BoxFn4<A0, A1, A2, A3, T>(Box<FnMut(A0, A1, A2, A3) -> T>);
impl<A0, A1, A2, A3, T> BoxFn4<A0, A1, A2, A3, T> {
    pub fn new<F: 'static + FnOnce(A0, A1, A2, A3) -> T>(f: F) -> Self {
        BoxFn4(Box::new(panicking_fn!(f(a0,a1,a2,a3))))
    }
    pub fn call(mut self, a0: A0, a1: A1, a2: A2, a3: A3) -> T {
        self.0(a0, a1, a2, a3)
    }
}
