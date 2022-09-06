//! Library to send messages to slack rooms
//! supports entire messaging API, including attachments and fields
//! also support for built-in colors as well as any hex colors

pub use self::attachment::*;
pub use self::error::*;
pub use self::hex::*;
pub use self::payload::*;
pub use self::slack::*;

#[macro_use]
mod macros;

mod attachment;
mod error;
mod hex;
mod payload;
mod slack;
