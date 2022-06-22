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
    #[doc = "0x14 - AHB1 enable register"]
    pub ahb1en: crate::Reg<ahb1en::AHB1EN_SPEC>,
    #[doc = "0x18 - APB2 clock enable register (RCU_APB2EN)"]
    pub apb2en: crate::Reg<apb2en::APB2EN_SPEC>,
    #[doc = "0x1c - APB1 clock enable register (RCU_APB1EN)"]
    pub apb1en: crate::Reg<apb1en::APB1EN_SPEC>,
    #[doc = "0x20 - Backup domain control register (RCU_BDCTL)"]
    pub bdctl: crate::Reg<bdctl::BDCTL_SPEC>,
    #[doc = "0x24 - Reset source /clock register (RCU_RSTSCK)"]
    pub rstsck: crate::Reg<rstsck::RSTSCK_SPEC>,
    #[doc = "0x28 - AHB1 reset register"]
    pub ahb1rst: crate::Reg<ahb1rst::AHB1RST_SPEC>,
    #[doc = "0x2c - Clock Configuration register 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Deep sleep mode Voltage register"]
    pub dsv: crate::Reg<dsv::DSV_SPEC>,
    _reserved13: [u8; 0x28],
    #[doc = "0x60 - AHB2 enable register"]
    pub ahb2en: crate::Reg<ahb2en::AHB2EN_SPEC>,
    #[doc = "0x64 - APB2 additional enable register"]
    pub addapb2en: crate::Reg<addapb2en::ADDAPB2EN_SPEC>,
    #[doc = "0x68 - APB1 additional enable register"]
    pub addapb1en: crate::Reg<addapb1en::ADDAPB1EN_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x70 - AHB2 reset register"]
    pub ahb2rst: crate::Reg<ahb2rst::AHB2RST_SPEC>,
    #[doc = "0x74 - APB2 additional enable register"]
    pub addapb2rst: crate::Reg<addapb2rst::ADDAPB2RST_SPEC>,
    #[doc = "0x78 - APB1 additional enable register"]
    pub addapb1rst: crate::Reg<addapb1rst::ADDAPB1RST_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x80 - configuration register 2"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x90 - PLLT control register"]
    pub plltctl: crate::Reg<plltctl::PLLTCTL_SPEC>,
    #[doc = "0x94 - PLLT interrupt register"]
    pub plltint: crate::Reg<plltint::PLLTINT_SPEC>,
    #[doc = "0x98 - PLLT configuration register"]
    pub plltcfg: crate::Reg<plltcfg::PLLTCFG_SPEC>,
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
#[doc = "AHB1EN register accessor: an alias for `Reg<AHB1EN_SPEC>`"]
pub type AHB1EN = crate::Reg<ahb1en::AHB1EN_SPEC>;
#[doc = "AHB1 enable register"]
pub mod ahb1en;
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
#[doc = "AHB1RST register accessor: an alias for `Reg<AHB1RST_SPEC>`"]
pub type AHB1RST = crate::Reg<ahb1rst::AHB1RST_SPEC>;
#[doc = "AHB1 reset register"]
pub mod ahb1rst;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Clock Configuration register 1"]
pub mod cfg1;
#[doc = "DSV register accessor: an alias for `Reg<DSV_SPEC>`"]
pub type DSV = crate::Reg<dsv::DSV_SPEC>;
#[doc = "Deep sleep mode Voltage register"]
pub mod dsv;
#[doc = "AHB2EN register accessor: an alias for `Reg<AHB2EN_SPEC>`"]
pub type AHB2EN = crate::Reg<ahb2en::AHB2EN_SPEC>;
#[doc = "AHB2 enable register"]
pub mod ahb2en;
#[doc = "ADDAPB2EN register accessor: an alias for `Reg<ADDAPB2EN_SPEC>`"]
pub type ADDAPB2EN = crate::Reg<addapb2en::ADDAPB2EN_SPEC>;
#[doc = "APB2 additional enable register"]
pub mod addapb2en;
#[doc = "ADDAPB1EN register accessor: an alias for `Reg<ADDAPB1EN_SPEC>`"]
pub type ADDAPB1EN = crate::Reg<addapb1en::ADDAPB1EN_SPEC>;
#[doc = "APB1 additional enable register"]
pub mod addapb1en;
#[doc = "AHB2RST register accessor: an alias for `Reg<AHB2RST_SPEC>`"]
pub type AHB2RST = crate::Reg<ahb2rst::AHB2RST_SPEC>;
#[doc = "AHB2 reset register"]
pub mod ahb2rst;
#[doc = "ADDAPB2RST register accessor: an alias for `Reg<ADDAPB2RST_SPEC>`"]
pub type ADDAPB2RST = crate::Reg<addapb2rst::ADDAPB2RST_SPEC>;
#[doc = "APB2 additional enable register"]
pub mod addapb2rst;
#[doc = "ADDAPB1RST register accessor: an alias for `Reg<ADDAPB1RST_SPEC>`"]
pub type ADDAPB1RST = crate::Reg<addapb1rst::ADDAPB1RST_SPEC>;
#[doc = "APB1 additional enable register"]
pub mod addapb1rst;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "configuration register 2"]
pub mod cfg2;
#[doc = "PLLTCTL register accessor: an alias for `Reg<PLLTCTL_SPEC>`"]
pub type PLLTCTL = crate::Reg<plltctl::PLLTCTL_SPEC>;
#[doc = "PLLT control register"]
pub mod plltctl;
#[doc = "PLLTINT register accessor: an alias for `Reg<PLLTINT_SPEC>`"]
pub type PLLTINT = crate::Reg<plltint::PLLTINT_SPEC>;
#[doc = "PLLT interrupt register"]
pub mod plltint;
#[doc = "PLLTCFG register accessor: an alias for `Reg<PLLTCFG_SPEC>`"]
pub type PLLTCFG = crate::Reg<plltcfg::PLLTCFG_SPEC>;
#[doc = "PLLT configuration register"]
pub mod plltcfg;
