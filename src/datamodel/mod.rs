mod buffer;
mod function;
mod list;
mod record;
mod table;
mod tuple;
mod value;

pub use buffer::Buffer;
pub use function::Function;
pub use list::List;
pub use record::{Record, WeakRecord};
pub use table::Table;
pub use tuple::Tuple;
pub use value::{Bool, Identity, Integer, Real, Unknown, Value, ValueTryIntoError};

// if a garbage collector for Rc cycles is ever needed, impl ToWeak for all Rc
// container types, and impl WeakContainer for a Weak<_> version of each type.

// pub trait WeakContainer {
//     fn get_children(&self) -> Option<Vec<Box<dyn WeakContainer>>>;
//     fn as_ptr(&self) -> *const ();
// }

// pub trait ToWeak {
//     fn to_weak(&self) -> Box<dyn WeakContainer>;
// }
