#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub ctl: CTL,
    #[doc = "0x04 - interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x08 - interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x0c - interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x10 - Pin hysteresis mode register"]
    pub phm: PHM,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - I/O analog switch register"]
    pub asw: ASW,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - I/O sample configuration register"]
    pub sampcfg: SAMPCFG,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - I/O channel configuration register"]
    pub chcfg: CHCFG,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - I/O group control register"]
    pub gctl: GCTL,
    #[doc = "0x34 - I/O group x cycle number register"]
    pub g0cycn: G0CYCN,
    #[doc = "0x38 - I/O group x cycle number register"]
    pub g1cycn: G1CYCN,
    #[doc = "0x3c - I/O group x cycle number register"]
    pub g2cycn: G2CYCN,
    #[doc = "0x40 - I/O group x cycle number register"]
    pub g3cycn: G3CYCN,
    #[doc = "0x44 - I/O group x cycle number register"]
    pub g4cycn: G4CYCN,
    #[doc = "0x48 - I/O group x cycle number register"]
    pub g5cycn: G5CYCN,
}
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "INTEN (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "interrupt enable register"]
pub mod inten;
#[doc = "INTC (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod intc;
#[doc = "INTF (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "PHM (rw) register accessor: Pin hysteresis mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`phm`]
module"]
pub type PHM = crate::Reg<phm::PHM_SPEC>;
#[doc = "Pin hysteresis mode register"]
pub mod phm;
#[doc = "ASW (rw) register accessor: I/O analog switch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`asw`]
module"]
pub type ASW = crate::Reg<asw::ASW_SPEC>;
#[doc = "I/O analog switch register"]
pub mod asw;
#[doc = "SAMPCFG (rw) register accessor: I/O sample configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sampcfg`]
module"]
pub type SAMPCFG = crate::Reg<sampcfg::SAMPCFG_SPEC>;
#[doc = "I/O sample configuration register"]
pub mod sampcfg;
#[doc = "CHCFG (rw) register accessor: I/O channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chcfg`]
module"]
pub type CHCFG = crate::Reg<chcfg::CHCFG_SPEC>;
#[doc = "I/O channel configuration register"]
pub mod chcfg;
#[doc = "GCTL (rw) register accessor: I/O group control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gctl`]
module"]
pub type GCTL = crate::Reg<gctl::GCTL_SPEC>;
#[doc = "I/O group control register"]
pub mod gctl;
#[doc = "G0CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g0cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`g0cycn`]
module"]
pub type G0CYCN = crate::Reg<g0cycn::G0CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g0cycn;
#[doc = "G1CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g1cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`g1cycn`]
module"]
pub type G1CYCN = crate::Reg<g1cycn::G1CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g1cycn;
#[doc = "G2CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g2cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`g2cycn`]
module"]
pub type G2CYCN = crate::Reg<g2cycn::G2CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g2cycn;
#[doc = "G3CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g3cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`g3cycn`]
module"]
pub type G3CYCN = crate::Reg<g3cycn::G3CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g3cycn;
#[doc = "G4CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g4cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`g4cycn`]
module"]
pub type G4CYCN = crate::Reg<g4cycn::G4CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g4cycn;
#[doc = "G5CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g5cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`g5cycn`]
module"]
pub type G5CYCN = crate::Reg<g5cycn::G5CYCN_SPEC>;
#[doc = "I/O group x cycle number register"]
pub mod g5cycn;
