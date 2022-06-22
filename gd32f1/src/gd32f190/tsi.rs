#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - interrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x08 - interrupt flag clear register"]
    pub intc: crate::Reg<intc::INTC_SPEC>,
    #[doc = "0x0c - interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x10 - Pin hysteresis mode register"]
    pub phm: crate::Reg<phm::PHM_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - I/O analog switch register"]
    pub asw: crate::Reg<asw::ASW_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - I/O sample configuration register"]
    pub sampcfg: crate::Reg<sampcfg::SAMPCFG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - I/O channel configuration register"]
    pub chcfg: crate::Reg<chcfg::CHCFG_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - I/O group control register"]
    pub gctl: crate::Reg<gctl::GCTL_SPEC>,
    #[doc = "0x34 - I/O group x cycle number register"]
    pub g0cycn: crate::Reg<g0cycn::G0CYCN_SPEC>,
    #[doc = "0x38 - I/O group x cycle number register"]
    pub g1cycn: crate::Reg<g1cycn::G1CYCN_SPEC>,
    #[doc = "0x3c - I/O group x cycle number register"]
    pub g2cycn: crate::Reg<g2cycn::G2CYCN_SPEC>,
    #[doc = "0x40 - I/O group x cycle number register"]
    pub g3cycn: crate::Reg<g3cycn::G3CYCN_SPEC>,
    #[doc = "0x44 - I/O group x cycle number register"]
    pub g4cycn: crate::Reg<g4cycn::G4CYCN_SPEC>,
    #[doc = "0x48 - I/O group x cycle number register"]
    pub g5cycn: crate::Reg<g5cycn::G5CYCN_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "interrupt enable register"]
pub mod inten;
#[doc = "INTC register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod intc;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "PHM register accessor: an alias for `Reg<PHM_SPEC>`"]
pub type PHM = crate::Reg<phm::PHM_SPEC>;
#[doc = "Pin hysteresis mode register"]
pub mod phm;
#[doc = "ASW register accessor: an alias for `Reg<ASW_SPEC>`"]
pub type ASW = crate::Reg<asw::ASW_SPEC>;
#[doc = "I/O analog switch register"]
pub mod asw;
#[doc = "SAMPCFG register accessor: an alias for `Reg<SAMPCFG_SPEC>`"]
pub type SAMPCFG = crate::Reg<sampcfg::SAMPCFG_SPEC>;
#[doc = "I/O sample configuration register"]
pub mod sampcfg;
#[doc = "CHCFG register accessor: an alias for `Reg<CHCFG_SPEC>`"]
pub type CHCFG = crate::Reg<chcfg::CHCFG_SPEC>;
#[doc = "I/O channel configuration register"]
pub mod chcfg;
#[doc = "GCTL register accessor: an alias for `Reg<GCTL_SPEC>`"]
pub type GCTL = crate::Reg<gctl::GCTL_SPEC>;
#[doc = "I/O group control register"]
pub mod gctl;
#[doc = "G0CYCN register accessor: an alias for `Reg<G0CYCN_SPEC>`"]
pub type G0CYCN = crate::Reg<g0cycn::G0CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g0cycn;
#[doc = "G1CYCN register accessor: an alias for `Reg<G1CYCN_SPEC>`"]
pub type G1CYCN = crate::Reg<g1cycn::G1CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g1cycn;
#[doc = "G2CYCN register accessor: an alias for `Reg<G2CYCN_SPEC>`"]
pub type G2CYCN = crate::Reg<g2cycn::G2CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g2cycn;
#[doc = "G3CYCN register accessor: an alias for `Reg<G3CYCN_SPEC>`"]
pub type G3CYCN = crate::Reg<g3cycn::G3CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g3cycn;
#[doc = "G4CYCN register accessor: an alias for `Reg<G4CYCN_SPEC>`"]
pub type G4CYCN = crate::Reg<g4cycn::G4CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g4cycn;
#[doc = "G5CYCN register accessor: an alias for `Reg<G5CYCN_SPEC>`"]
pub type G5CYCN = crate::Reg<g5cycn::G5CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g5cycn;
