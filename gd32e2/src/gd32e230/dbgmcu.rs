#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x04 - Debug Control Register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x08 - Debug Control Register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
}
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "MCU Device ID Code Register"]
pub mod id;
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Debug Control Register 0"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Debug Control Register 1"]
pub mod ctl1;
