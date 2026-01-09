pub mod main;
pub mod deposit;
pub mod withdraw;

pub use main::*; // this is gonna use all the functions written in make module
pub use deposit::*;
pub use withdraw::*;