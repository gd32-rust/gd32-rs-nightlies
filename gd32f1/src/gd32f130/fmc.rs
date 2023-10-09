#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wait state register"]
    pub ws: WS,
    #[doc = "0x04 - Flash unlock key register"]
    pub key: KEY,
    #[doc = "0x08 - Flash option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Flash status register"]
    pub stat: STAT,
    #[doc = "0x10 - Flash control register"]
    pub ctl: CTL,
    #[doc = "0x14 - Flash address register"]
    pub addr: ADDR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte status register"]
    pub obstat: OBSTAT,
    #[doc = "0x20 - Write protection register"]
    pub wp: WP,
    _reserved8: [u8; 0xd8],
    #[doc = "0xfc - Flash wait state control register"]
    pub wsen: WSEN,
    #[doc = "0x100 - Flash Product ID register"]
    pub pid: PID,
}
#[doc = "WS (rw) register accessor: Wait state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ws`]
module"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "Wait state register"]
pub mod ws;
#[doc = "KEY (w) register accessor: Flash unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key`]
module"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "Flash unlock key register"]
pub mod key;
#[doc = "OBKEY (w) register accessor: Flash option byte unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obkey`]
module"]
pub type OBKEY = crate::Reg<obkey::OBKEY_SPEC>;
#[doc = "Flash option byte unlock key register"]
pub mod obkey;
#[doc = "STAT (rw) register accessor: Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Flash status register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: Flash control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Flash control register"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: Flash address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Flash address register"]
pub mod addr;
#[doc = "OBSTAT (r) register accessor: Option byte status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obstat`]
module"]
pub type OBSTAT = crate::Reg<obstat::OBSTAT_SPEC>;
#[doc = "Option byte status register"]
pub mod obstat;
#[doc = "WP (r) register accessor: Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wp`]
module"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Write protection register"]
pub mod wp;
#[doc = "WSEN (r) register accessor: Flash wait state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wsen`]
module"]
pub type WSEN = crate::Reg<wsen::WSEN_SPEC>;
#[doc = "Flash wait state control register"]
pub mod wsen;
#[doc = "PID (r) register accessor: Flash Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pid`]
module"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Flash Product ID register"]
pub mod pid;
