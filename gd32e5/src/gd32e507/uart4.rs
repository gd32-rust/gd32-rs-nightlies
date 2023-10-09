#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register 0"]
    pub stat0: STAT0,
    #[doc = "0x04 - Data register"]
    pub data: DATA,
    #[doc = "0x08 - Baud rate register"]
    pub baud: BAUD,
    #[doc = "0x0c - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x10 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x14 - Control register 2"]
    pub ctl2: CTL2,
    #[doc = "0x18 - Guard time and prescaler register"]
    pub gp: GP,
    _reserved7: [u8; 0xb4],
    #[doc = "0xd0 - GD control register"]
    pub gdctl: GDCTL,
}
#[doc = "STAT0 (r) register accessor: Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat0`]
module"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "DATA (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
#[doc = "BAUD (rw) register accessor: Baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`baud`]
module"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud rate register"]
pub mod baud;
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "CTL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl2`]
module"]
pub type CTL2 = crate::Reg<ctl2::CTL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "GP (rw) register accessor: Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gp`]
module"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "Guard time and prescaler register"]
pub mod gp;
#[doc = "GDCTL (rw) register accessor: GD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gdctl`]
module"]
pub type GDCTL = crate::Reg<gdctl::GDCTL_SPEC>;
#[doc = "GD control register"]
pub mod gdctl;
