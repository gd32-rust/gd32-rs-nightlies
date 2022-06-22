#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x04 - Control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    #[doc = "0x08 - Own address register 0"]
    pub saddr0: crate::Reg<saddr0::SADDR0_SPEC>,
    #[doc = "0x0c - Own address register 1"]
    pub saddr1: crate::Reg<saddr1::SADDR1_SPEC>,
    #[doc = "0x10 - Data register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x14 - Transfer status register 0"]
    pub stat0: crate::Reg<stat0::STAT0_SPEC>,
    #[doc = "0x18 - Transfer status register 1"]
    pub stat1: crate::Reg<stat1::STAT1_SPEC>,
    #[doc = "0x1c - Clock configure register"]
    pub ckcfg: crate::Reg<ckcfg::CKCFG_SPEC>,
    #[doc = "0x20 - Rise time register"]
    pub rt: crate::Reg<rt::RT_SPEC>,
    _reserved9: [u8; 0x5c],
    #[doc = "0x80 - SAM control and status register"]
    pub samcs: crate::Reg<samcs::SAMCS_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x90 - Fast-mode-plus configure register"]
    pub fmpcfg: crate::Reg<fmpcfg::FMPCFG_SPEC>,
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "SADDR0 register accessor: an alias for `Reg<SADDR0_SPEC>`"]
pub type SADDR0 = crate::Reg<saddr0::SADDR0_SPEC>;
#[doc = "Own address register 0"]
pub mod saddr0;
#[doc = "SADDR1 register accessor: an alias for `Reg<SADDR1_SPEC>`"]
pub type SADDR1 = crate::Reg<saddr1::SADDR1_SPEC>;
#[doc = "Own address register 1"]
pub mod saddr1;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
#[doc = "STAT0 register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Transfer status register 0"]
pub mod stat0;
#[doc = "STAT1 register accessor: an alias for `Reg<STAT1_SPEC>`"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "Transfer status register 1"]
pub mod stat1;
#[doc = "CKCFG register accessor: an alias for `Reg<CKCFG_SPEC>`"]
pub type CKCFG = crate::Reg<ckcfg::CKCFG_SPEC>;
#[doc = "Clock configure register"]
pub mod ckcfg;
#[doc = "RT register accessor: an alias for `Reg<RT_SPEC>`"]
pub type RT = crate::Reg<rt::RT_SPEC>;
#[doc = "Rise time register"]
pub mod rt;
#[doc = "SAMCS register accessor: an alias for `Reg<SAMCS_SPEC>`"]
pub type SAMCS = crate::Reg<samcs::SAMCS_SPEC>;
#[doc = "SAM control and status register"]
pub mod samcs;
#[doc = "FMPCFG register accessor: an alias for `Reg<FMPCFG_SPEC>`"]
pub type FMPCFG = crate::Reg<fmpcfg::FMPCFG_SPEC>;
#[doc = "Fast-mode-plus configure register"]
pub mod fmpcfg;
