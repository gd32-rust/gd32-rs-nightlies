#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - software trigger register"]
    pub swt: SWT,
    #[doc = "0x08 - DAC_OUT0 12-bit right-aligned data holding register"]
    pub out0_r12dh: OUT0_R12DH,
    #[doc = "0x0c - DAC_OUT0 12-bit left-aligned data holding register"]
    pub out0_l12dh: OUT0_L12DH,
    #[doc = "0x10 - DAC_OUT0 8-bit right aligned data holding register"]
    pub out0_r8dh: OUT0_R8DH,
    #[doc = "0x14 - DAC_OUT1 12-bit right-aligned data holding register"]
    pub out1_r12dh: OUT1_R12DH,
    #[doc = "0x18 - DAC_OUT1 12-bit left aligned data holding register"]
    pub out1_l12dh: OUT1_L12DH,
    #[doc = "0x1c - DAC_OUT1 8-bit right aligned data holding register"]
    pub out1_r8dh: OUT1_R8DH,
    #[doc = "0x20 - DAC concurrent mode 12-bit right-aligned data holding register"]
    pub dacc_r12dh: DACC_R12DH,
    #[doc = "0x24 - DAC concurrent mode 12-bit left aligned data holding register"]
    pub dacc_l12dh: DACC_L12DH,
    #[doc = "0x28 - DAC concurrent mode 8-bit right aligned data holding register"]
    pub dacc_r8dh: DACC_R8DH,
    #[doc = "0x2c - DAC_OUT0 data output register"]
    pub out0_do: OUT0_DO,
    #[doc = "0x30 - DAC_OUT1 data output register"]
    pub out1_do: OUT1_DO,
    #[doc = "0x34 - DAC Status register 0"]
    pub stat0: STAT0,
    _reserved14: [u8; 0x48],
    #[doc = "0x80 - DAC Control Register 1"]
    pub ctl1: CTL1,
    #[doc = "0x84 - DAC Status register 1"]
    pub stat1: STAT1,
}
#[doc = "CTL0 (rw) register accessor: control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "SWT (w) register accessor: software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swt`]
module"]
pub type SWT = crate::Reg<swt::SWT_SPEC>;
#[doc = "software trigger register"]
pub mod swt;
#[doc = "OUT0_R12DH (rw) register accessor: DAC_OUT0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out0_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out0_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out0_r12dh`]
module"]
pub type OUT0_R12DH = crate::Reg<out0_r12dh::OUT0_R12DH_SPEC>;
#[doc = "DAC_OUT0 12-bit right-aligned data holding register"]
pub mod out0_r12dh;
#[doc = "OUT0_L12DH (rw) register accessor: DAC_OUT0 12-bit left-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out0_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out0_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out0_l12dh`]
module"]
pub type OUT0_L12DH = crate::Reg<out0_l12dh::OUT0_L12DH_SPEC>;
#[doc = "DAC_OUT0 12-bit left-aligned data holding register"]
pub mod out0_l12dh;
#[doc = "OUT0_R8DH (rw) register accessor: DAC_OUT0 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out0_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out0_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out0_r8dh`]
module"]
pub type OUT0_R8DH = crate::Reg<out0_r8dh::OUT0_R8DH_SPEC>;
#[doc = "DAC_OUT0 8-bit right aligned data holding register"]
pub mod out0_r8dh;
#[doc = "OUT1_R12DH (rw) register accessor: DAC_OUT1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_r12dh`]
module"]
pub type OUT1_R12DH = crate::Reg<out1_r12dh::OUT1_R12DH_SPEC>;
#[doc = "DAC_OUT1 12-bit right-aligned data holding register"]
pub mod out1_r12dh;
#[doc = "OUT1_L12DH (rw) register accessor: DAC_OUT1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_l12dh`]
module"]
pub type OUT1_L12DH = crate::Reg<out1_l12dh::OUT1_L12DH_SPEC>;
#[doc = "DAC_OUT1 12-bit left aligned data holding register"]
pub mod out1_l12dh;
#[doc = "OUT1_R8DH (rw) register accessor: DAC_OUT1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_r8dh`]
module"]
pub type OUT1_R8DH = crate::Reg<out1_r8dh::OUT1_R8DH_SPEC>;
#[doc = "DAC_OUT1 8-bit right aligned data holding register"]
pub mod out1_r8dh;
#[doc = "DACC_R12DH (rw) register accessor: DAC concurrent mode 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dacc_r12dh`]
module"]
pub type DACC_R12DH = crate::Reg<dacc_r12dh::DACC_R12DH_SPEC>;
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register"]
pub mod dacc_r12dh;
#[doc = "DACC_L12DH (rw) register accessor: DAC concurrent mode 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dacc_l12dh`]
module"]
pub type DACC_L12DH = crate::Reg<dacc_l12dh::DACC_L12DH_SPEC>;
#[doc = "DAC concurrent mode 12-bit left aligned data holding register"]
pub mod dacc_l12dh;
#[doc = "DACC_R8DH (rw) register accessor: DAC concurrent mode 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dacc_r8dh`]
module"]
pub type DACC_R8DH = crate::Reg<dacc_r8dh::DACC_R8DH_SPEC>;
#[doc = "DAC concurrent mode 8-bit right aligned data holding register"]
pub mod dacc_r8dh;
#[doc = "OUT0_DO (r) register accessor: DAC_OUT0 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out0_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out0_do`]
module"]
pub type OUT0_DO = crate::Reg<out0_do::OUT0_DO_SPEC>;
#[doc = "DAC_OUT0 data output register"]
pub mod out0_do;
#[doc = "OUT1_DO (r) register accessor: DAC_OUT1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_do`]
module"]
pub type OUT1_DO = crate::Reg<out1_do::OUT1_DO_SPEC>;
#[doc = "DAC_OUT1 data output register"]
pub mod out1_do;
#[doc = "STAT0 (rw) register accessor: DAC Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat0`]
module"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "DAC Status register 0"]
pub mod stat0;
#[doc = "CTL1 (rw) register accessor: DAC Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "DAC Control Register 1"]
pub mod ctl1;
#[doc = "STAT1 (rw) register accessor: DAC Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat1`]
module"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "DAC Status register 1"]
pub mod stat1;
