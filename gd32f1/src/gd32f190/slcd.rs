#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - SLCD configuration register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - SLCD status flag register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x0c - SLCD status flag clear register"]
    pub statc: crate::Reg<statc::STATC_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - SLCD display data register"]
    pub data0: crate::Reg<data0::DATA0_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - SLCD display data register"]
    pub data1: crate::Reg<data1::DATA1_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - SLCD display data register"]
    pub data2: crate::Reg<data2::DATA2_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - SLCD display data register"]
    pub data3: crate::Reg<data3::DATA3_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x34 - SLCD display data register"]
    pub data4: crate::Reg<data4::DATA4_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x3c - SLCD display data register"]
    pub data5: crate::Reg<data5::DATA5_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x44 - SLCD display data register"]
    pub data6: crate::Reg<data6::DATA6_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x4c - SLCD display data register"]
    pub data7: crate::Reg<data7::DATA7_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "SLCD configuration register"]
pub mod cfg;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SLCD status flag register"]
pub mod stat;
#[doc = "STATC register accessor: an alias for `Reg<STATC_SPEC>`"]
pub type STATC = crate::Reg<statc::STATC_SPEC>;
#[doc = "SLCD status flag clear register"]
pub mod statc;
#[doc = "DATA0 register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "SLCD display data register"]
pub mod data0;
#[doc = "DATA1 register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "SLCD display data register"]
pub mod data1;
#[doc = "DATA2 register accessor: an alias for `Reg<DATA2_SPEC>`"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "SLCD display data register"]
pub mod data2;
#[doc = "DATA3 register accessor: an alias for `Reg<DATA3_SPEC>`"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "SLCD display data register"]
pub mod data3;
#[doc = "DATA4 register accessor: an alias for `Reg<DATA4_SPEC>`"]
pub type DATA4 = crate::Reg<data4::DATA4_SPEC>;
#[doc = "SLCD display data register"]
pub mod data4;
#[doc = "DATA5 register accessor: an alias for `Reg<DATA5_SPEC>`"]
pub type DATA5 = crate::Reg<data5::DATA5_SPEC>;
#[doc = "SLCD display data register"]
pub mod data5;
#[doc = "DATA6 register accessor: an alias for `Reg<DATA6_SPEC>`"]
pub type DATA6 = crate::Reg<data6::DATA6_SPEC>;
#[doc = "SLCD display data register"]
pub mod data6;
#[doc = "DATA7 register accessor: an alias for `Reg<DATA7_SPEC>`"]
pub type DATA7 = crate::Reg<data7::DATA7_SPEC>;
#[doc = "SLCD display data register"]
pub mod data7;
