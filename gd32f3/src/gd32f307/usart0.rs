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
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Guard time and prescaler register"]
    pub gp: crate::Reg<gp::GP_SPEC>,
    _reserved7: [u8; 0x60],
    #[doc = "0x80 - Control register 3"]
    pub ctl3: crate::Reg<ctl3::CTL3_SPEC>,
    #[doc = "0x84 - Receiver timeout register"]
    pub rt: crate::Reg<rt::RT_SPEC>,
    #[doc = "0x88 - Status register 1"]
    pub stat1: crate::Reg<stat1::STAT1_SPEC>,
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
#[doc = "CTL3 register accessor: an alias for `Reg<CTL3_SPEC>`"]
pub type CTL3 = crate::Reg<ctl3::CTL3_SPEC>;
#[doc = "Control register 3"]
pub mod ctl3;
#[doc = "RT register accessor: an alias for `Reg<RT_SPEC>`"]
pub type RT = crate::Reg<rt::RT_SPEC>;
#[doc = "Receiver timeout register"]
pub mod rt;
#[doc = "STAT1 register accessor: an alias for `Reg<STAT1_SPEC>`"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "Status register 1"]
pub mod stat1;
