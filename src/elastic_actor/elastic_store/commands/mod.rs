pub mod simple_index;
pub mod ensure_index_exists;

pub use self::{
    ensure_index_exists::{EnsureIndexExists, MappingDoc},
    simple_index::Document,
};
