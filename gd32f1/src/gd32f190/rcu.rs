#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x04 - Clock configuration register 0 (RCU_CFG0)"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    #[doc = "0x08 - Clock interrupt register (RCU_INT)"]
    pub int: crate::Reg<int::INT_SPEC>,
    #[doc = "0x0c - APB2 reset register (RCU_APB2RST)"]
    pub apb2rst: crate::Reg<apb2rst::APB2RST_SPEC>,
    #[doc = "0x10 - APB1 reset register (RCU_APB1RST)"]
    pub apb1rst: crate::Reg<apb1rst::APB1RST_SPEC>,
    #[doc = "0x14 - AHB enable register (RCU_AHBEN)"]
    pub ahben: crate::Reg<ahben::AHBEN_SPEC>,
    #[doc = "0x18 - APB2 enable register (RCU_APB2EN)"]
    pub apb2en: crate::Reg<apb2en::APB2EN_SPEC>,
    #[doc = "0x1c - APB1 enable register (RCU_APB1EN)"]
    pub apb1en: crate::Reg<apb1en::APB1EN_SPEC>,
    #[doc = "0x20 - Backup domain control register (RCU_BDCTL)"]
    pub bdctl: crate::Reg<bdctl::BDCTL_SPEC>,
    #[doc = "0x24 - Reset source /clock register (RCU_RSTSCK)"]
    pub rstsck: crate::Reg<rstsck::RSTSCK_SPEC>,
    #[doc = "0x28 - AHB reset register"]
    pub ahbrst: crate::Reg<ahbrst::AHBRST_SPEC>,
    #[doc = "0x2c - Configuration register 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0x30 - Configuration register 2"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    #[doc = "0x34 - Control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    _reserved14: [u8; 0x48],
    #[doc = "0x80 - Configuration register 4"]
    pub cfg3: crate::Reg<cfg3::CFG3_SPEC>,
    _reserved15: [u8; 0x74],
    #[doc = "0xf8 - Additional enable register"]
    pub adden: crate::Reg<adden::ADDEN_SPEC>,
    #[doc = "0xfc - Additional reset register"]
    pub addrst: crate::Reg<addrst::ADDRST_SPEC>,
    #[doc = "0x100 - Voltage key register"]
    pub vkey: crate::Reg<vkey::VKEY_SPEC>,
    _reserved18: [u8; 0x30],
    #[doc = "0x134 - Deep-sleep mode voltage register"]
    pub dsv: crate::Reg<dsv::DSV_SPEC>,
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
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
#[doc = "AHB enable register (RCU_AHBEN)"]
pub mod ahben;
#[doc = "APB2EN register accessor: an alias for `Reg<APB2EN_SPEC>`"]
pub type APB2EN = crate::Reg<apb2en::APB2EN_SPEC>;
#[doc = "APB2 enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "APB1EN register accessor: an alias for `Reg<APB1EN_SPEC>`"]
pub type APB1EN = crate::Reg<apb1en::APB1EN_SPEC>;
#[doc = "APB1 enable register (RCU_APB1EN)"]
pub mod apb1en;
#[doc = "BDCTL register accessor: an alias for `Reg<BDCTL_SPEC>`"]
pub type BDCTL = crate::Reg<bdctl::BDCTL_SPEC>;
#[doc = "Backup domain control register (RCU_BDCTL)"]
pub mod bdctl;
#[doc = "RSTSCK register accessor: an alias for `Reg<RSTSCK_SPEC>`"]
pub type RSTSCK = crate::Reg<rstsck::RSTSCK_SPEC>;
#[doc = "Reset source /clock register (RCU_RSTSCK)"]
pub mod rstsck;
#[doc = "AHBRST register accessor: an alias for `Reg<AHBRST_SPEC>`"]
pub type AHBRST = crate::Reg<ahbrst::AHBRST_SPEC>;
#[doc = "AHB reset register"]
pub mod ahbrst;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "Configuration register 2"]
pub mod cfg2;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "CFG3 register accessor: an alias for `Reg<CFG3_SPEC>`"]
pub type CFG3 = crate::Reg<cfg3::CFG3_SPEC>;
#[doc = "Configuration register 4"]
pub mod cfg3;
#[doc = "ADDEN register accessor: an alias for `Reg<ADDEN_SPEC>`"]
pub type ADDEN = crate::Reg<adden::ADDEN_SPEC>;
#[doc = "Additional enable register"]
pub mod adden;
#[doc = "ADDRST register accessor: an alias for `Reg<ADDRST_SPEC>`"]
pub type ADDRST = crate::Reg<addrst::ADDRST_SPEC>;
#[doc = "Additional reset register"]
pub mod addrst;
#[doc = "VKEY register accessor: an alias for `Reg<VKEY_SPEC>`"]
pub type VKEY = crate::Reg<vkey::VKEY_SPEC>;
#[doc = "Voltage key register"]
pub mod vkey;
#[doc = "DSV register accessor: an alias for `Reg<DSV_SPEC>`"]
pub type DSV = crate::Reg<dsv::DSV_SPEC>;
#[doc = "Deep-sleep mode voltage register"]
pub mod dsv;
