#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR flash control register 0"]
    pub snctl0: crate::Reg<snctl0::SNCTL0_SPEC>,
    #[doc = "0x04 - SRAM/NOR flash timing configuration register 0"]
    pub sntcfg0: crate::Reg<sntcfg0::SNTCFG0_SPEC>,
    #[doc = "0x08 - SRAM/NOR flash control register 1"]
    pub snctl1: crate::Reg<snctl1::SNCTL1_SPEC>,
    #[doc = "0x0c - SRAM/NOR flash timing configuration register 1"]
    pub sntcfg1: crate::Reg<sntcfg1::SNTCFG1_SPEC>,
    #[doc = "0x10 - SRAM/NOR flash control register 2"]
    pub snctl2: crate::Reg<snctl2::SNCTL2_SPEC>,
    #[doc = "0x14 - SRAM/NOR flash timing configuration register 2"]
    pub sntcfg2: crate::Reg<sntcfg2::SNTCFG2_SPEC>,
    #[doc = "0x18 - SRAM/NOR flash control register 3"]
    pub snctl3: crate::Reg<snctl3::SNCTL3_SPEC>,
    #[doc = "0x1c - SRAM/NOR flash timing configuration register 3"]
    pub sntcfg3: crate::Reg<sntcfg3::SNTCFG3_SPEC>,
    _reserved8: [u8; 0x40],
    #[doc = "0x60 - NAND flash/PC card control register 1"]
    pub npctl1: crate::Reg<npctl1::NPCTL1_SPEC>,
    #[doc = "0x64 - NAND flash/PC card interrupt enable register 1"]
    pub npinten1: crate::Reg<npinten1::NPINTEN1_SPEC>,
    #[doc = "0x68 - NAND flash/PC card common space timing configuration register 1"]
    pub npctcfg1: crate::Reg<npctcfg1::NPCTCFG1_SPEC>,
    #[doc = "0x6c - NAND flash/PC card attribute space timing configuration register 1"]
    pub npatcfg1: crate::Reg<npatcfg1::NPATCFG1_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x74 - NAND flash ECC register 1"]
    pub necc1: crate::Reg<necc1::NECC1_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - NAND flash/PC card control register 2"]
    pub npctl2: crate::Reg<npctl2::NPCTL2_SPEC>,
    #[doc = "0x84 - NAND flash/PC card interrupt enable register 2"]
    pub npinten2: crate::Reg<npinten2::NPINTEN2_SPEC>,
    #[doc = "0x88 - NAND flash/PC card common space timing configuration register 2"]
    pub npctcfg2: crate::Reg<npctcfg2::NPCTCFG2_SPEC>,
    #[doc = "0x8c - NAND flash/PC card attribute space timing configuration register 2"]
    pub npatcfg2: crate::Reg<npatcfg2::NPATCFG2_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x94 - NAND flash ECC register 2"]
    pub necc2: crate::Reg<necc2::NECC2_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0xa0 - NAND flash/PC card control register 3"]
    pub npctl3: crate::Reg<npctl3::NPCTL3_SPEC>,
    #[doc = "0xa4 - NAND flash/PC card interrupt enable register 3"]
    pub npinten3: crate::Reg<npinten3::NPINTEN3_SPEC>,
    #[doc = "0xa8 - NAND flash/PC card common space timing configuration register 3"]
    pub npctcfg3: crate::Reg<npctcfg3::NPCTCFG3_SPEC>,
    #[doc = "0xac - NAND flash/PC card attribute space timing configuration register 3"]
    pub npatcfg3: crate::Reg<npatcfg3::NPATCFG3_SPEC>,
    #[doc = "0xb0 - PC card I/O space timing configuration register"]
    pub piotcfg3: crate::Reg<piotcfg3::PIOTCFG3_SPEC>,
    _reserved23: [u8; 0x50],
    #[doc = "0x104 - SRAM/NOR flash write timing configuration register 0"]
    pub snwtcfg0: crate::Reg<snwtcfg0::SNWTCFG0_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR flash write timing configuration register 1"]
    pub snwtcfg1: crate::Reg<snwtcfg1::SNWTCFG1_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR flash write timing configuration register 2"]
    pub snwtcfg2: crate::Reg<snwtcfg2::SNWTCFG2_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR flash write timing configuration register 3"]
    pub snwtcfg3: crate::Reg<snwtcfg3::SNWTCFG3_SPEC>,
}
#[doc = "SNCTL0 register accessor: an alias for `Reg<SNCTL0_SPEC>`"]
pub type SNCTL0 = crate::Reg<snctl0::SNCTL0_SPEC>;
#[doc = "SRAM/NOR flash control register 0"]
pub mod snctl0;
#[doc = "SNTCFG0 register accessor: an alias for `Reg<SNTCFG0_SPEC>`"]
pub type SNTCFG0 = crate::Reg<sntcfg0::SNTCFG0_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub mod sntcfg0;
#[doc = "SNCTL1 register accessor: an alias for `Reg<SNCTL1_SPEC>`"]
pub type SNCTL1 = crate::Reg<snctl1::SNCTL1_SPEC>;
#[doc = "SRAM/NOR flash control register 1"]
pub mod snctl1;
#[doc = "SNTCFG1 register accessor: an alias for `Reg<SNTCFG1_SPEC>`"]
pub type SNTCFG1 = crate::Reg<sntcfg1::SNTCFG1_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 1"]
pub mod sntcfg1;
#[doc = "SNCTL2 register accessor: an alias for `Reg<SNCTL2_SPEC>`"]
pub type SNCTL2 = crate::Reg<snctl2::SNCTL2_SPEC>;
#[doc = "SRAM/NOR flash control register 2"]
pub mod snctl2;
#[doc = "SNTCFG2 register accessor: an alias for `Reg<SNTCFG2_SPEC>`"]
pub type SNTCFG2 = crate::Reg<sntcfg2::SNTCFG2_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 2"]
pub mod sntcfg2;
#[doc = "SNCTL3 register accessor: an alias for `Reg<SNCTL3_SPEC>`"]
pub type SNCTL3 = crate::Reg<snctl3::SNCTL3_SPEC>;
#[doc = "SRAM/NOR flash control register 3"]
pub mod snctl3;
#[doc = "SNTCFG3 register accessor: an alias for `Reg<SNTCFG3_SPEC>`"]
pub type SNTCFG3 = crate::Reg<sntcfg3::SNTCFG3_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 3"]
pub mod sntcfg3;
#[doc = "SNWTCFG0 register accessor: an alias for `Reg<SNWTCFG0_SPEC>`"]
pub type SNWTCFG0 = crate::Reg<snwtcfg0::SNWTCFG0_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 0"]
pub mod snwtcfg0;
#[doc = "SNWTCFG1 register accessor: an alias for `Reg<SNWTCFG1_SPEC>`"]
pub type SNWTCFG1 = crate::Reg<snwtcfg1::SNWTCFG1_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 1"]
pub mod snwtcfg1;
#[doc = "SNWTCFG2 register accessor: an alias for `Reg<SNWTCFG2_SPEC>`"]
pub type SNWTCFG2 = crate::Reg<snwtcfg2::SNWTCFG2_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 2"]
pub mod snwtcfg2;
#[doc = "SNWTCFG3 register accessor: an alias for `Reg<SNWTCFG3_SPEC>`"]
pub type SNWTCFG3 = crate::Reg<snwtcfg3::SNWTCFG3_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 3"]
pub mod snwtcfg3;
#[doc = "NPCTL1 register accessor: an alias for `Reg<NPCTL1_SPEC>`"]
pub type NPCTL1 = crate::Reg<npctl1::NPCTL1_SPEC>;
#[doc = "NAND flash/PC card control register 1"]
pub mod npctl1;
#[doc = "NPCTL2 register accessor: an alias for `Reg<NPCTL2_SPEC>`"]
pub type NPCTL2 = crate::Reg<npctl2::NPCTL2_SPEC>;
#[doc = "NAND flash/PC card control register 2"]
pub mod npctl2;
#[doc = "NPCTL3 register accessor: an alias for `Reg<NPCTL3_SPEC>`"]
pub type NPCTL3 = crate::Reg<npctl3::NPCTL3_SPEC>;
#[doc = "NAND flash/PC card control register 3"]
pub mod npctl3;
#[doc = "NPINTEN1 register accessor: an alias for `Reg<NPINTEN1_SPEC>`"]
pub type NPINTEN1 = crate::Reg<npinten1::NPINTEN1_SPEC>;
#[doc = "NAND flash/PC card interrupt enable register 1"]
pub mod npinten1;
#[doc = "NPINTEN2 register accessor: an alias for `Reg<NPINTEN2_SPEC>`"]
pub type NPINTEN2 = crate::Reg<npinten2::NPINTEN2_SPEC>;
#[doc = "NAND flash/PC card interrupt enable register 2"]
pub mod npinten2;
#[doc = "NPINTEN3 register accessor: an alias for `Reg<NPINTEN3_SPEC>`"]
pub type NPINTEN3 = crate::Reg<npinten3::NPINTEN3_SPEC>;
#[doc = "NAND flash/PC card interrupt enable register 3"]
pub mod npinten3;
#[doc = "NPCTCFG1 register accessor: an alias for `Reg<NPCTCFG1_SPEC>`"]
pub type NPCTCFG1 = crate::Reg<npctcfg1::NPCTCFG1_SPEC>;
#[doc = "NAND flash/PC card common space timing configuration register 1"]
pub mod npctcfg1;
#[doc = "NPCTCFG2 register accessor: an alias for `Reg<NPCTCFG2_SPEC>`"]
pub type NPCTCFG2 = crate::Reg<npctcfg2::NPCTCFG2_SPEC>;
#[doc = "NAND flash/PC card common space timing configuration register 2"]
pub mod npctcfg2;
#[doc = "NPCTCFG3 register accessor: an alias for `Reg<NPCTCFG3_SPEC>`"]
pub type NPCTCFG3 = crate::Reg<npctcfg3::NPCTCFG3_SPEC>;
#[doc = "NAND flash/PC card common space timing configuration register 3"]
pub mod npctcfg3;
#[doc = "NPATCFG1 register accessor: an alias for `Reg<NPATCFG1_SPEC>`"]
pub type NPATCFG1 = crate::Reg<npatcfg1::NPATCFG1_SPEC>;
#[doc = "NAND flash/PC card attribute space timing configuration register 1"]
pub mod npatcfg1;
#[doc = "NPATCFG2 register accessor: an alias for `Reg<NPATCFG2_SPEC>`"]
pub type NPATCFG2 = crate::Reg<npatcfg2::NPATCFG2_SPEC>;
#[doc = "NAND flash/PC card attribute space timing configuration register 2"]
pub mod npatcfg2;
#[doc = "NPATCFG3 register accessor: an alias for `Reg<NPATCFG3_SPEC>`"]
pub type NPATCFG3 = crate::Reg<npatcfg3::NPATCFG3_SPEC>;
#[doc = "NAND flash/PC card attribute space timing configuration register 3"]
pub mod npatcfg3;
#[doc = "PIOTCFG3 register accessor: an alias for `Reg<PIOTCFG3_SPEC>`"]
pub type PIOTCFG3 = crate::Reg<piotcfg3::PIOTCFG3_SPEC>;
#[doc = "PC card I/O space timing configuration register"]
pub mod piotcfg3;
#[doc = "NECC1 register accessor: an alias for `Reg<NECC1_SPEC>`"]
pub type NECC1 = crate::Reg<necc1::NECC1_SPEC>;
#[doc = "NAND flash ECC register 1"]
pub mod necc1;
#[doc = "NECC2 register accessor: an alias for `Reg<NECC2_SPEC>`"]
pub type NECC2 = crate::Reg<necc2::NECC2_SPEC>;
#[doc = "NAND flash ECC register 2"]
pub mod necc2;
