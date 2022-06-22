#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Synchronous pulse size register"]
    pub spsz: crate::Reg<spsz::SPSZ_SPEC>,
    #[doc = "0x0c - Back-porch size register"]
    pub bpsz: crate::Reg<bpsz::BPSZ_SPEC>,
    #[doc = "0x10 - Active size register"]
    pub asz: crate::Reg<asz::ASZ_SPEC>,
    #[doc = "0x14 - Total size register"]
    pub tsz: crate::Reg<tsz::TSZ_SPEC>,
    #[doc = "0x18 - Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Reload layer register"]
    pub rl: crate::Reg<rl::RL_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x2c - Background color register"]
    pub bgc: crate::Reg<bgc::BGC_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x34 - Interrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x38 - Interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x3c - Interrupt flag clear register"]
    pub intc: crate::Reg<intc::INTC_SPEC>,
    #[doc = "0x40 - Line mark register"]
    pub lm: crate::Reg<lm::LM_SPEC>,
    #[doc = "0x44 - Current pixel position register"]
    pub cppos: crate::Reg<cppos::CPPOS_SPEC>,
    #[doc = "0x48 - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved13: [u8; 0x38],
    #[doc = "0x84 - Layer 0 control register"]
    pub l0ctl: crate::Reg<l0ctl::L0CTL_SPEC>,
    #[doc = "0x88 - Layer 0 horizontal position parameters register"]
    pub l0hpos: crate::Reg<l0hpos::L0HPOS_SPEC>,
    #[doc = "0x8c - Layer 0 vertical position parameters register"]
    pub l0vpos: crate::Reg<l0vpos::L0VPOS_SPEC>,
    #[doc = "0x90 - Layer 0 color key register"]
    pub l0ckey: crate::Reg<l0ckey::L0CKEY_SPEC>,
    #[doc = "0x94 - Layer 0 packeted pixel format register"]
    pub l0ppf: crate::Reg<l0ppf::L0PPF_SPEC>,
    #[doc = "0x98 - Layer 0 specified alpha register"]
    pub l0sa: crate::Reg<l0sa::L0SA_SPEC>,
    #[doc = "0x9c - Layer 0 default color register"]
    pub l0dc: crate::Reg<l0dc::L0DC_SPEC>,
    #[doc = "0xa0 - Layer 0 blending register"]
    pub l0blend: crate::Reg<l0blend::L0BLEND_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0xac - Layer 0 frame base address register"]
    pub l0fbaddr: crate::Reg<l0fbaddr::L0FBADDR_SPEC>,
    #[doc = "0xb0 - Layer 0 frame line length register"]
    pub l0fllen: crate::Reg<l0fllen::L0FLLEN_SPEC>,
    #[doc = "0xb4 - Layer 0 frame total line number register"]
    pub l0ftln: crate::Reg<l0ftln::L0FTLN_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0xc4 - Layer 0 look up table register"]
    pub l0lut: crate::Reg<l0lut::L0LUT_SPEC>,
    _reserved25: [u8; 0x3c],
    #[doc = "0x104 - Layer 1 control register"]
    pub l1ctl: crate::Reg<l1ctl::L1CTL_SPEC>,
    #[doc = "0x108 - Layer 1 horizontal position parameters register"]
    pub l1hpos: crate::Reg<l1hpos::L1HPOS_SPEC>,
    #[doc = "0x10c - Layer 1 vertical position parameters register"]
    pub l1vpos: crate::Reg<l1vpos::L1VPOS_SPEC>,
    #[doc = "0x110 - Layer 1 color key register"]
    pub l1ckey: crate::Reg<l1ckey::L1CKEY_SPEC>,
    #[doc = "0x114 - Layer 1 packeted pixel format register"]
    pub l1ppf: crate::Reg<l1ppf::L1PPF_SPEC>,
    #[doc = "0x118 - Layer 1 specified alpha register"]
    pub l1sa: crate::Reg<l1sa::L1SA_SPEC>,
    #[doc = "0x11c - Layer 1 default color register"]
    pub l1dc: crate::Reg<l1dc::L1DC_SPEC>,
    #[doc = "0x120 - Layer 1 blending register"]
    pub l1blend: crate::Reg<l1blend::L1BLEND_SPEC>,
    _reserved33: [u8; 0x08],
    #[doc = "0x12c - Layer 1 frame base address register"]
    pub l1fbaddr: crate::Reg<l1fbaddr::L1FBADDR_SPEC>,
    #[doc = "0x130 - Layer 1 frame line length register"]
    pub l1fllen: crate::Reg<l1fllen::L1FLLEN_SPEC>,
    #[doc = "0x134 - Layer 1 frame total line number register"]
    pub l1ftln: crate::Reg<l1ftln::L1FTLN_SPEC>,
    _reserved36: [u8; 0x0c],
    #[doc = "0x144 - Layer 1 look up table register"]
    pub l1lut: crate::Reg<l1lut::L1LUT_SPEC>,
}
#[doc = "SPSZ register accessor: an alias for `Reg<SPSZ_SPEC>`"]
pub type SPSZ = crate::Reg<spsz::SPSZ_SPEC>;
#[doc = "Synchronous pulse size register"]
pub mod spsz;
#[doc = "BPSZ register accessor: an alias for `Reg<BPSZ_SPEC>`"]
pub type BPSZ = crate::Reg<bpsz::BPSZ_SPEC>;
#[doc = "Back-porch size register"]
pub mod bpsz;
#[doc = "ASZ register accessor: an alias for `Reg<ASZ_SPEC>`"]
pub type ASZ = crate::Reg<asz::ASZ_SPEC>;
#[doc = "Active size register"]
pub mod asz;
#[doc = "TSZ register accessor: an alias for `Reg<TSZ_SPEC>`"]
pub type TSZ = crate::Reg<tsz::TSZ_SPEC>;
#[doc = "Total size register"]
pub mod tsz;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "RL register accessor: an alias for `Reg<RL_SPEC>`"]
pub type RL = crate::Reg<rl::RL_SPEC>;
#[doc = "Reload layer register"]
pub mod rl;
#[doc = "BGC register accessor: an alias for `Reg<BGC_SPEC>`"]
pub type BGC = crate::Reg<bgc::BGC_SPEC>;
#[doc = "Background color register"]
pub mod bgc;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "INTC register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "LM register accessor: an alias for `Reg<LM_SPEC>`"]
pub type LM = crate::Reg<lm::LM_SPEC>;
#[doc = "Line mark register"]
pub mod lm;
#[doc = "CPPOS register accessor: an alias for `Reg<CPPOS_SPEC>`"]
pub type CPPOS = crate::Reg<cppos::CPPOS_SPEC>;
#[doc = "Current pixel position register"]
pub mod cppos;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "L0CTL register accessor: an alias for `Reg<L0CTL_SPEC>`"]
pub type L0CTL = crate::Reg<l0ctl::L0CTL_SPEC>;
#[doc = "Layer 0 control register"]
pub mod l0ctl;
#[doc = "L1CTL register accessor: an alias for `Reg<L1CTL_SPEC>`"]
pub type L1CTL = crate::Reg<l1ctl::L1CTL_SPEC>;
#[doc = "Layer 1 control register"]
pub mod l1ctl;
#[doc = "L0HPOS register accessor: an alias for `Reg<L0HPOS_SPEC>`"]
pub type L0HPOS = crate::Reg<l0hpos::L0HPOS_SPEC>;
#[doc = "Layer 0 horizontal position parameters register"]
pub mod l0hpos;
#[doc = "L1HPOS register accessor: an alias for `Reg<L1HPOS_SPEC>`"]
pub type L1HPOS = crate::Reg<l1hpos::L1HPOS_SPEC>;
#[doc = "Layer 1 horizontal position parameters register"]
pub mod l1hpos;
#[doc = "L0VPOS register accessor: an alias for `Reg<L0VPOS_SPEC>`"]
pub type L0VPOS = crate::Reg<l0vpos::L0VPOS_SPEC>;
#[doc = "Layer 0 vertical position parameters register"]
pub mod l0vpos;
#[doc = "L1VPOS register accessor: an alias for `Reg<L1VPOS_SPEC>`"]
pub type L1VPOS = crate::Reg<l1vpos::L1VPOS_SPEC>;
#[doc = "Layer 1 vertical position parameters register"]
pub mod l1vpos;
#[doc = "L0CKEY register accessor: an alias for `Reg<L0CKEY_SPEC>`"]
pub type L0CKEY = crate::Reg<l0ckey::L0CKEY_SPEC>;
#[doc = "Layer 0 color key register"]
pub mod l0ckey;
#[doc = "L1CKEY register accessor: an alias for `Reg<L1CKEY_SPEC>`"]
pub type L1CKEY = crate::Reg<l1ckey::L1CKEY_SPEC>;
#[doc = "Layer 1 color key register"]
pub mod l1ckey;
#[doc = "L0PPF register accessor: an alias for `Reg<L0PPF_SPEC>`"]
pub type L0PPF = crate::Reg<l0ppf::L0PPF_SPEC>;
#[doc = "Layer 0 packeted pixel format register"]
pub mod l0ppf;
#[doc = "L1PPF register accessor: an alias for `Reg<L1PPF_SPEC>`"]
pub type L1PPF = crate::Reg<l1ppf::L1PPF_SPEC>;
#[doc = "Layer 1 packeted pixel format register"]
pub mod l1ppf;
#[doc = "L0SA register accessor: an alias for `Reg<L0SA_SPEC>`"]
pub type L0SA = crate::Reg<l0sa::L0SA_SPEC>;
#[doc = "Layer 0 specified alpha register"]
pub mod l0sa;
#[doc = "L1SA register accessor: an alias for `Reg<L1SA_SPEC>`"]
pub type L1SA = crate::Reg<l1sa::L1SA_SPEC>;
#[doc = "Layer 1 specified alpha register"]
pub mod l1sa;
#[doc = "L0DC register accessor: an alias for `Reg<L0DC_SPEC>`"]
pub type L0DC = crate::Reg<l0dc::L0DC_SPEC>;
#[doc = "Layer 0 default color register"]
pub mod l0dc;
#[doc = "L1DC register accessor: an alias for `Reg<L1DC_SPEC>`"]
pub type L1DC = crate::Reg<l1dc::L1DC_SPEC>;
#[doc = "Layer 1 default color register"]
pub mod l1dc;
#[doc = "L0BLEND register accessor: an alias for `Reg<L0BLEND_SPEC>`"]
pub type L0BLEND = crate::Reg<l0blend::L0BLEND_SPEC>;
#[doc = "Layer 0 blending register"]
pub mod l0blend;
#[doc = "L1BLEND register accessor: an alias for `Reg<L1BLEND_SPEC>`"]
pub type L1BLEND = crate::Reg<l1blend::L1BLEND_SPEC>;
#[doc = "Layer 1 blending register"]
pub mod l1blend;
#[doc = "L0FBADDR register accessor: an alias for `Reg<L0FBADDR_SPEC>`"]
pub type L0FBADDR = crate::Reg<l0fbaddr::L0FBADDR_SPEC>;
#[doc = "Layer 0 frame base address register"]
pub mod l0fbaddr;
#[doc = "L1FBADDR register accessor: an alias for `Reg<L1FBADDR_SPEC>`"]
pub type L1FBADDR = crate::Reg<l1fbaddr::L1FBADDR_SPEC>;
#[doc = "Layer 1 frame base address register"]
pub mod l1fbaddr;
#[doc = "L0FLLEN register accessor: an alias for `Reg<L0FLLEN_SPEC>`"]
pub type L0FLLEN = crate::Reg<l0fllen::L0FLLEN_SPEC>;
#[doc = "Layer 0 frame line length register"]
pub mod l0fllen;
#[doc = "L1FLLEN register accessor: an alias for `Reg<L1FLLEN_SPEC>`"]
pub type L1FLLEN = crate::Reg<l1fllen::L1FLLEN_SPEC>;
#[doc = "Layer 1 frame line length register"]
pub mod l1fllen;
#[doc = "L0FTLN register accessor: an alias for `Reg<L0FTLN_SPEC>`"]
pub type L0FTLN = crate::Reg<l0ftln::L0FTLN_SPEC>;
#[doc = "Layer 0 frame total line number register"]
pub mod l0ftln;
#[doc = "L1FTLN register accessor: an alias for `Reg<L1FTLN_SPEC>`"]
pub type L1FTLN = crate::Reg<l1ftln::L1FTLN_SPEC>;
#[doc = "Layer 1 frame total line number register"]
pub mod l1ftln;
#[doc = "L0LUT register accessor: an alias for `Reg<L0LUT_SPEC>`"]
pub type L0LUT = crate::Reg<l0lut::L0LUT_SPEC>;
#[doc = "Layer 0 look up table register"]
pub mod l0lut;
#[doc = "L1LUT register accessor: an alias for `Reg<L1LUT_SPEC>`"]
pub type L1LUT = crate::Reg<l1lut::L1LUT_SPEC>;
#[doc = "Layer 1 look up table register"]
pub mod l1lut;
