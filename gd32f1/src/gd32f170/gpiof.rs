#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOF port control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub omode: crate::Reg<omode::OMODE_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub ospd: crate::Reg<ospd::OSPD_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub pud: crate::Reg<pud::PUD_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub istat: crate::Reg<istat::ISTAT_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub octl: crate::Reg<octl::OCTL_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub bop: crate::Reg<bop::BOP_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x28 - Port bit reset register"]
    pub bc: crate::Reg<bc::BC_SPEC>,
    #[doc = "0x2c - Port bit toggle register"]
    pub tg: crate::Reg<tg::TG_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "GPIOF port control register"]
pub mod ctl;
#[doc = "OMODE register accessor: an alias for `Reg<OMODE_SPEC>`"]
pub type OMODE = crate::Reg<omode::OMODE_SPEC>;
#[doc = "GPIO port output type register"]
pub mod omode;
#[doc = "OSPD register accessor: an alias for `Reg<OSPD_SPEC>`"]
pub type OSPD = crate::Reg<ospd::OSPD_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod ospd;
#[doc = "PUD register accessor: an alias for `Reg<PUD_SPEC>`"]
pub type PUD = crate::Reg<pud::PUD_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pud;
#[doc = "ISTAT register accessor: an alias for `Reg<ISTAT_SPEC>`"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "GPIO port input data register"]
pub mod istat;
#[doc = "OCTL register accessor: an alias for `Reg<OCTL_SPEC>`"]
pub type OCTL = crate::Reg<octl::OCTL_SPEC>;
#[doc = "GPIO port output data register"]
pub mod octl;
#[doc = "BOP register accessor: an alias for `Reg<BOP_SPEC>`"]
pub type BOP = crate::Reg<bop::BOP_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod bop;
#[doc = "BC register accessor: an alias for `Reg<BC_SPEC>`"]
pub type BC = crate::Reg<bc::BC_SPEC>;
#[doc = "Port bit reset register"]
pub mod bc;
#[doc = "TG register accessor: an alias for `Reg<TG_SPEC>`"]
pub type TG = crate::Reg<tg::TG_SPEC>;
#[doc = "Port bit toggle register"]
pub mod tg;
