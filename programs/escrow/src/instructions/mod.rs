pub mod main;
pub mod deposit;
pub mod withdraw;
pub mod pick_winner;
pub mod claim_prize;

pub use main::*; // this is gonna use all the functions written in make module
pub use deposit::*;
pub use withdraw::*;
pub use pick_winner::*;
pub use claim_prize::*;