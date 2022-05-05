use crate::Expect;

use super::context::Context;

pub trait Action {
    fn execute(&self, ctx: &Context) -> Expect<()>;

    // fn display(&self, ctx: &Context) -> Expect<String>;
}
