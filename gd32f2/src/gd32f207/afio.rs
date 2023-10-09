#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event control register"]
    pub ec: EC,
    #[doc = "0x04 - AFIO port configuration register 0"]
    pub pcf0: PCF0,
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: EXTISS0,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: EXTISS1,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: EXTISS2,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: EXTISS3,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - AFIO port configuration register 1"]
    pub pcf1: PCF1,
    _reserved7: [u8; 0x1c],
    #[doc = "0x3c - AFIO port configuration register 2"]
    pub pcf2: PCF2,
    #[doc = "0x40 - AFIO port configuration register 3"]
    pub pcf3: PCF3,
    #[doc = "0x44 - AFIO port configuration register 4"]
    pub pcf4: PCF4,
    #[doc = "0x48 - AFIO port configuration register 5"]
    pub pcf5: PCF5,
}
#[doc = "EC (rw) register accessor: Event control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ec`]
module"]
pub type EC = crate::Reg<ec::EC_SPEC>;
#[doc = "Event control register"]
pub mod ec;
#[doc = "PCF0 (rw) register accessor: AFIO port configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf0`]
module"]
pub type PCF0 = crate::Reg<pcf0::PCF0_SPEC>;
#[doc = "AFIO port configuration register 0"]
pub mod pcf0;
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
#[doc = "PCF1 (rw) register accessor: AFIO port configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf1`]
module"]
pub type PCF1 = crate::Reg<pcf1::PCF1_SPEC>;
#[doc = "AFIO port configuration register 1"]
pub mod pcf1;
#[doc = "PCF2 (rw) register accessor: AFIO port configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf2`]
module"]
pub type PCF2 = crate::Reg<pcf2::PCF2_SPEC>;
#[doc = "AFIO port configuration register 2"]
pub mod pcf2;
#[doc = "PCF3 (rw) register accessor: AFIO port configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf3`]
module"]
pub type PCF3 = crate::Reg<pcf3::PCF3_SPEC>;
#[doc = "AFIO port configuration register 3"]
pub mod pcf3;
#[doc = "PCF4 (rw) register accessor: AFIO port configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf4`]
module"]
pub type PCF4 = crate::Reg<pcf4::PCF4_SPEC>;
#[doc = "AFIO port configuration register 4"]
pub mod pcf4;
#[doc = "PCF5 (rw) register accessor: AFIO port configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf5`]
module"]
pub type PCF5 = crate::Reg<pcf5::PCF5_SPEC>;
#[doc = "AFIO port configuration register 5"]
pub mod pcf5;
