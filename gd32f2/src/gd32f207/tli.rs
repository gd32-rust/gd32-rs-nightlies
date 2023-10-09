#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Synchronous pulse size register"]
    pub spsz: SPSZ,
    #[doc = "0x0c - Back-porch size register"]
    pub bpsz: BPSZ,
    #[doc = "0x10 - Active size register"]
    pub asz: ASZ,
    #[doc = "0x14 - Total size register"]
    pub tsz: TSZ,
    #[doc = "0x18 - Control register"]
    pub ctl: CTL,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Reload layer register"]
    pub rl: RL,
    _reserved6: [u8; 0x04],
    #[doc = "0x2c - Background color register"]
    pub bgc: BGC,
    _reserved7: [u8; 0x04],
    #[doc = "0x34 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x38 - Interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x3c - Interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x40 - Line mark register"]
    pub lm: LM,
    #[doc = "0x44 - Current pixel position register"]
    pub cppos: CPPOS,
    #[doc = "0x48 - Status register"]
    pub stat: STAT,
    _reserved13: [u8; 0x38],
    #[doc = "0x84 - Layer 0 control register"]
    pub l0ctl: L0CTL,
    #[doc = "0x88 - Layer 0 horizontal position parameters register"]
    pub l0hpos: L0HPOS,
    #[doc = "0x8c - Layer 0 vertical position parameters register"]
    pub l0vpos: L0VPOS,
    #[doc = "0x90 - Layer 0 color key register"]
    pub l0ckey: L0CKEY,
    #[doc = "0x94 - Layer 0 packeted pixel format register"]
    pub l0ppf: L0PPF,
    #[doc = "0x98 - Layer 0 specified alpha register"]
    pub l0sa: L0SA,
    #[doc = "0x9c - Layer 0 default color register"]
    pub l0dc: L0DC,
    #[doc = "0xa0 - Layer 0 blending register"]
    pub l0blend: L0BLEND,
    _reserved21: [u8; 0x08],
    #[doc = "0xac - Layer 0 frame base address register"]
    pub l0fbaddr: L0FBADDR,
    #[doc = "0xb0 - Layer 0 frame line length register"]
    pub l0fllen: L0FLLEN,
    #[doc = "0xb4 - Layer 0 frame total line number register"]
    pub l0ftln: L0FTLN,
    _reserved24: [u8; 0x0c],
    #[doc = "0xc4 - Layer 0 look up table register"]
    pub l0lut: L0LUT,
    _reserved25: [u8; 0x3c],
    #[doc = "0x104 - Layer 1 control register"]
    pub l1ctl: L1CTL,
    #[doc = "0x108 - Layer 1 horizontal position parameters register"]
    pub l1hpos: L1HPOS,
    #[doc = "0x10c - Layer 1 vertical position parameters register"]
    pub l1vpos: L1VPOS,
    #[doc = "0x110 - Layer 1 color key register"]
    pub l1ckey: L1CKEY,
    #[doc = "0x114 - Layer 1 packeted pixel format register"]
    pub l1ppf: L1PPF,
    #[doc = "0x118 - Layer 1 specified alpha register"]
    pub l1sa: L1SA,
    #[doc = "0x11c - Layer 1 default color register"]
    pub l1dc: L1DC,
    #[doc = "0x120 - Layer 1 blending register"]
    pub l1blend: L1BLEND,
    _reserved33: [u8; 0x08],
    #[doc = "0x12c - Layer 1 frame base address register"]
    pub l1fbaddr: L1FBADDR,
    #[doc = "0x130 - Layer 1 frame line length register"]
    pub l1fllen: L1FLLEN,
    #[doc = "0x134 - Layer 1 frame total line number register"]
    pub l1ftln: L1FTLN,
    _reserved36: [u8; 0x0c],
    #[doc = "0x144 - Layer 1 look up table register"]
    pub l1lut: L1LUT,
}
#[doc = "SPSZ (rw) register accessor: Synchronous pulse size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spsz`]
module"]
pub type SPSZ = crate::Reg<spsz::SPSZ_SPEC>;
#[doc = "Synchronous pulse size register"]
pub mod spsz;
#[doc = "BPSZ (rw) register accessor: Back-porch size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpsz`]
module"]
pub type BPSZ = crate::Reg<bpsz::BPSZ_SPEC>;
#[doc = "Back-porch size register"]
pub mod bpsz;
#[doc = "ASZ (rw) register accessor: Active size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`asz`]
module"]
pub type ASZ = crate::Reg<asz::ASZ_SPEC>;
#[doc = "Active size register"]
pub mod asz;
#[doc = "TSZ (rw) register accessor: Total size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tsz`]
module"]
pub type TSZ = crate::Reg<tsz::TSZ_SPEC>;
#[doc = "Total size register"]
pub mod tsz;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "RL (rw) register accessor: Reload layer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rl`]
module"]
pub type RL = crate::Reg<rl::RL_SPEC>;
#[doc = "Reload layer register"]
pub mod rl;
#[doc = "BGC (rw) register accessor: Background color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bgc`]
module"]
pub type BGC = crate::Reg<bgc::BGC_SPEC>;
#[doc = "Background color register"]
pub mod bgc;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "INTF (r) register accessor: Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "LM (rw) register accessor: Line mark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lm`]
module"]
pub type LM = crate::Reg<lm::LM_SPEC>;
#[doc = "Line mark register"]
pub mod lm;
#[doc = "CPPOS (r) register accessor: Current pixel position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cppos::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cppos`]
module"]
pub type CPPOS = crate::Reg<cppos::CPPOS_SPEC>;
#[doc = "Current pixel position register"]
pub mod cppos;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "L0CTL (rw) register accessor: Layer 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0ctl`]
module"]
pub type L0CTL = crate::Reg<l0ctl::L0CTL_SPEC>;
#[doc = "Layer 0 control register"]
pub mod l0ctl;
#[doc = "L1CTL (rw) register accessor: Layer 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1ctl`]
module"]
pub type L1CTL = crate::Reg<l1ctl::L1CTL_SPEC>;
#[doc = "Layer 1 control register"]
pub mod l1ctl;
#[doc = "L0HPOS (rw) register accessor: Layer 0 horizontal position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0hpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0hpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0hpos`]
module"]
pub type L0HPOS = crate::Reg<l0hpos::L0HPOS_SPEC>;
#[doc = "Layer 0 horizontal position parameters register"]
pub mod l0hpos;
#[doc = "L1HPOS (rw) register accessor: Layer 1 horizontal position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1hpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1hpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1hpos`]
module"]
pub type L1HPOS = crate::Reg<l1hpos::L1HPOS_SPEC>;
#[doc = "Layer 1 horizontal position parameters register"]
pub mod l1hpos;
#[doc = "L0VPOS (rw) register accessor: Layer 0 vertical position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0vpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0vpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0vpos`]
module"]
pub type L0VPOS = crate::Reg<l0vpos::L0VPOS_SPEC>;
#[doc = "Layer 0 vertical position parameters register"]
pub mod l0vpos;
#[doc = "L1VPOS (rw) register accessor: Layer 1 vertical position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1vpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1vpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1vpos`]
module"]
pub type L1VPOS = crate::Reg<l1vpos::L1VPOS_SPEC>;
#[doc = "Layer 1 vertical position parameters register"]
pub mod l1vpos;
#[doc = "L0CKEY (rw) register accessor: Layer 0 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ckey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ckey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0ckey`]
module"]
pub type L0CKEY = crate::Reg<l0ckey::L0CKEY_SPEC>;
#[doc = "Layer 0 color key register"]
pub mod l0ckey;
#[doc = "L1CKEY (rw) register accessor: Layer 1 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ckey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ckey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1ckey`]
module"]
pub type L1CKEY = crate::Reg<l1ckey::L1CKEY_SPEC>;
#[doc = "Layer 1 color key register"]
pub mod l1ckey;
#[doc = "L0PPF (rw) register accessor: Layer 0 packeted pixel format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ppf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ppf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0ppf`]
module"]
pub type L0PPF = crate::Reg<l0ppf::L0PPF_SPEC>;
#[doc = "Layer 0 packeted pixel format register"]
pub mod l0ppf;
#[doc = "L1PPF (rw) register accessor: Layer 1 packeted pixel format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ppf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ppf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1ppf`]
module"]
pub type L1PPF = crate::Reg<l1ppf::L1PPF_SPEC>;
#[doc = "Layer 1 packeted pixel format register"]
pub mod l1ppf;
#[doc = "L0SA (rw) register accessor: Layer 0 specified alpha register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0sa`]
module"]
pub type L0SA = crate::Reg<l0sa::L0SA_SPEC>;
#[doc = "Layer 0 specified alpha register"]
pub mod l0sa;
#[doc = "L1SA (rw) register accessor: Layer 1 specified alpha register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1sa`]
module"]
pub type L1SA = crate::Reg<l1sa::L1SA_SPEC>;
#[doc = "Layer 1 specified alpha register"]
pub mod l1sa;
#[doc = "L0DC (rw) register accessor: Layer 0 default color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0dc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0dc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0dc`]
module"]
pub type L0DC = crate::Reg<l0dc::L0DC_SPEC>;
#[doc = "Layer 0 default color register"]
pub mod l0dc;
#[doc = "L1DC (rw) register accessor: Layer 1 default color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1dc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1dc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1dc`]
module"]
pub type L1DC = crate::Reg<l1dc::L1DC_SPEC>;
#[doc = "Layer 1 default color register"]
pub mod l1dc;
#[doc = "L0BLEND (rw) register accessor: Layer 0 blending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0blend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0blend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0blend`]
module"]
pub type L0BLEND = crate::Reg<l0blend::L0BLEND_SPEC>;
#[doc = "Layer 0 blending register"]
pub mod l0blend;
#[doc = "L1BLEND (rw) register accessor: Layer 1 blending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1blend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1blend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1blend`]
module"]
pub type L1BLEND = crate::Reg<l1blend::L1BLEND_SPEC>;
#[doc = "Layer 1 blending register"]
pub mod l1blend;
#[doc = "L0FBADDR (rw) register accessor: Layer 0 frame base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0fbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0fbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0fbaddr`]
module"]
pub type L0FBADDR = crate::Reg<l0fbaddr::L0FBADDR_SPEC>;
#[doc = "Layer 0 frame base address register"]
pub mod l0fbaddr;
#[doc = "L1FBADDR (rw) register accessor: Layer 1 frame base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1fbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1fbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1fbaddr`]
module"]
pub type L1FBADDR = crate::Reg<l1fbaddr::L1FBADDR_SPEC>;
#[doc = "Layer 1 frame base address register"]
pub mod l1fbaddr;
#[doc = "L0FLLEN (rw) register accessor: Layer 0 frame line length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0fllen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0fllen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0fllen`]
module"]
pub type L0FLLEN = crate::Reg<l0fllen::L0FLLEN_SPEC>;
#[doc = "Layer 0 frame line length register"]
pub mod l0fllen;
#[doc = "L1FLLEN (rw) register accessor: Layer 1 frame line length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1fllen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1fllen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1fllen`]
module"]
pub type L1FLLEN = crate::Reg<l1fllen::L1FLLEN_SPEC>;
#[doc = "Layer 1 frame line length register"]
pub mod l1fllen;
#[doc = "L0FTLN (rw) register accessor: Layer 0 frame total line number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ftln::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ftln::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0ftln`]
module"]
pub type L0FTLN = crate::Reg<l0ftln::L0FTLN_SPEC>;
#[doc = "Layer 0 frame total line number register"]
pub mod l0ftln;
#[doc = "L1FTLN (rw) register accessor: Layer 1 frame total line number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ftln::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ftln::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1ftln`]
module"]
pub type L1FTLN = crate::Reg<l1ftln::L1FTLN_SPEC>;
#[doc = "Layer 1 frame total line number register"]
pub mod l1ftln;
#[doc = "L0LUT (w) register accessor: Layer 0 look up table register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0lut::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l0lut`]
module"]
pub type L0LUT = crate::Reg<l0lut::L0LUT_SPEC>;
#[doc = "Layer 0 look up table register"]
pub mod l0lut;
#[doc = "L1LUT (w) register accessor: Layer 1 look up table register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1lut::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`l1lut`]
module"]
pub type L1LUT = crate::Reg<l1lut::L1LUT_SPEC>;
#[doc = "Layer 1 look up table register"]
pub mod l1lut;
