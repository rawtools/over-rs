pub trait Action {
    fn execute(dryrun: bool);
}

mod link;

pub use link::EnsureLink;
