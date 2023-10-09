#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x5c],
    #[doc = "0x5c - OPA control register"]
    pub opa_ctl: OPA_CTL,
    #[doc = "0x60 - OPA offset trimming for normal mode register"]
    pub opa_bt: OPA_BT,
    #[doc = "0x64 - OPA offset trimming for low power mode register"]
    pub opa_lpbt: OPA_LPBT,
    _reserved3: [u8; 0x0298],
    #[doc = "0x300 - IVREF control register"]
    pub ivref_ctl: IVREF_CTL,
}
#[doc = "OPA_CTL (rw) register accessor: OPA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`opa_ctl`]
module"]
pub type OPA_CTL = crate::Reg<opa_ctl::OPA_CTL_SPEC>;
#[doc = "OPA control register"]
pub mod opa_ctl;
#[doc = "OPA_BT (rw) register accessor: OPA offset trimming for normal mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_bt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_bt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`opa_bt`]
module"]
pub type OPA_BT = crate::Reg<opa_bt::OPA_BT_SPEC>;
#[doc = "OPA offset trimming for normal mode register"]
pub mod opa_bt;
#[doc = "OPA_LPBT (rw) register accessor: OPA offset trimming for low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_lpbt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_lpbt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`opa_lpbt`]
module"]
pub type OPA_LPBT = crate::Reg<opa_lpbt::OPA_LPBT_SPEC>;
#[doc = "OPA offset trimming for low power mode register"]
pub mod opa_lpbt;
#[doc = "IVREF_CTL (rw) register accessor: IVREF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivref_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivref_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ivref_ctl`]
module"]
pub type IVREF_CTL = crate::Reg<ivref_ctl::IVREF_CTL_SPEC>;
#[doc = "IVREF control register"]
pub mod ivref_ctl;
