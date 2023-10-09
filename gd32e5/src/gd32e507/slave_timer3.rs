#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHRTIMER Slave_TIMERx control register 0"]
    pub st3ctl0: ST3CTL0,
    #[doc = "0x04 - SHRTIMER Slave_TIMERx interrupt flag register"]
    pub st3intf: ST3INTF,
    #[doc = "0x08 - SHRTIMER Slave_TIMERx interrupt flag clear register"]
    pub st3intc: ST3INTC,
    #[doc = "0x0c - SHRTIMER Slave_TIMERx DMA and interrupt enable register"]
    pub st3dmainten: ST3DMAINTEN,
    #[doc = "0x10 - SHRTIMER Slave_TIMERx counter register"]
    pub st3cnt: ST3CNT,
    #[doc = "0x14 - SHRTIMER Slave_TIMER3 counter auto reload register"]
    pub st3car: ST3CAR,
    #[doc = "0x18 - SHRTIMER Slave_TIMER3 counter repetition register"]
    pub st3crep: ST3CREP,
    #[doc = "0x1c - SHRTIMER Slave_TIMER3 compare 0 value register"]
    pub st3cmp0v: ST3CMP0V,
    #[doc = "0x20 - SHRTIMER Slave_TIMERx compare 0 composite register"]
    pub st3cmp0cp: ST3CMP0CP,
    #[doc = "0x24 - SHRTIMER Slave_TIMERx compare 1 value register"]
    pub st3cmp1v: ST3CMP1V,
    #[doc = "0x28 - SHRTIMER Slave_TIMERx compare 2 value register"]
    pub st3cmp2v: ST3CMP2V,
    #[doc = "0x2c - SHRTIMER Slave_TIMERx compare 3 value register"]
    pub st3cmp3v: ST3CMP3V,
    #[doc = "0x30 - SHRTIMER Slave_TIMERx capture 0 value register"]
    pub st3cap0v: ST3CAP0V,
    #[doc = "0x34 - SHRTIMER Slave_TIMERx capture 1 value register"]
    pub st3cap1v: ST3CAP1V,
    #[doc = "0x38 - SHRTIMER Slave_TIMERx dead-time control register"]
    pub st3dtctl: ST3DTCTL,
    #[doc = "0x3c - SHRTIMER Slave_TIMERx channel 0 set request register"]
    pub st3ch0set: ST3CH0SET,
    #[doc = "0x40 - SHRTIMER Slave_TIMERx channel 0 reset request register"]
    pub st3ch0rst: ST3CH0RST,
    #[doc = "0x44 - SHRTIMER Slave_TIMERx channel 1 set request register"]
    pub st3ch1set: ST3CH1SET,
    #[doc = "0x48 - SHRTIMER Slave_TIMERx channel 1 reset request register"]
    pub st3ch1rst: ST3CH1RST,
    #[doc = "0x4c - SHRTIMER Slave_TIMERx external event filter configuration register 0"]
    pub st3exevfcfg0: ST3EXEVFCFG0,
    #[doc = "0x50 - SHRTIMER Slave_TIMERx external event filter configuration register 1"]
    pub st3exevfcfg1: ST3EXEVFCFG1,
    #[doc = "0x54 - SHRTIMER Slave_TIMERx counter reset register"]
    pub st3cntrst: ST3CNTRST,
    #[doc = "0x58 - SHRTIMER Slave_TIMERx carrier-signal control register"]
    pub st3csctl: ST3CSCTL,
    #[doc = "0x5c - SHRTIMER Slave_TIMERx capture 0 trigger register"]
    pub st3cap0trg: ST3CAP0TRG,
    #[doc = "0x60 - SHRTIMER Slave_TIMERx capture 1 trigger register"]
    pub st3cap1trg: ST3CAP1TRG,
    #[doc = "0x64 - SHRTIMER Slave_TIMERx channel output control register"]
    pub st3choctl: ST3CHOCTL,
    #[doc = "0x68 - SHRTIMER Slave_TIMERx fault control register"]
    pub st3fltctl: ST3FLTCTL,
    _reserved27: [u8; 0x10],
    #[doc = "0x7c - SHRTIMER Slave_TIMERx additional control register"]
    pub st3actl: ST3ACTL,
}
#[doc = "ST3CTL0 (rw) register accessor: SHRTIMER Slave_TIMERx control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3ctl0`]
module"]
pub type ST3CTL0 = crate::Reg<st3ctl0::ST3CTL0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx control register 0"]
pub mod st3ctl0;
#[doc = "ST3INTF (r) register accessor: SHRTIMER Slave_TIMERx interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3intf`]
module"]
pub type ST3INTF = crate::Reg<st3intf::ST3INTF_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx interrupt flag register"]
pub mod st3intf;
#[doc = "ST3INTC (w) register accessor: SHRTIMER Slave_TIMERx interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3intc`]
module"]
pub type ST3INTC = crate::Reg<st3intc::ST3INTC_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx interrupt flag clear register"]
pub mod st3intc;
#[doc = "ST3DMAINTEN (rw) register accessor: SHRTIMER Slave_TIMERx DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3dmainten`]
module"]
pub type ST3DMAINTEN = crate::Reg<st3dmainten::ST3DMAINTEN_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx DMA and interrupt enable register"]
pub mod st3dmainten;
#[doc = "ST3CNT (rw) register accessor: SHRTIMER Slave_TIMERx counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cnt`]
module"]
pub type ST3CNT = crate::Reg<st3cnt::ST3CNT_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx counter register"]
pub mod st3cnt;
#[doc = "ST3CAR (rw) register accessor: SHRTIMER Slave_TIMER3 counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3car`]
module"]
pub type ST3CAR = crate::Reg<st3car::ST3CAR_SPEC>;
#[doc = "SHRTIMER Slave_TIMER3 counter auto reload register"]
pub mod st3car;
#[doc = "ST3CREP (rw) register accessor: SHRTIMER Slave_TIMER3 counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3crep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3crep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3crep`]
module"]
pub type ST3CREP = crate::Reg<st3crep::ST3CREP_SPEC>;
#[doc = "SHRTIMER Slave_TIMER3 counter repetition register"]
pub mod st3crep;
#[doc = "ST3CMP0V (rw) register accessor: SHRTIMER Slave_TIMER3 compare 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cmp0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cmp0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cmp0v`]
module"]
pub type ST3CMP0V = crate::Reg<st3cmp0v::ST3CMP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER3 compare 0 value register"]
pub mod st3cmp0v;
#[doc = "ST3CMP0CP (rw) register accessor: SHRTIMER Slave_TIMERx compare 0 composite register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cmp0cp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cmp0cp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cmp0cp`]
module"]
pub type ST3CMP0CP = crate::Reg<st3cmp0cp::ST3CMP0CP_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 0 composite register"]
pub mod st3cmp0cp;
#[doc = "ST3CMP1V (rw) register accessor: SHRTIMER Slave_TIMERx compare 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cmp1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cmp1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cmp1v`]
module"]
pub type ST3CMP1V = crate::Reg<st3cmp1v::ST3CMP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 1 value register"]
pub mod st3cmp1v;
#[doc = "ST3CMP2V (rw) register accessor: SHRTIMER Slave_TIMERx compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cmp2v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cmp2v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cmp2v`]
module"]
pub type ST3CMP2V = crate::Reg<st3cmp2v::ST3CMP2V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 2 value register"]
pub mod st3cmp2v;
#[doc = "ST3CMP3V (rw) register accessor: SHRTIMER Slave_TIMERx compare 3 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cmp3v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cmp3v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cmp3v`]
module"]
pub type ST3CMP3V = crate::Reg<st3cmp3v::ST3CMP3V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 3 value register"]
pub mod st3cmp3v;
#[doc = "ST3CAP0V (rw) register accessor: SHRTIMER Slave_TIMERx capture 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cap0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cap0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cap0v`]
module"]
pub type ST3CAP0V = crate::Reg<st3cap0v::ST3CAP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 0 value register"]
pub mod st3cap0v;
#[doc = "ST3CAP1V (rw) register accessor: SHRTIMER Slave_TIMERx capture 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cap1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cap1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cap1v`]
module"]
pub type ST3CAP1V = crate::Reg<st3cap1v::ST3CAP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 1 value register"]
pub mod st3cap1v;
#[doc = "ST3DTCTL (rw) register accessor: SHRTIMER Slave_TIMERx dead-time control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3dtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3dtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3dtctl`]
module"]
pub type ST3DTCTL = crate::Reg<st3dtctl::ST3DTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx dead-time control register"]
pub mod st3dtctl;
#[doc = "ST3CH0SET (rw) register accessor: SHRTIMER Slave_TIMERx channel 0 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ch0set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ch0set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3ch0set`]
module"]
pub type ST3CH0SET = crate::Reg<st3ch0set::ST3CH0SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 0 set request register"]
pub mod st3ch0set;
#[doc = "ST3CH0RST (rw) register accessor: SHRTIMER Slave_TIMERx channel 0 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ch0rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ch0rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3ch0rst`]
module"]
pub type ST3CH0RST = crate::Reg<st3ch0rst::ST3CH0RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 0 reset request register"]
pub mod st3ch0rst;
#[doc = "ST3CH1SET (rw) register accessor: SHRTIMER Slave_TIMERx channel 1 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ch1set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ch1set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3ch1set`]
module"]
pub type ST3CH1SET = crate::Reg<st3ch1set::ST3CH1SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 1 set request register"]
pub mod st3ch1set;
#[doc = "ST3CH1RST (rw) register accessor: SHRTIMER Slave_TIMERx channel 1 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ch1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ch1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3ch1rst`]
module"]
pub type ST3CH1RST = crate::Reg<st3ch1rst::ST3CH1RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 1 reset request register"]
pub mod st3ch1rst;
#[doc = "ST3EXEVFCFG0 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3exevfcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3exevfcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3exevfcfg0`]
module"]
pub type ST3EXEVFCFG0 = crate::Reg<st3exevfcfg0::ST3EXEVFCFG0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 0"]
pub mod st3exevfcfg0;
#[doc = "ST3EXEVFCFG1 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3exevfcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3exevfcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3exevfcfg1`]
module"]
pub type ST3EXEVFCFG1 = crate::Reg<st3exevfcfg1::ST3EXEVFCFG1_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 1"]
pub mod st3exevfcfg1;
#[doc = "ST3CNTRST (rw) register accessor: SHRTIMER Slave_TIMERx counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cntrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cntrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cntrst`]
module"]
pub type ST3CNTRST = crate::Reg<st3cntrst::ST3CNTRST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx counter reset register"]
pub mod st3cntrst;
#[doc = "ST3CSCTL (rw) register accessor: SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3csctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3csctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3csctl`]
module"]
pub type ST3CSCTL = crate::Reg<st3csctl::ST3CSCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register"]
pub mod st3csctl;
#[doc = "ST3CAP0TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 0 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cap0trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cap0trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cap0trg`]
module"]
pub type ST3CAP0TRG = crate::Reg<st3cap0trg::ST3CAP0TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 0 trigger register"]
pub mod st3cap0trg;
#[doc = "ST3CAP1TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3cap1trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3cap1trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3cap1trg`]
module"]
pub type ST3CAP1TRG = crate::Reg<st3cap1trg::ST3CAP1TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 1 trigger register"]
pub mod st3cap1trg;
#[doc = "ST3CHOCTL (rw) register accessor: SHRTIMER Slave_TIMERx channel output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3choctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3choctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3choctl`]
module"]
pub type ST3CHOCTL = crate::Reg<st3choctl::ST3CHOCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel output control register"]
pub mod st3choctl;
#[doc = "ST3FLTCTL (rw) register accessor: SHRTIMER Slave_TIMERx fault control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3fltctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3fltctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3fltctl`]
module"]
pub type ST3FLTCTL = crate::Reg<st3fltctl::ST3FLTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx fault control register"]
pub mod st3fltctl;
#[doc = "ST3ACTL (rw) register accessor: SHRTIMER Slave_TIMERx additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3actl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3actl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3actl`]
module"]
pub type ST3ACTL = crate::Reg<st3actl::ST3ACTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx additional control register"]
pub mod st3actl;
