use super::Frobnicate;

pub(crate) struct BarFrobnicator {
    msg: String,
}

impl BarFrobnicator {
    pub fn new() -> Self {
        let msg = std::env::args().nth(1).unwrap_or("bar".into());
        Self { msg }
    }
}

impl Frobnicate for BarFrobnicator {
    async fn frobnicate(&self) -> &str {
        &self.msg
    }
}
