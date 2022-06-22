#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HAU control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - HAU data input register"]
    pub di: crate::Reg<di::DI_SPEC>,
    #[doc = "0x08 - HAU configuration register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x0c - HAU data output register"]
    pub do0: crate::Reg<do0::DO0_SPEC>,
    #[doc = "0x10 - HAU data output register"]
    pub do1: crate::Reg<do1::DO1_SPEC>,
    #[doc = "0x14 - HAU data output register"]
    pub do2: crate::Reg<do2::DO2_SPEC>,
    #[doc = "0x18 - HAU data output register"]
    pub do3: crate::Reg<do3::DO3_SPEC>,
    #[doc = "0x1c - HAU data output register"]
    pub do4: crate::Reg<do4::DO4_SPEC>,
    #[doc = "0x20 - HAU interrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x24 - HAU status and interrupt flag register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved10: [u8; 0x02fc],
    #[doc = "0x324 - HAU data output register"]
    pub do5: crate::Reg<do5::DO5_SPEC>,
    #[doc = "0x328 - HAU data output register"]
    pub do6: crate::Reg<do6::DO6_SPEC>,
    #[doc = "0x32c - HAU data output register"]
    pub do7: crate::Reg<do7::DO7_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "HAU control register"]
pub mod ctl;
#[doc = "DI register accessor: an alias for `Reg<DI_SPEC>`"]
pub type DI = crate::Reg<di::DI_SPEC>;
#[doc = "HAU data input register"]
pub mod di;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "HAU configuration register"]
pub mod cfg;
#[doc = "DO0 register accessor: an alias for `Reg<DO0_SPEC>`"]
pub type DO0 = crate::Reg<do0::DO0_SPEC>;
#[doc = "HAU data output register"]
pub mod do0;
#[doc = "DO1 register accessor: an alias for `Reg<DO1_SPEC>`"]
pub type DO1 = crate::Reg<do1::DO1_SPEC>;
#[doc = "HAU data output register"]
pub mod do1;
#[doc = "DO2 register accessor: an alias for `Reg<DO2_SPEC>`"]
pub type DO2 = crate::Reg<do2::DO2_SPEC>;
#[doc = "HAU data output register"]
pub mod do2;
#[doc = "DO3 register accessor: an alias for `Reg<DO3_SPEC>`"]
pub type DO3 = crate::Reg<do3::DO3_SPEC>;
#[doc = "HAU data output register"]
pub mod do3;
#[doc = "DO4 register accessor: an alias for `Reg<DO4_SPEC>`"]
pub type DO4 = crate::Reg<do4::DO4_SPEC>;
#[doc = "HAU data output register"]
pub mod do4;
#[doc = "DO5 register accessor: an alias for `Reg<DO5_SPEC>`"]
pub type DO5 = crate::Reg<do5::DO5_SPEC>;
#[doc = "HAU data output register"]
pub mod do5;
#[doc = "DO6 register accessor: an alias for `Reg<DO6_SPEC>`"]
pub type DO6 = crate::Reg<do6::DO6_SPEC>;
#[doc = "HAU data output register"]
pub mod do6;
#[doc = "DO7 register accessor: an alias for `Reg<DO7_SPEC>`"]
pub type DO7 = crate::Reg<do7::DO7_SPEC>;
#[doc = "HAU data output register"]
pub mod do7;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "HAU interrupt enable register"]
pub mod inten;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "HAU status and interrupt flag register"]
pub mod stat;
