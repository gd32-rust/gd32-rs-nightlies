#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub data: DATA,
    #[doc = "0x04 - Free data register"]
    pub fdata: FDATA,
    #[doc = "0x08 - Control register"]
    pub ctl: CTL,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Initialization Data Register"]
    pub idata: IDATA,
    #[doc = "0x14 - Polynomial register"]
    pub poly: POLY,
}
#[doc = "DATA (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
#[doc = "FDATA (rw) register accessor: Free data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fdata`]
module"]
pub type FDATA = crate::Reg<fdata::FDATA_SPEC>;
#[doc = "Free data register"]
pub mod fdata;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "IDATA (rw) register accessor: Initialization Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idata`]
module"]
pub type IDATA = crate::Reg<idata::IDATA_SPEC>;
#[doc = "Initialization Data Register"]
pub mod idata;
#[doc = "POLY (rw) register accessor: Polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`poly`]
module"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "Polynomial register"]
pub mod poly;
