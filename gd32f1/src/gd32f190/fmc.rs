#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wait state register"]
    pub ws: crate::Reg<ws::WS_SPEC>,
    #[doc = "0x04 - Flash unlock key register"]
    pub key: crate::Reg<key::KEY_SPEC>,
    #[doc = "0x08 - Flash option byte unlock key register"]
    pub obkey: crate::Reg<obkey::OBKEY_SPEC>,
    #[doc = "0x0c - Flash status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x10 - Flash control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x14 - Flash address register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte status register"]
    pub obstat: crate::Reg<obstat::OBSTAT_SPEC>,
    #[doc = "0x20 - Write protection register"]
    pub wp: crate::Reg<wp::WP_SPEC>,
    _reserved8: [u8; 0xd8],
    #[doc = "0xfc - Flash wait state control register"]
    pub wsen: crate::Reg<wsen::WSEN_SPEC>,
    #[doc = "0x100 - Flash Product ID register"]
    pub pid: crate::Reg<pid::PID_SPEC>,
}
#[doc = "WS register accessor: an alias for `Reg<WS_SPEC>`"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "Wait state register"]
pub mod ws;
#[doc = "KEY register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Flash unlock key register"]
pub mod key;
#[doc = "OBKEY register accessor: an alias for `Reg<OBKEY_SPEC>`"]
pub type OBKEY = crate::Reg<obkey::OBKEY_SPEC>;
#[doc = "Flash option byte unlock key register"]
pub mod obkey;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Flash status register"]
pub mod stat;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Flash control register"]
pub mod ctl;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Flash address register"]
pub mod addr;
#[doc = "OBSTAT register accessor: an alias for `Reg<OBSTAT_SPEC>`"]
pub type OBSTAT = crate::Reg<obstat::OBSTAT_SPEC>;
#[doc = "Option byte status register"]
pub mod obstat;
#[doc = "WP register accessor: an alias for `Reg<WP_SPEC>`"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Write protection register"]
pub mod wp;
#[doc = "WSEN register accessor: an alias for `Reg<WSEN_SPEC>`"]
pub type WSEN = crate::Reg<wsen::WSEN_SPEC>;
#[doc = "Flash wait state control register"]
pub mod wsen;
#[doc = "PID register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Flash Product ID register"]
pub mod pid;
