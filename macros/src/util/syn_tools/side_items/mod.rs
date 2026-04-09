//! Side items.
//! 
//! This module provides new types for each syntax type, implementing
//! [`SideItem`](super::SideItem).

pub(crate) use field_nt::*;
pub(crate) use impl_item_nt::*;
pub(crate) use trait_item_nt::*;
pub(crate) use variant_nt::*;

mod field_nt;
mod impl_item_nt;
mod trait_item_nt;
mod variant_nt;
