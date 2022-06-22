#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    #[doc = "0x300 - IVREF control register"]
    pub ivref_ctl: crate::Reg<ivref_ctl::IVREF_CTL_SPEC>,
}
#[doc = "IVREF_CTL register accessor: an alias for `Reg<IVREF_CTL_SPEC>`"]
pub type IVREF_CTL = crate::Reg<ivref_ctl::IVREF_CTL_SPEC>;
#[doc = "IVREF control register"]
pub mod ivref_ctl;
