#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System configuration register 0"]
    pub cfg0: CFG0,
    #[doc = "0x04 - System configuration register 1"]
    pub cfg1: CFG1,
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: EXTISS0,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: EXTISS1,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: EXTISS2,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: EXTISS3,
    #[doc = "0x18 - System configuration register 2"]
    pub cfg2: CFG2,
}
#[doc = "CFG0 (rw) register accessor: System configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfg0`]
module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "System configuration register 0"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: System configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "System configuration register 1"]
pub mod cfg1;
#[doc = "EXTISS0 (rw) register accessor: EXTI sources selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss0`]
module"]
pub type EXTISS0 = crate::Reg<extiss0::EXTISS0_SPEC>;
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTISS1 (rw) register accessor: EXTI sources selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss1`]
module"]
pub type EXTISS1 = crate::Reg<extiss1::EXTISS1_SPEC>;
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTISS2 (rw) register accessor: EXTI sources selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss2`]
module"]
pub type EXTISS2 = crate::Reg<extiss2::EXTISS2_SPEC>;
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTISS3 (rw) register accessor: EXTI sources selection register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss3`]
module"]
pub type EXTISS3 = crate::Reg<extiss3::EXTISS3_SPEC>;
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "CFG2 (rw) register accessor: System configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfg2`]
module"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "System configuration register 2"]
pub mod cfg2;
