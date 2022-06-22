#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ID code register"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x04 - Control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
}
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ID code register"]
pub mod id;
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
