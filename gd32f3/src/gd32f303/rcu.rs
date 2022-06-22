#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x04 - Clock configuration register 0 (RCU_CFG0)"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    #[doc = "0x08 - Clock interrupt register (RCU_INT)"]
    pub int: crate::Reg<int::INT_SPEC>,
    #[doc = "0x0c - APB2 reset register (RCU_APB2RST)"]
    pub apb2rst: crate::Reg<apb2rst::APB2RST_SPEC>,
    #[doc = "0x10 - APB1 reset register (RCU_APB1RST)"]
    pub apb1rst: crate::Reg<apb1rst::APB1RST_SPEC>,
    #[doc = "0x14 - AHB enable register"]
    pub ahben: crate::Reg<ahben::AHBEN_SPEC>,
    #[doc = "0x18 - APB2 clock enable register (RCU_APB2EN)"]
    pub apb2en: crate::Reg<apb2en::APB2EN_SPEC>,
    #[doc = "0x1c - APB1 clock enable register (RCU_APB1EN)"]
    pub apb1en: crate::Reg<apb1en::APB1EN_SPEC>,
    #[doc = "0x20 - Backup domain control register (RCU_BDCTL)"]
    pub bdctl: crate::Reg<bdctl::BDCTL_SPEC>,
    #[doc = "0x24 - Reset source /clock register (RCU_RSTSCK)"]
    pub rstsck: crate::Reg<rstsck::RSTSCK_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - Clock Configuration register 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x34 - Deep sleep mode Voltage register"]
    pub dsv: crate::Reg<dsv::DSV_SPEC>,
    _reserved12: [u8; 0x88],
    #[doc = "0xc0 - Additional clock control register"]
    pub addctl: crate::Reg<addctl::ADDCTL_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0xcc - Additional clock interrupt register"]
    pub addint: crate::Reg<addint::ADDINT_SPEC>,
    _reserved14: [u8; 0x10],
    #[doc = "0xe0 - APB1 additional reset register"]
    pub addapb1rst: crate::Reg<addapb1rst::ADDAPB1RST_SPEC>,
    #[doc = "0xe4 - APB1 additional enable register"]
    pub addapb1en: crate::Reg<addapb1en::ADDAPB1EN_SPEC>,
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register"]
pub mod ctl0;
#[doc = "CFG0 register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "Clock configuration register 0 (RCU_CFG0)"]
pub mod cfg0;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Clock interrupt register (RCU_INT)"]
pub mod int;
#[doc = "APB2RST register accessor: an alias for `Reg<APB2RST_SPEC>`"]
pub type APB2RST = crate::Reg<apb2rst::APB2RST_SPEC>;
#[doc = "APB2 reset register (RCU_APB2RST)"]
pub mod apb2rst;
#[doc = "APB1RST register accessor: an alias for `Reg<APB1RST_SPEC>`"]
pub type APB1RST = crate::Reg<apb1rst::APB1RST_SPEC>;
#[doc = "APB1 reset register (RCU_APB1RST)"]
pub mod apb1rst;
#[doc = "AHBEN register accessor: an alias for `Reg<AHBEN_SPEC>`"]
pub type AHBEN = crate::Reg<ahben::AHBEN_SPEC>;
#[doc = "AHB enable register"]
pub mod ahben;
#[doc = "APB2EN register accessor: an alias for `Reg<APB2EN_SPEC>`"]
pub type APB2EN = crate::Reg<apb2en::APB2EN_SPEC>;
#[doc = "APB2 clock enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "APB1EN register accessor: an alias for `Reg<APB1EN_SPEC>`"]
pub type APB1EN = crate::Reg<apb1en::APB1EN_SPEC>;
#[doc = "APB1 clock enable register (RCU_APB1EN)"]
pub mod apb1en;
#[doc = "BDCTL register accessor: an alias for `Reg<BDCTL_SPEC>`"]
pub type BDCTL = crate::Reg<bdctl::BDCTL_SPEC>;
#[doc = "Backup domain control register (RCU_BDCTL)"]
pub mod bdctl;
#[doc = "RSTSCK register accessor: an alias for `Reg<RSTSCK_SPEC>`"]
pub type RSTSCK = crate::Reg<rstsck::RSTSCK_SPEC>;
#[doc = "Reset source /clock register (RCU_RSTSCK)"]
pub mod rstsck;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Clock Configuration register 1"]
pub mod cfg1;
#[doc = "DSV register accessor: an alias for `Reg<DSV_SPEC>`"]
pub type DSV = crate::Reg<dsv::DSV_SPEC>;
#[doc = "Deep sleep mode Voltage register"]
pub mod dsv;
#[doc = "ADDCTL register accessor: an alias for `Reg<ADDCTL_SPEC>`"]
pub type ADDCTL = crate::Reg<addctl::ADDCTL_SPEC>;
#[doc = "Additional clock control register"]
pub mod addctl;
#[doc = "ADDINT register accessor: an alias for `Reg<ADDINT_SPEC>`"]
pub type ADDINT = crate::Reg<addint::ADDINT_SPEC>;
#[doc = "Additional clock interrupt register"]
pub mod addint;
#[doc = "ADDAPB1RST register accessor: an alias for `Reg<ADDAPB1RST_SPEC>`"]
pub type ADDAPB1RST = crate::Reg<addapb1rst::ADDAPB1RST_SPEC>;
#[doc = "APB1 additional reset register"]
pub mod addapb1rst;
#[doc = "ADDAPB1EN register accessor: an alias for `Reg<ADDAPB1EN_SPEC>`"]
pub type ADDAPB1EN = crate::Reg<addapb1en::ADDAPB1EN_SPEC>;
#[doc = "APB1 additional enable register"]
pub mod addapb1en;
