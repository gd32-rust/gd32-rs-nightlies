#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x04 - control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: crate::Reg<dmainten::DMAINTEN_SPEC>,
    #[doc = "0x10 - status register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x14 - event generation register"]
    pub swevg: crate::Reg<swevg::SWEVG_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x24 - counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - prescaler"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - auto-reload register"]
    pub car: crate::Reg<car::CAR_SPEC>,
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "DMAINTEN register accessor: an alias for `Reg<DMAINTEN_SPEC>`"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dmainten;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "status register"]
pub mod intf;
#[doc = "SWEVG register accessor: an alias for `Reg<SWEVG_SPEC>`"]
pub type SWEVG = crate::Reg<swevg::SWEVG_SPEC>;
#[doc = "event generation register"]
pub mod swevg;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "CAR register accessor: an alias for `Reg<CAR_SPEC>`"]
pub type CAR = crate::Reg<car::CAR_SPEC>;
#[doc = "auto-reload register"]
pub mod car;
