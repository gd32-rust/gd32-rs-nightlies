#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHRTIMER Slave_TIMERx control register 0"]
    pub st3ctl0: ST3CTL0,
    #[doc = "0x04 - SHRTIMER Slave_TIMERx interrupt flag register"]
    pub st4intf: ST4INTF,
    #[doc = "0x08 - SHRTIMER Slave_TIMERx interrupt flag clear register"]
    pub st4intc: ST4INTC,
    #[doc = "0x0c - SHRTIMER Slave_TIMERx DMA and interrupt enable register"]
    pub st4dmainten: ST4DMAINTEN,
    #[doc = "0x10 - SHRTIMER Slave_TIMERx counter register"]
    pub st4cnt: ST4CNT,
    #[doc = "0x14 - SHRTIMER Slave_TIMER4 counter auto reload register"]
    pub st4car: ST4CAR,
    #[doc = "0x18 - SHRTIMER Slave_TIMER4 counter repetition register"]
    pub st4crep: ST4CREP,
    #[doc = "0x1c - SHRTIMER Slave_TIMER4 compare 0 value register"]
    pub st4cmp0v: ST4CMP0V,
    #[doc = "0x20 - SHRTIMER Slave_TIMERx compare 0 composite register"]
    pub st4cmp0cp: ST4CMP0CP,
    #[doc = "0x24 - SHRTIMER Slave_TIMERx compare 1 value register"]
    pub st4cmp1v: ST4CMP1V,
    #[doc = "0x28 - SHRTIMER Slave_TIMERx compare 2 value register"]
    pub st4cmp2v: ST4CMP2V,
    #[doc = "0x2c - SHRTIMER Slave_TIMERx compare 3 value register"]
    pub st4cmp3v: ST4CMP3V,
    #[doc = "0x30 - SHRTIMER Slave_TIMERx capture 0 value register"]
    pub st4cap0v: ST4CAP0V,
    #[doc = "0x34 - SHRTIMER Slave_TIMERx capture 1 value register"]
    pub st4cap1v: ST4CAP1V,
    #[doc = "0x38 - SHRTIMER Slave_TIMERx dead-time control register"]
    pub st4dtctl: ST4DTCTL,
    #[doc = "0x3c - SHRTIMER Slave_TIMERx channel 0 set request register"]
    pub st4ch0set: ST4CH0SET,
    #[doc = "0x40 - SHRTIMER Slave_TIMERx channel 0 reset request register"]
    pub st4ch0rst: ST4CH0RST,
    #[doc = "0x44 - SHRTIMER Slave_TIMERx channel 1 set request register"]
    pub st4ch1set: ST4CH1SET,
    #[doc = "0x48 - SHRTIMER Slave_TIMERx channel 1 reset request register"]
    pub st4ch1rst: ST4CH1RST,
    #[doc = "0x4c - SHRTIMER Slave_TIMERx external event filter configuration register 0"]
    pub st4exevfcfg0: ST4EXEVFCFG0,
    #[doc = "0x50 - SHRTIMER Slave_TIMERx external event filter configuration register 1"]
    pub st4exevfcfg1: ST4EXEVFCFG1,
    #[doc = "0x54 - SHRTIMER Slave_TIMERx counter reset register"]
    pub st4cntrst: ST4CNTRST,
    #[doc = "0x58 - SHRTIMER Slave_TIMERx carrier-signal control register"]
    pub st4csctl: ST4CSCTL,
    #[doc = "0x5c - SHRTIMER Slave_TIMERx capture 0 trigger register"]
    pub st4cap0trg: ST4CAP0TRG,
    #[doc = "0x60 - SHRTIMER Slave_TIMERx capture 1 trigger register"]
    pub st4cap1trg: ST4CAP1TRG,
    #[doc = "0x64 - SHRTIMER Slave_TIMERx channel output control register"]
    pub st4choctl: ST4CHOCTL,
    #[doc = "0x68 - SHRTIMER Slave_TIMERx fault control register"]
    pub st4fltctl: ST4FLTCTL,
    _reserved27: [u8; 0x10],
    #[doc = "0x7c - SHRTIMER Slave_TIMERx additional control register"]
    pub st4actl: ST4ACTL,
}
#[doc = "ST3CTL0 (rw) register accessor: SHRTIMER Slave_TIMERx control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st3ctl0`]
module"]
pub type ST3CTL0 = crate::Reg<st3ctl0::ST3CTL0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx control register 0"]
pub mod st3ctl0;
#[doc = "ST4INTF (r) register accessor: SHRTIMER Slave_TIMERx interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4intf`]
module"]
pub type ST4INTF = crate::Reg<st4intf::ST4INTF_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx interrupt flag register"]
pub mod st4intf;
#[doc = "ST4INTC (w) register accessor: SHRTIMER Slave_TIMERx interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4intc`]
module"]
pub type ST4INTC = crate::Reg<st4intc::ST4INTC_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx interrupt flag clear register"]
pub mod st4intc;
#[doc = "ST4DMAINTEN (rw) register accessor: SHRTIMER Slave_TIMERx DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4dmainten`]
module"]
pub type ST4DMAINTEN = crate::Reg<st4dmainten::ST4DMAINTEN_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx DMA and interrupt enable register"]
pub mod st4dmainten;
#[doc = "ST4CNT (rw) register accessor: SHRTIMER Slave_TIMERx counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cnt`]
module"]
pub type ST4CNT = crate::Reg<st4cnt::ST4CNT_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx counter register"]
pub mod st4cnt;
#[doc = "ST4CAR (rw) register accessor: SHRTIMER Slave_TIMER4 counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4car`]
module"]
pub type ST4CAR = crate::Reg<st4car::ST4CAR_SPEC>;
#[doc = "SHRTIMER Slave_TIMER4 counter auto reload register"]
pub mod st4car;
#[doc = "ST4CREP (rw) register accessor: SHRTIMER Slave_TIMER4 counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4crep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4crep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4crep`]
module"]
pub type ST4CREP = crate::Reg<st4crep::ST4CREP_SPEC>;
#[doc = "SHRTIMER Slave_TIMER4 counter repetition register"]
pub mod st4crep;
#[doc = "ST4CMP0V (rw) register accessor: SHRTIMER Slave_TIMER4 compare 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cmp0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cmp0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cmp0v`]
module"]
pub type ST4CMP0V = crate::Reg<st4cmp0v::ST4CMP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER4 compare 0 value register"]
pub mod st4cmp0v;
#[doc = "ST4CMP0CP (rw) register accessor: SHRTIMER Slave_TIMERx compare 0 composite register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cmp0cp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cmp0cp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cmp0cp`]
module"]
pub type ST4CMP0CP = crate::Reg<st4cmp0cp::ST4CMP0CP_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 0 composite register"]
pub mod st4cmp0cp;
#[doc = "ST4CMP1V (rw) register accessor: SHRTIMER Slave_TIMERx compare 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cmp1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cmp1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cmp1v`]
module"]
pub type ST4CMP1V = crate::Reg<st4cmp1v::ST4CMP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 1 value register"]
pub mod st4cmp1v;
#[doc = "ST4CMP2V (rw) register accessor: SHRTIMER Slave_TIMERx compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cmp2v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cmp2v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cmp2v`]
module"]
pub type ST4CMP2V = crate::Reg<st4cmp2v::ST4CMP2V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 2 value register"]
pub mod st4cmp2v;
#[doc = "ST4CMP3V (rw) register accessor: SHRTIMER Slave_TIMERx compare 3 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cmp3v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cmp3v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cmp3v`]
module"]
pub type ST4CMP3V = crate::Reg<st4cmp3v::ST4CMP3V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 3 value register"]
pub mod st4cmp3v;
#[doc = "ST4CAP0V (rw) register accessor: SHRTIMER Slave_TIMERx capture 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cap0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cap0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cap0v`]
module"]
pub type ST4CAP0V = crate::Reg<st4cap0v::ST4CAP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 0 value register"]
pub mod st4cap0v;
#[doc = "ST4CAP1V (rw) register accessor: SHRTIMER Slave_TIMERx capture 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cap1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cap1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cap1v`]
module"]
pub type ST4CAP1V = crate::Reg<st4cap1v::ST4CAP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 1 value register"]
pub mod st4cap1v;
#[doc = "ST4DTCTL (rw) register accessor: SHRTIMER Slave_TIMERx dead-time control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4dtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4dtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4dtctl`]
module"]
pub type ST4DTCTL = crate::Reg<st4dtctl::ST4DTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx dead-time control register"]
pub mod st4dtctl;
#[doc = "ST4CH0SET (rw) register accessor: SHRTIMER Slave_TIMERx channel 0 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4ch0set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4ch0set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4ch0set`]
module"]
pub type ST4CH0SET = crate::Reg<st4ch0set::ST4CH0SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 0 set request register"]
pub mod st4ch0set;
#[doc = "ST4CH0RST (rw) register accessor: SHRTIMER Slave_TIMERx channel 0 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4ch0rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4ch0rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4ch0rst`]
module"]
pub type ST4CH0RST = crate::Reg<st4ch0rst::ST4CH0RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 0 reset request register"]
pub mod st4ch0rst;
#[doc = "ST4CH1SET (rw) register accessor: SHRTIMER Slave_TIMERx channel 1 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4ch1set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4ch1set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4ch1set`]
module"]
pub type ST4CH1SET = crate::Reg<st4ch1set::ST4CH1SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 1 set request register"]
pub mod st4ch1set;
#[doc = "ST4CH1RST (rw) register accessor: SHRTIMER Slave_TIMERx channel 1 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4ch1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4ch1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4ch1rst`]
module"]
pub type ST4CH1RST = crate::Reg<st4ch1rst::ST4CH1RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 1 reset request register"]
pub mod st4ch1rst;
#[doc = "ST4EXEVFCFG0 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4exevfcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4exevfcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4exevfcfg0`]
module"]
pub type ST4EXEVFCFG0 = crate::Reg<st4exevfcfg0::ST4EXEVFCFG0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 0"]
pub mod st4exevfcfg0;
#[doc = "ST4EXEVFCFG1 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4exevfcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4exevfcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4exevfcfg1`]
module"]
pub type ST4EXEVFCFG1 = crate::Reg<st4exevfcfg1::ST4EXEVFCFG1_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 1"]
pub mod st4exevfcfg1;
#[doc = "ST4CNTRST (rw) register accessor: SHRTIMER Slave_TIMERx counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cntrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cntrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cntrst`]
module"]
pub type ST4CNTRST = crate::Reg<st4cntrst::ST4CNTRST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx counter reset register"]
pub mod st4cntrst;
#[doc = "ST4CSCTL (rw) register accessor: SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4csctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4csctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4csctl`]
module"]
pub type ST4CSCTL = crate::Reg<st4csctl::ST4CSCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register"]
pub mod st4csctl;
#[doc = "ST4CAP0TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 0 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cap0trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cap0trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cap0trg`]
module"]
pub type ST4CAP0TRG = crate::Reg<st4cap0trg::ST4CAP0TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 0 trigger register"]
pub mod st4cap0trg;
#[doc = "ST4CAP1TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4cap1trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4cap1trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4cap1trg`]
module"]
pub type ST4CAP1TRG = crate::Reg<st4cap1trg::ST4CAP1TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 1 trigger register"]
pub mod st4cap1trg;
#[doc = "ST4CHOCTL (rw) register accessor: SHRTIMER Slave_TIMERx channel output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4choctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4choctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4choctl`]
module"]
pub type ST4CHOCTL = crate::Reg<st4choctl::ST4CHOCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel output control register"]
pub mod st4choctl;
#[doc = "ST4FLTCTL (rw) register accessor: SHRTIMER Slave_TIMERx fault control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4fltctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4fltctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4fltctl`]
module"]
pub type ST4FLTCTL = crate::Reg<st4fltctl::ST4FLTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx fault control register"]
pub mod st4fltctl;
#[doc = "ST4ACTL (rw) register accessor: SHRTIMER Slave_TIMERx additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st4actl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st4actl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st4actl`]
module"]
pub type ST4ACTL = crate::Reg<st4actl::ST4ACTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx additional control register"]
pub mod st4actl;
