#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - power control/status register 0"]
    pub cs0: CS0,
    #[doc = "0x08 - power control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x0c - power control and status register 1"]
    pub cs1: CS1,
}
#[doc = "CTL0 (rw) register accessor: power control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "power control register 0"]
pub mod ctl0;
#[doc = "CS0 (rw) register accessor: power control/status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cs0`]
module"]
pub type CS0 = crate::Reg<cs0::CS0_SPEC>;
#[doc = "power control/status register 0"]
pub mod cs0;
#[doc = "CTL1 (rw) register accessor: power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "power control register 1"]
pub mod ctl1;
#[doc = "CS1 (rw) register accessor: power control and status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cs1`]
module"]
pub type CS1 = crate::Reg<cs1::CS1_SPEC>;
#[doc = "power control and status register 1"]
pub mod cs1;
