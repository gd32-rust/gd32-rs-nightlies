#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register 0"]
    pub stat0: crate::Reg<stat0::STAT0_SPEC>,
    #[doc = "0x04 - Data register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x08 - Baud rate register"]
    pub baud: crate::Reg<baud::BAUD_SPEC>,
    #[doc = "0x0c - Control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x10 - Control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    #[doc = "0x14 - Control register 2"]
    pub ctl2: crate::Reg<ctl2::CTL2_SPEC>,
    #[doc = "0x18 - Guard time and prescaler register"]
    pub gp: crate::Reg<gp::GP_SPEC>,
}
#[doc = "STAT0 register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
#[doc = "BAUD register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud rate register"]
pub mod baud;
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "CTL2 register accessor: an alias for `Reg<CTL2_SPEC>`"]
pub type CTL2 = crate::Reg<ctl2::CTL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "GP register accessor: an alias for `Reg<GP_SPEC>`"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "Guard time and prescaler register"]
pub mod gp;
