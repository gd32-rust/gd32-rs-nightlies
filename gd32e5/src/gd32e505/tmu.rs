#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input data0 register"]
    pub idata0: IDATA0,
    #[doc = "0x04 - Input data1 register"]
    pub idata1: IDATA1,
    #[doc = "0x08 - Control register"]
    pub ctl: CTL,
    #[doc = "0x0c - data0 register"]
    pub data0: DATA0,
    #[doc = "0x10 - data1 register"]
    pub data1: DATA1,
    #[doc = "0x14 - Status register"]
    pub stat: STAT,
}
#[doc = "IDATA0 (rw) register accessor: Input data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idata0`]
module"]
pub type IDATA0 = crate::Reg<idata0::IDATA0_SPEC>;
#[doc = "Input data0 register"]
pub mod idata0;
#[doc = "IDATA1 (rw) register accessor: Input data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idata1`]
module"]
pub type IDATA1 = crate::Reg<idata1::IDATA1_SPEC>;
#[doc = "Input data1 register"]
pub mod idata1;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "DATA0 (rw) register accessor: data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data0`]
module"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "data0 register"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data1`]
module"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "data1 register"]
pub mod data1;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
