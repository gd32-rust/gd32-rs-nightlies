#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - SLCD configuration register"]
    pub cfg: CFG,
    #[doc = "0x08 - SLCD status flag register"]
    pub stat: STAT,
    #[doc = "0x0c - SLCD status flag clear register"]
    pub statc: STATC,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - SLCD display data register"]
    pub data0: DATA0,
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - SLCD display data register"]
    pub data1: DATA1,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - SLCD display data register"]
    pub data2: DATA2,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - SLCD display data register"]
    pub data3: DATA3,
    _reserved8: [u8; 0x04],
    #[doc = "0x34 - SLCD display data register"]
    pub data4: DATA4,
    _reserved9: [u8; 0x04],
    #[doc = "0x3c - SLCD display data register"]
    pub data5: DATA5,
    _reserved10: [u8; 0x04],
    #[doc = "0x44 - SLCD display data register"]
    pub data6: DATA6,
    _reserved11: [u8; 0x04],
    #[doc = "0x4c - SLCD display data register"]
    pub data7: DATA7,
}
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "CFG (rw) register accessor: SLCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "SLCD configuration register"]
pub mod cfg;
#[doc = "STAT (rw) register accessor: SLCD status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SLCD status flag register"]
pub mod stat;
#[doc = "STATC (rw) register accessor: SLCD status flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statc`]
module"]
pub type STATC = crate::Reg<statc::STATC_SPEC>;
#[doc = "SLCD status flag clear register"]
pub mod statc;
#[doc = "DATA0 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data0`]
module"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "SLCD display data register"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data1`]
module"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "SLCD display data register"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data2`]
module"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "SLCD display data register"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data3`]
module"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "SLCD display data register"]
pub mod data3;
#[doc = "DATA4 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data4`]
module"]
pub type DATA4 = crate::Reg<data4::DATA4_SPEC>;
#[doc = "SLCD display data register"]
pub mod data4;
#[doc = "DATA5 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data5`]
module"]
pub type DATA5 = crate::Reg<data5::DATA5_SPEC>;
#[doc = "SLCD display data register"]
pub mod data5;
#[doc = "DATA6 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data6`]
module"]
pub type DATA6 = crate::Reg<data6::DATA6_SPEC>;
#[doc = "SLCD display data register"]
pub mod data6;
#[doc = "DATA7 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data7`]
module"]
pub type DATA7 = crate::Reg<data7::DATA7_SPEC>;
#[doc = "SLCD display data register"]
pub mod data7;
