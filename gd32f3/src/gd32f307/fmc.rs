#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state counter register"]
    pub ws: crate::Reg<ws::WS_SPEC>,
    #[doc = "0x04 - Unlock key register"]
    pub key: crate::Reg<key::KEY_SPEC>,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: crate::Reg<obkey::OBKEY_SPEC>,
    #[doc = "0x0c - Status register 0"]
    pub stat0: crate::Reg<stat0::STAT0_SPEC>,
    #[doc = "0x10 - Control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x14 - Address register 0"]
    pub addr0: crate::Reg<addr0::ADDR0_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte control register"]
    pub obctl: crate::Reg<obctl::OBCTL_SPEC>,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: crate::Reg<wp::WP_SPEC>,
    _reserved8: [u8; 0x20],
    #[doc = "0x44 - Unlock key register 1"]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x4c - Status register 1"]
    pub stat1: crate::Reg<stat1::STAT1_SPEC>,
    #[doc = "0x50 - Control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    #[doc = "0x54 - Address register 1"]
    pub addr1: crate::Reg<addr1::ADDR1_SPEC>,
    _reserved12: [u8; 0xa4],
    #[doc = "0xfc - Wait state enable register"]
    pub wsen: crate::Reg<wsen::WSEN_SPEC>,
    #[doc = "0x100 - Product ID register"]
    pub pid: crate::Reg<pid::PID_SPEC>,
}
#[doc = "WS register accessor: an alias for `Reg<WS_SPEC>`"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "KEY register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Unlock key register"]
pub mod key;
#[doc = "OBKEY register accessor: an alias for `Reg<OBKEY_SPEC>`"]
pub type OBKEY = crate::Reg<obkey::OBKEY_SPEC>;
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "STAT0 register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "ADDR0 register accessor: an alias for `Reg<ADDR0_SPEC>`"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "OBCTL register accessor: an alias for `Reg<OBCTL_SPEC>`"]
pub type OBCTL = crate::Reg<obctl::OBCTL_SPEC>;
#[doc = "Option byte control register"]
pub mod obctl;
#[doc = "WP register accessor: an alias for `Reg<WP_SPEC>`"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "Unlock key register 1"]
pub mod key1;
#[doc = "STAT1 register accessor: an alias for `Reg<STAT1_SPEC>`"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "Status register 1"]
pub mod stat1;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "ADDR1 register accessor: an alias for `Reg<ADDR1_SPEC>`"]
pub type ADDR1 = crate::Reg<addr1::ADDR1_SPEC>;
#[doc = "Address register 1"]
pub mod addr1;
#[doc = "WSEN register accessor: an alias for `Reg<WSEN_SPEC>`"]
pub type WSEN = crate::Reg<wsen::WSEN_SPEC>;
#[doc = "Wait state enable register"]
pub mod wsen;
#[doc = "PID register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Product ID register"]
pub mod pid;
