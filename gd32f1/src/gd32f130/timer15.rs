#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved2: [u8; 0x06],
    #[doc = "0x0c - DMA and interrupt enable register"]
    pub dmainten: DMAINTEN,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - interrupt flag register"]
    pub intf: INTF,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Software event generation register"]
    pub swevg: SWEVG,
    _reserved5: [u8; 0x02],
    _reserved_5_chctl0: [u8; 0x02],
    _reserved6: [u8; 0x06],
    #[doc = "0x20 - Channel control register 2"]
    pub chctl2: CHCTL2,
    _reserved7: [u8; 0x02],
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    _reserved8: [u8; 0x02],
    #[doc = "0x28 - Prescaler register"]
    pub psc: PSC,
    _reserved9: [u8; 0x02],
    #[doc = "0x2c - Counter auto reload register"]
    pub car: CAR,
    _reserved10: [u8; 0x02],
    #[doc = "0x30 - Counter repetition register"]
    pub crep: CREP,
    _reserved11: [u8; 0x02],
    #[doc = "0x34 - Channel 0 capture/compare value register"]
    pub ch0cv: CH0CV,
    _reserved12: [u8; 0x0e],
    #[doc = "0x44 - Channel Complementary Protection register"]
    pub cchp: CCHP,
    _reserved13: [u8; 0x02],
    #[doc = "0x48 - DMA configuration register"]
    pub dmacfg: DMACFG,
    _reserved14: [u8; 0x02],
    #[doc = "0x4c - DMA transfer buffer register"]
    pub dmatb: DMATB,
}
impl RegisterBlock {
    #[doc = "0x18 - Channel control register 0 (input mode)"]
    #[inline(always)]
    pub const fn chctl0_input(&self) -> &CHCTL0_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Channel control register 0 (output mode)"]
    #[inline(always)]
    pub const fn chctl0_output(&self) -> &CHCTL0_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
#[doc = "CTL0 (rw) register accessor: control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "DMAINTEN (rw) register accessor: DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmainten`]
module"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "DMA and interrupt enable register"]
pub mod dmainten;
#[doc = "INTF (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "SWEVG (w) register accessor: Software event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swevg`]
module"]
pub type SWEVG = crate::Reg<swevg::SWEVG_SPEC>;
#[doc = "Software event generation register"]
pub mod swevg;
#[doc = "CHCTL0_Output (rw) register accessor: Channel control register 0 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chctl0_output`]
module"]
pub type CHCTL0_OUTPUT = crate::Reg<chctl0_output::CHCTL0_OUTPUT_SPEC>;
#[doc = "Channel control register 0 (output mode)"]
pub mod chctl0_output;
#[doc = "CHCTL0_Input (rw) register accessor: Channel control register 0 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chctl0_input`]
module"]
pub type CHCTL0_INPUT = crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC>;
#[doc = "Channel control register 0 (input mode)"]
pub mod chctl0_input;
#[doc = "CHCTL2 (rw) register accessor: Channel control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chctl2`]
module"]
pub type CHCTL2 = crate::Reg<chctl2::CHCTL2_SPEC>;
#[doc = "Channel control register 2"]
pub mod chctl2;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psc`]
module"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "CAR (rw) register accessor: Counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`car`]
module"]
pub type CAR = crate::Reg<car::CAR_SPEC>;
#[doc = "Counter auto reload register"]
pub mod car;
#[doc = "CREP (rw) register accessor: Counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crep`]
module"]
pub type CREP = crate::Reg<crep::CREP_SPEC>;
#[doc = "Counter repetition register"]
pub mod crep;
#[doc = "CH0CV (rw) register accessor: Channel 0 capture/compare value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ch0cv`]
module"]
pub type CH0CV = crate::Reg<ch0cv::CH0CV_SPEC>;
#[doc = "Channel 0 capture/compare value register"]
pub mod ch0cv;
#[doc = "CCHP (rw) register accessor: Channel Complementary Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cchp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cchp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cchp`]
module"]
pub type CCHP = crate::Reg<cchp::CCHP_SPEC>;
#[doc = "Channel Complementary Protection register"]
pub mod cchp;
#[doc = "DMACFG (rw) register accessor: DMA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacfg`]
module"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA configuration register"]
pub mod dmacfg;
#[doc = "DMATB (rw) register accessor: DMA transfer buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatb`]
module"]
pub type DMATB = crate::Reg<dmatb::DMATB_SPEC>;
#[doc = "DMA transfer buffer register"]
pub mod dmatb;
