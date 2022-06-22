#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - Configuration register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - Transmit data register"]
    pub tdata: crate::Reg<tdata::TDATA_SPEC>,
    #[doc = "0x0c - Rx Data Register"]
    pub rdata: crate::Reg<rdata::RDATA_SPEC>,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x14 - interrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration register"]
pub mod cfg;
#[doc = "TDATA register accessor: an alias for `Reg<TDATA_SPEC>`"]
pub type TDATA = crate::Reg<tdata::TDATA_SPEC>;
#[doc = "Transmit data register"]
pub mod tdata;
#[doc = "RDATA register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "Rx Data Register"]
pub mod rdata;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod intf;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "interrupt enable register"]
pub mod inten;
