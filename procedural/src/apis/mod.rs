mod assets;
mod benchmark;
mod consensus;
mod evm;
mod system;
mod tanssi;

pub use assets::*;
pub use benchmark::*;
pub use consensus::*;
pub use evm::*;
pub use system::*;
pub use tanssi::*;

use syn::{Ident, Type};

pub fn fetch_ident(ty: &Type) -> Ident {
    match ty {
        Type::Path(p) => p
            .path
            .get_ident()
            .unwrap_or_else(|| panic!("Wrong type received: {:?}", p))
            .clone(),
        _ => panic!("Wrong type received: {:?}", ty),
    }
}
