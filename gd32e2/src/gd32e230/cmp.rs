#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control and status register"]
    pub cs: crate::Reg<cs::CS_SPEC>,
}
#[doc = "CS register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "control and status register"]
pub mod cs;
