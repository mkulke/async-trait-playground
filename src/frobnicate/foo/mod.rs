use super::Frobnicate;

pub(crate) struct FooFrobnicator;

impl Frobnicate for FooFrobnicator {
    async fn frobnicate(&self) -> &str {
        "foo"
    }
}
