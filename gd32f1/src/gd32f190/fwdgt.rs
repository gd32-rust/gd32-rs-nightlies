#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Prescaler register"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x08 - Reload register"]
    pub rld: crate::Reg<rld::RLD_SPEC>,
    #[doc = "0x0c - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x10 - Window register"]
    pub wnd: crate::Reg<wnd::WND_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "RLD register accessor: an alias for `Reg<RLD_SPEC>`"]
pub type RLD = crate::Reg<rld::RLD_SPEC>;
#[doc = "Reload register"]
pub mod rld;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "WND register accessor: an alias for `Reg<WND_SPEC>`"]
pub type WND = crate::Reg<wnd::WND_SPEC>;
#[doc = "Window register"]
pub mod wnd;
