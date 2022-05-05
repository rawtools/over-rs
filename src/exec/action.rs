use async_trait::async_trait;

use crate::Expect;

use super::context::Context;

#[async_trait]
pub trait Action {
    async fn execute(&self, ctx: &Context) -> Expect<()>;

    // fn display(&self, ctx: &Context) -> Expect<String>;
}
