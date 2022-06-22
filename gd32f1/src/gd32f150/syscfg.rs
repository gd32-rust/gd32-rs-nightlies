#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System configuration register 0"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    #[doc = "0x04 - System configuration register 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: crate::Reg<extiss0::EXTISS0_SPEC>,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: crate::Reg<extiss1::EXTISS1_SPEC>,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: crate::Reg<extiss2::EXTISS2_SPEC>,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: crate::Reg<extiss3::EXTISS3_SPEC>,
    #[doc = "0x18 - System configuration register 2"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
}
#[doc = "CFG0 register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "System configuration register 0"]
pub mod cfg0;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "System configuration register 1"]
pub mod cfg1;
#[doc = "EXTISS0 register accessor: an alias for `Reg<EXTISS0_SPEC>`"]
pub type EXTISS0 = crate::Reg<extiss0::EXTISS0_SPEC>;
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTISS1 register accessor: an alias for `Reg<EXTISS1_SPEC>`"]
pub type EXTISS1 = crate::Reg<extiss1::EXTISS1_SPEC>;
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTISS2 register accessor: an alias for `Reg<EXTISS2_SPEC>`"]
pub type EXTISS2 = crate::Reg<extiss2::EXTISS2_SPEC>;
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTISS3 register accessor: an alias for `Reg<EXTISS3_SPEC>`"]
pub type EXTISS3 = crate::Reg<extiss3::EXTISS3_SPEC>;
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "System configuration register 2"]
pub mod cfg2;
