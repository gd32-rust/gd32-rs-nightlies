#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state counter register"]
    pub ws: WS,
    #[doc = "0x04 - Unlock key register"]
    pub key: KEY,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Status register 0"]
    pub stat0: STAT0,
    #[doc = "0x10 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x14 - Address register 0"]
    pub addr0: ADDR0,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte control register"]
    pub obctl: OBCTL,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: WP,
    _reserved8: [u8; 0x20],
    #[doc = "0x44 - Unlock key register 1"]
    pub key1: KEY1,
    _reserved9: [u8; 0x04],
    #[doc = "0x4c - Status register 1"]
    pub stat1: STAT1,
    #[doc = "0x50 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x54 - Address register 1"]
    pub addr1: ADDR1,
    _reserved12: [u8; 0xa4],
    #[doc = "0xfc - Wait state enable register"]
    pub wsen: WSEN,
    #[doc = "0x100 - Product ID register"]
    pub pid: PID,
}
#[doc = "WS (rw) register accessor: wait state counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ws`]
module"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "KEY (w) register accessor: Unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key`]
module"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Unlock key register"]
pub mod key;
#[doc = "OBKEY (w) register accessor: Option byte unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obkey`]
module"]
pub type OBKEY = crate::Reg<obkey::OBKEY_SPEC>;
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "STAT0 (rw) register accessor: Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat0`]
module"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "ADDR0 (w) register accessor: Address register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr0`]
module"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "OBCTL (r) register accessor: Option byte control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obctl`]
module"]
pub type OBCTL = crate::Reg<obctl::OBCTL_SPEC>;
#[doc = "Option byte control register"]
pub mod obctl;
#[doc = "WP (r) register accessor: Erase/Program Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wp`]
module"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "KEY1 (w) register accessor: Unlock key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key1`]
module"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "Unlock key register 1"]
pub mod key1;
#[doc = "STAT1 (rw) register accessor: Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat1`]
module"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "Status register 1"]
pub mod stat1;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "ADDR1 (w) register accessor: Address register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr1`]
module"]
pub type ADDR1 = crate::Reg<addr1::ADDR1_SPEC>;
#[doc = "Address register 1"]
pub mod addr1;
#[doc = "WSEN (rw) register accessor: Wait state enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wsen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wsen`]
module"]
pub type WSEN = crate::Reg<wsen::WSEN_SPEC>;
#[doc = "Wait state enable register"]
pub mod wsen;
#[doc = "PID (r) register accessor: Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pid`]
module"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Product ID register"]
pub mod pid;
