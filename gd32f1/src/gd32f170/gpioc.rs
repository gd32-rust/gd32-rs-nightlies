#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port control register"]
    pub ctl: CTL,
    #[doc = "0x04 - GPIO port output type register"]
    pub omode: OMODE,
    #[doc = "0x08 - GPIO port output speed register"]
    pub ospd: OSPD,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub pud: PUD,
    #[doc = "0x10 - GPIO port input data register"]
    pub istat: ISTAT,
    #[doc = "0x14 - GPIO port output data register"]
    pub octl: OCTL,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub bop: BOP,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - GPIO alternate function low register"]
    pub afsel0: AFSEL0,
    #[doc = "0x24 - GPIO alternate function register 1"]
    pub afsel1: AFSEL1,
    #[doc = "0x28 - Port bit reset register"]
    pub bc: BC,
    #[doc = "0x2c - Port bit toggle register"]
    pub tg: TG,
}
#[doc = "CTL (rw) register accessor: GPIO port control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "GPIO port control register"]
pub mod ctl;
#[doc = "OMODE (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`omode`]
module"]
pub type OMODE = crate::Reg<omode::OMODE_SPEC>;
#[doc = "GPIO port output type register"]
pub mod omode;
#[doc = "OSPD (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ospd`]
module"]
pub type OSPD = crate::Reg<ospd::OSPD_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod ospd;
#[doc = "PUD (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pud`]
module"]
pub type PUD = crate::Reg<pud::PUD_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pud;
#[doc = "ISTAT (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`istat`]
module"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "GPIO port input data register"]
pub mod istat;
#[doc = "OCTL (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`octl`]
module"]
pub type OCTL = crate::Reg<octl::OCTL_SPEC>;
#[doc = "GPIO port output data register"]
pub mod octl;
#[doc = "BOP (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bop`]
module"]
pub type BOP = crate::Reg<bop::BOP_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod bop;
#[doc = "AFSEL0 (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`afsel0`]
module"]
pub type AFSEL0 = crate::Reg<afsel0::AFSEL0_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod afsel0;
#[doc = "AFSEL1 (rw) register accessor: GPIO alternate function register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`afsel1`]
module"]
pub type AFSEL1 = crate::Reg<afsel1::AFSEL1_SPEC>;
#[doc = "GPIO alternate function register 1"]
pub mod afsel1;
#[doc = "BC (w) register accessor: Port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bc`]
module"]
pub type BC = crate::Reg<bc::BC_SPEC>;
#[doc = "Port bit reset register"]
pub mod bc;
#[doc = "TG (w) register accessor: Port bit toggle register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg`]
module"]
pub type TG = crate::Reg<tg::TG_SPEC>;
#[doc = "Port bit toggle register"]
pub mod tg;
