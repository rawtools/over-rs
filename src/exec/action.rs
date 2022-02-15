use super::context::Context;

pub trait Action {
    fn execute(ctx: Context);
}
