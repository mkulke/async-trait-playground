use std::future::Future;

#[cfg(feature = "bar")]
mod bar;
mod foo;

pub(crate) trait Frobnicate {
    fn frobnicate(&self) -> impl Future<Output = &str> + Send;
}

pub fn get_frobnicator() -> impl Frobnicate {
    #[cfg(not(feature = "bar"))]
    return foo::FooFrobnicator;

    #[cfg(feature = "bar")]
    bar::BarFrobnicator::new()
}
