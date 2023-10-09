#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x04 - Event enable register"]
    pub even: EVEN,
    #[doc = "0x08 - Rising edge trigger enable register"]
    pub rten: RTEN,
    #[doc = "0x0c - Falling Egde Trigger Enable register"]
    pub ften: FTEN,
    #[doc = "0x10 - Software interrupt event register"]
    pub swiev: SWIEV,
    #[doc = "0x14 - Pending register (EXTI_PD)"]
    pub pd: PD,
}
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "EVEN (rw) register accessor: Event enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`even::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`even::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`even`]
module"]
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
#[doc = "Event enable register"]
pub mod even;
#[doc = "RTEN (rw) register accessor: Rising edge trigger enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rten`]
module"]
pub type RTEN = crate::Reg<rten::RTEN_SPEC>;
#[doc = "Rising edge trigger enable register"]
pub mod rten;
#[doc = "FTEN (rw) register accessor: Falling Egde Trigger Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ften::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ften::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ften`]
module"]
pub type FTEN = crate::Reg<ften::FTEN_SPEC>;
#[doc = "Falling Egde Trigger Enable register"]
pub mod ften;
#[doc = "SWIEV (rw) register accessor: Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swiev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swiev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swiev`]
module"]
pub type SWIEV = crate::Reg<swiev::SWIEV_SPEC>;
#[doc = "Software interrupt event register"]
pub mod swiev;
#[doc = "PD (rw) register accessor: Pending register (EXTI_PD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pd`]
module"]
pub type PD = crate::Reg<pd::PD_SPEC>;
#[doc = "Pending register (EXTI_PD)"]
pub mod pd;
