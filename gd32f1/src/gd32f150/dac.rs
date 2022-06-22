#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - software trigger register"]
    pub swt: crate::Reg<swt::SWT_SPEC>,
    #[doc = "0x08 - DAC0 12-bit right-aligned data holding register"]
    pub dac0_r12dh: crate::Reg<dac0_r12dh::DAC0_R12DH_SPEC>,
    #[doc = "0x0c - DAC0 12-bit left aligned data holding register"]
    pub dac0_l12dh: crate::Reg<dac0_l12dh::DAC0_L12DH_SPEC>,
    #[doc = "0x10 - DAC0 8-bit right aligned data holding register"]
    pub dac0_r8dh: crate::Reg<dac0_r8dh::DAC0_R8DH_SPEC>,
    _reserved5: [u8; 0x18],
    #[doc = "0x2c - DAC0 data output register"]
    pub dac0_do: crate::Reg<dac0_do::DAC0_DO_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x34 - status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "SWT register accessor: an alias for `Reg<SWT_SPEC>`"]
pub type SWT = crate::Reg<swt::SWT_SPEC>;
#[doc = "software trigger register"]
pub mod swt;
#[doc = "DAC0_R12DH register accessor: an alias for `Reg<DAC0_R12DH_SPEC>`"]
pub type DAC0_R12DH = crate::Reg<dac0_r12dh::DAC0_R12DH_SPEC>;
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub mod dac0_r12dh;
#[doc = "DAC0_L12DH register accessor: an alias for `Reg<DAC0_L12DH_SPEC>`"]
pub type DAC0_L12DH = crate::Reg<dac0_l12dh::DAC0_L12DH_SPEC>;
#[doc = "DAC0 12-bit left aligned data holding register"]
pub mod dac0_l12dh;
#[doc = "DAC0_R8DH register accessor: an alias for `Reg<DAC0_R8DH_SPEC>`"]
pub type DAC0_R8DH = crate::Reg<dac0_r8dh::DAC0_R8DH_SPEC>;
#[doc = "DAC0 8-bit right aligned data holding register"]
pub mod dac0_r8dh;
#[doc = "DAC0_DO register accessor: an alias for `Reg<DAC0_DO_SPEC>`"]
pub type DAC0_DO = crate::Reg<dac0_do::DAC0_DO_SPEC>;
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "status register"]
pub mod stat;
