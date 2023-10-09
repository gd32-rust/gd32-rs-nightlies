#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MSC control register (MSC_CTL)"]
    pub msc_ctl: MSC_CTL,
    #[doc = "0x04 - Ethernet MSC receive interrupt flag register (MSC_RINTF)"]
    pub msc_rintf: MSC_RINTF,
    #[doc = "0x08 - Ethernet MSC transmit interrupt flag register (MSC_TINTF)"]
    pub msc_tintf: MSC_TINTF,
    #[doc = "0x0c - Ethernet MSC receive interrupt mask register (MSC_RINTMSK)"]
    pub msc_rintmsk: MSC_RINTMSK,
    #[doc = "0x10 - Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)"]
    pub msc_tintmsk: MSC_TINTMSK,
    _reserved5: [u8; 0x38],
    #[doc = "0x4c - Ethernet MSC transmitted good frames after a single collision counter"]
    pub msc_sccnt: MSC_SCCNT,
    #[doc = "0x50 - Ethernet MSC transmitted good frames after more than a single collision"]
    pub msc_msccnt: MSC_MSCCNT,
    _reserved7: [u8; 0x14],
    #[doc = "0x68 - Ethernet MSC transmitted good frames counter register"]
    pub msc_tgfcnt: MSC_TGFCNT,
    _reserved8: [u8; 0x28],
    #[doc = "0x94 - Ethernet MSC received frames with CRC error counter register"]
    pub msc_rfcecnt: MSC_RFCECNT,
    #[doc = "0x98 - Ethernet MSC received frames with alignment error counter register"]
    pub msc_rfaecnt: MSC_RFAECNT,
    _reserved10: [u8; 0x28],
    #[doc = "0xc4 - MSC received good unicast frames counter register"]
    pub msc_rgufcnt: MSC_RGUFCNT,
}
#[doc = "MSC_CTL (rw) register accessor: Ethernet MSC control register (MSC_CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_ctl`]
module"]
pub type MSC_CTL = crate::Reg<msc_ctl::MSC_CTL_SPEC>;
#[doc = "Ethernet MSC control register (MSC_CTL)"]
pub mod msc_ctl;
#[doc = "MSC_RINTF (r) register accessor: Ethernet MSC receive interrupt flag register (MSC_RINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rintf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_rintf`]
module"]
pub type MSC_RINTF = crate::Reg<msc_rintf::MSC_RINTF_SPEC>;
#[doc = "Ethernet MSC receive interrupt flag register (MSC_RINTF)"]
pub mod msc_rintf;
#[doc = "MSC_TINTF (r) register accessor: Ethernet MSC transmit interrupt flag register (MSC_TINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_tintf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_tintf`]
module"]
pub type MSC_TINTF = crate::Reg<msc_tintf::MSC_TINTF_SPEC>;
#[doc = "Ethernet MSC transmit interrupt flag register (MSC_TINTF)"]
pub mod msc_tintf;
#[doc = "MSC_RINTMSK (rw) register accessor: Ethernet MSC receive interrupt mask register (MSC_RINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msc_rintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_rintmsk`]
module"]
pub type MSC_RINTMSK = crate::Reg<msc_rintmsk::MSC_RINTMSK_SPEC>;
#[doc = "Ethernet MSC receive interrupt mask register (MSC_RINTMSK)"]
pub mod msc_rintmsk;
#[doc = "MSC_TINTMSK (rw) register accessor: Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_tintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msc_tintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_tintmsk`]
module"]
pub type MSC_TINTMSK = crate::Reg<msc_tintmsk::MSC_TINTMSK_SPEC>;
#[doc = "Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)"]
pub mod msc_tintmsk;
#[doc = "MSC_SCCNT (r) register accessor: Ethernet MSC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_sccnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_sccnt`]
module"]
pub type MSC_SCCNT = crate::Reg<msc_sccnt::MSC_SCCNT_SPEC>;
#[doc = "Ethernet MSC transmitted good frames after a single collision counter"]
pub mod msc_sccnt;
#[doc = "MSC_MSCCNT (r) register accessor: Ethernet MSC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_msccnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_msccnt`]
module"]
pub type MSC_MSCCNT = crate::Reg<msc_msccnt::MSC_MSCCNT_SPEC>;
#[doc = "Ethernet MSC transmitted good frames after more than a single collision"]
pub mod msc_msccnt;
#[doc = "MSC_TGFCNT (r) register accessor: Ethernet MSC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_tgfcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_tgfcnt`]
module"]
pub type MSC_TGFCNT = crate::Reg<msc_tgfcnt::MSC_TGFCNT_SPEC>;
#[doc = "Ethernet MSC transmitted good frames counter register"]
pub mod msc_tgfcnt;
#[doc = "MSC_RFCECNT (r) register accessor: Ethernet MSC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rfcecnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_rfcecnt`]
module"]
pub type MSC_RFCECNT = crate::Reg<msc_rfcecnt::MSC_RFCECNT_SPEC>;
#[doc = "Ethernet MSC received frames with CRC error counter register"]
pub mod msc_rfcecnt;
#[doc = "MSC_RFAECNT (r) register accessor: Ethernet MSC received frames with alignment error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rfaecnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_rfaecnt`]
module"]
pub type MSC_RFAECNT = crate::Reg<msc_rfaecnt::MSC_RFAECNT_SPEC>;
#[doc = "Ethernet MSC received frames with alignment error counter register"]
pub mod msc_rfaecnt;
#[doc = "MSC_RGUFCNT (r) register accessor: MSC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rgufcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msc_rgufcnt`]
module"]
pub type MSC_RGUFCNT = crate::Reg<msc_rgufcnt::MSC_RGUFCNT_SPEC>;
#[doc = "MSC received good unicast frames counter register"]
pub mod msc_rgufcnt;
