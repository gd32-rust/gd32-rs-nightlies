#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt enable register (EXTI_INTEN)"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x04 - Event enable register (EXTI_EVEN)"]
    pub even: crate::Reg<even::EVEN_SPEC>,
    #[doc = "0x08 - Rising Edge Trigger Enable register (EXTI_RTEN)"]
    pub rten: crate::Reg<rten::RTEN_SPEC>,
    #[doc = "0x0c - Falling Egde Trigger Enable register (EXTI_FTEN)"]
    pub ften: crate::Reg<ften::FTEN_SPEC>,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIEV)"]
    pub swiev: crate::Reg<swiev::SWIEV_SPEC>,
    #[doc = "0x14 - Pending register (EXTI_PD)"]
    pub pd: crate::Reg<pd::PD_SPEC>,
}
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register (EXTI_INTEN)"]
pub mod inten;
#[doc = "EVEN register accessor: an alias for `Reg<EVEN_SPEC>`"]
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
#[doc = "Event enable register (EXTI_EVEN)"]
pub mod even;
#[doc = "RTEN register accessor: an alias for `Reg<RTEN_SPEC>`"]
pub type RTEN = crate::Reg<rten::RTEN_SPEC>;
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
pub mod rten;
#[doc = "FTEN register accessor: an alias for `Reg<FTEN_SPEC>`"]
pub type FTEN = crate::Reg<ften::FTEN_SPEC>;
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
pub mod ften;
#[doc = "SWIEV register accessor: an alias for `Reg<SWIEV_SPEC>`"]
pub type SWIEV = crate::Reg<swiev::SWIEV_SPEC>;
#[doc = "Software interrupt event register (EXTI_SWIEV)"]
pub mod swiev;
#[doc = "PD register accessor: an alias for `Reg<PD_SPEC>`"]
pub type PD = crate::Reg<pd::PD_SPEC>;
#[doc = "Pending register (EXTI_PD)"]
pub mod pd;
