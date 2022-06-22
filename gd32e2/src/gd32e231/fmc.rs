#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state register"]
    pub ws: crate::Reg<ws::WS_SPEC>,
    #[doc = "0x04 - Unlock key register"]
    pub key: crate::Reg<key::KEY_SPEC>,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: crate::Reg<obkey::OBKEY_SPEC>,
    #[doc = "0x0c - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x10 - Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x14 - Address register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte control register"]
    pub obstat: crate::Reg<obstat::OBSTAT_SPEC>,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: crate::Reg<wp::WP_SPEC>,
    _reserved8: [u8; 0xdc],
    #[doc = "0x100 - Product ID register"]
    pub pid: crate::Reg<pid::PID_SPEC>,
}
#[doc = "WS register accessor: an alias for `Reg<WS_SPEC>`"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "wait state register"]
pub mod ws;
#[doc = "KEY register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Unlock key register"]
pub mod key;
#[doc = "OBKEY register accessor: an alias for `Reg<OBKEY_SPEC>`"]
pub type OBKEY = crate::Reg<obkey::OBKEY_SPEC>;
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "OBSTAT register accessor: an alias for `Reg<OBSTAT_SPEC>`"]
pub type OBSTAT = crate::Reg<obstat::OBSTAT_SPEC>;
#[doc = "Option byte control register"]
pub mod obstat;
#[doc = "WP register accessor: an alias for `Reg<WP_SPEC>`"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "PID register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Product ID register"]
pub mod pid;
