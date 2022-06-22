#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x04 - Independent Data register"]
    pub fdata: crate::Reg<fdata::FDATA_SPEC>,
    #[doc = "0x08 - Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
#[doc = "FDATA register accessor: an alias for `Reg<FDATA_SPEC>`"]
pub type FDATA = crate::Reg<fdata::FDATA_SPEC>;
#[doc = "Independent Data register"]
pub mod fdata;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
