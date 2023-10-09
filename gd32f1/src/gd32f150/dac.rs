#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub ctl: CTL,
    #[doc = "0x04 - software trigger register"]
    pub swt: SWT,
    #[doc = "0x08 - DAC0 12-bit right-aligned data holding register"]
    pub dac0_r12dh: DAC0_R12DH,
    #[doc = "0x0c - DAC0 12-bit left aligned data holding register"]
    pub dac0_l12dh: DAC0_L12DH,
    #[doc = "0x10 - DAC0 8-bit right aligned data holding register"]
    pub dac0_r8dh: DAC0_R8DH,
    _reserved5: [u8; 0x18],
    #[doc = "0x2c - DAC0 data output register"]
    pub dac0_do: DAC0_DO,
    _reserved6: [u8; 0x04],
    #[doc = "0x34 - status register"]
    pub stat: STAT,
}
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "SWT (w) register accessor: software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swt`]
module"]
pub type SWT = crate::Reg<swt::SWT_SPEC>;
#[doc = "software trigger register"]
pub mod swt;
#[doc = "DAC0_R12DH (rw) register accessor: DAC0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dac0_r12dh`]
module"]
pub type DAC0_R12DH = crate::Reg<dac0_r12dh::DAC0_R12DH_SPEC>;
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub mod dac0_r12dh;
#[doc = "DAC0_L12DH (rw) register accessor: DAC0 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dac0_l12dh`]
module"]
pub type DAC0_L12DH = crate::Reg<dac0_l12dh::DAC0_L12DH_SPEC>;
#[doc = "DAC0 12-bit left aligned data holding register"]
pub mod dac0_l12dh;
#[doc = "DAC0_R8DH (rw) register accessor: DAC0 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dac0_r8dh`]
module"]
pub type DAC0_R8DH = crate::Reg<dac0_r8dh::DAC0_R8DH_SPEC>;
#[doc = "DAC0 8-bit right aligned data holding register"]
pub mod dac0_r8dh;
#[doc = "DAC0_DO (r) register accessor: DAC0 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dac0_do`]
module"]
pub type DAC0_DO = crate::Reg<dac0_do::DAC0_DO_SPEC>;
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "status register"]
pub mod stat;
