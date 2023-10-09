#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state register"]
    pub ws: WS,
    #[doc = "0x04 - Unlock key register"]
    pub key: KEY,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Status register"]
    pub stat: STAT,
    #[doc = "0x10 - Control register"]
    pub ctl: CTL,
    #[doc = "0x14 - Address register"]
    pub addr: ADDR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte control register"]
    pub obstat: OBSTAT,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: WP,
    _reserved8: [u8; 0xdc],
    #[doc = "0x100 - Product ID register"]
    pub pid: PID,
}
#[doc = "WS (rw) register accessor: wait state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ws`]
module"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "wait state register"]
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
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "ADDR (w) register accessor: Address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "OBSTAT (r) register accessor: Option byte control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obstat`]
module"]
pub type OBSTAT = crate::Reg<obstat::OBSTAT_SPEC>;
#[doc = "Option byte control register"]
pub mod obstat;
#[doc = "WP (r) register accessor: Erase/Program Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wp`]
module"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "PID (r) register accessor: Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pid`]
module"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Product ID register"]
pub mod pid;
