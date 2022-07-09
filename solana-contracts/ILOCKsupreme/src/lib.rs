/****************************************************************
 * ILOCKsupreme Solana Contractj
 ****************************************************************/

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

// additionals modules
// .
// .
// .
// V

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
