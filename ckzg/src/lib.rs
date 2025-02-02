pub mod consts;
pub mod fftsettings;
pub mod finite;
pub mod fk20settings;
pub mod kzgsettings;
pub mod poly;
pub mod utils;

#[cfg(feature = "parallel")]
const RUN_PARALLEL: bool = true;
#[cfg(not(feature = "parallel"))]
const RUN_PARALLEL: bool = false;
