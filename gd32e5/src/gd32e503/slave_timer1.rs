#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHRTIMER Slave_TIMER1 control register 0"]
    pub st1ctl0: ST1CTL0,
    #[doc = "0x04 - SHRTIMER Slave_TIMER1 interrupt flag register"]
    pub st1intf: ST1INTF,
    #[doc = "0x08 - SHRTIMER Slave_TIMER1 interrupt flag clear register"]
    pub st1intc: ST1INTC,
    #[doc = "0x0c - SHRTIMER Slave_TIMER1 DMA and interrupt enable register"]
    pub st1dmainten: ST1DMAINTEN,
    #[doc = "0x10 - SHRTIMER Slave_TIMER1 counter register"]
    pub st1cnt: ST1CNT,
    #[doc = "0x14 - SHRTIMER Slave_TIMER1 counter auto reload register"]
    pub st1car: ST1CAR,
    #[doc = "0x18 - SHRTIMER Slave_TIMER1 counter repetition register"]
    pub st1crep: ST1CREP,
    #[doc = "0x1c - SHRTIMER Slave_TIMER1 compare 0 value register"]
    pub st1cmp0v: ST1CMP0V,
    #[doc = "0x20 - SHRTIMER Slave_TIMER1 compare 0 composite register"]
    pub st1cmp0cp: ST1CMP0CP,
    #[doc = "0x24 - SHRTIMER Slave_TIMER1 compare 1 value register"]
    pub st1cmp1v: ST1CMP1V,
    #[doc = "0x28 - SHRTIMER Slave_TIMER1 compare 2 value register"]
    pub st1cmp2v: ST1CMP2V,
    #[doc = "0x2c - SHRTIMER Slave_TIMER1 compare 3 value register"]
    pub st1cmp3v: ST1CMP3V,
    #[doc = "0x30 - SHRTIMER Slave_TIMER1 capture 0 value register"]
    pub st1cap0v: ST1CAP0V,
    #[doc = "0x34 - SHRTIMER Slave_TIMER1 capture 1 value register"]
    pub st1cap1v: ST1CAP1V,
    #[doc = "0x38 - SHRTIMER Slave_TIMER1 dead-time control register"]
    pub st1dtctl: ST1DTCTL,
    #[doc = "0x3c - SHRTIMER Slave_TIMER1 channel 0 set request register"]
    pub st1ch0set: ST1CH0SET,
    #[doc = "0x40 - SHRTIMER Slave_TIMER1 channel 0 reset request register"]
    pub st1ch0rst: ST1CH0RST,
    #[doc = "0x44 - SHRTIMER Slave_TIMER1 channel 1 set request register"]
    pub st1ch1set: ST1CH1SET,
    #[doc = "0x48 - SHRTIMER Slave_TIMER1 channel 1 reset request register"]
    pub st1ch1rst: ST1CH1RST,
    #[doc = "0x4c - SHRTIMER Slave_TIMERx external event filter configuration register 0"]
    pub st1exevfcfg0: ST1EXEVFCFG0,
    #[doc = "0x50 - SHRTIMER Slave_TIMERx external event filter configuration register 1"]
    pub st1exevfcfg1: ST1EXEVFCFG1,
    #[doc = "0x54 - SHRTIMER Slave_TIMERx counter reset register"]
    pub st1cntrst: ST1CNTRST,
    #[doc = "0x58 - SHRTIMER Slave_TIMERx carrier-signal control register"]
    pub st1csctl: ST1CSCTL,
    #[doc = "0x5c - SHRTIMER Slave_TIMERx capture 0 trigger register"]
    pub st1cap0trg: ST1CAP0TRG,
    #[doc = "0x60 - SHRTIMER Slave_TIMERx capture 1 trigger register"]
    pub st1cap1trg: ST1CAP1TRG,
    #[doc = "0x64 - SHRTIMER Slave_TIMERx channel output control register"]
    pub st1choctl: ST1CHOCTL,
    #[doc = "0x68 - SHRTIMER Slave_TIMERx fault control register"]
    pub st1fltctl: ST1FLTCTL,
    _reserved27: [u8; 0x10],
    #[doc = "0x7c - SHRTIMER Slave_TIMERx additional control register"]
    pub st1actl: ST1ACTL,
}
#[doc = "ST1CTL0 (rw) register accessor: SHRTIMER Slave_TIMER1 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1ctl0`]
module"]
pub type ST1CTL0 = crate::Reg<st1ctl0::ST1CTL0_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 control register 0"]
pub mod st1ctl0;
#[doc = "ST1INTF (r) register accessor: SHRTIMER Slave_TIMER1 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1intf`]
module"]
pub type ST1INTF = crate::Reg<st1intf::ST1INTF_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 interrupt flag register"]
pub mod st1intf;
#[doc = "ST1INTC (w) register accessor: SHRTIMER Slave_TIMER1 interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1intc`]
module"]
pub type ST1INTC = crate::Reg<st1intc::ST1INTC_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 interrupt flag clear register"]
pub mod st1intc;
#[doc = "ST1DMAINTEN (rw) register accessor: SHRTIMER Slave_TIMER1 DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1dmainten`]
module"]
pub type ST1DMAINTEN = crate::Reg<st1dmainten::ST1DMAINTEN_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 DMA and interrupt enable register"]
pub mod st1dmainten;
#[doc = "ST1CNT (rw) register accessor: SHRTIMER Slave_TIMER1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cnt`]
module"]
pub type ST1CNT = crate::Reg<st1cnt::ST1CNT_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 counter register"]
pub mod st1cnt;
#[doc = "ST1CAR (rw) register accessor: SHRTIMER Slave_TIMER1 counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1car`]
module"]
pub type ST1CAR = crate::Reg<st1car::ST1CAR_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 counter auto reload register"]
pub mod st1car;
#[doc = "ST1CREP (rw) register accessor: SHRTIMER Slave_TIMER1 counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1crep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1crep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1crep`]
module"]
pub type ST1CREP = crate::Reg<st1crep::ST1CREP_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 counter repetition register"]
pub mod st1crep;
#[doc = "ST1CMP0V (rw) register accessor: SHRTIMER Slave_TIMER1 compare 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cmp0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cmp0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cmp0v`]
module"]
pub type ST1CMP0V = crate::Reg<st1cmp0v::ST1CMP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 compare 0 value register"]
pub mod st1cmp0v;
#[doc = "ST1CMP0CP (rw) register accessor: SHRTIMER Slave_TIMER1 compare 0 composite register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cmp0cp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cmp0cp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cmp0cp`]
module"]
pub type ST1CMP0CP = crate::Reg<st1cmp0cp::ST1CMP0CP_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 compare 0 composite register"]
pub mod st1cmp0cp;
#[doc = "ST1CMP1V (rw) register accessor: SHRTIMER Slave_TIMER1 compare 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cmp1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cmp1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cmp1v`]
module"]
pub type ST1CMP1V = crate::Reg<st1cmp1v::ST1CMP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 compare 1 value register"]
pub mod st1cmp1v;
#[doc = "ST1CMP2V (rw) register accessor: SHRTIMER Slave_TIMER1 compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cmp2v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cmp2v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cmp2v`]
module"]
pub type ST1CMP2V = crate::Reg<st1cmp2v::ST1CMP2V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 compare 2 value register"]
pub mod st1cmp2v;
#[doc = "ST1CMP3V (rw) register accessor: SHRTIMER Slave_TIMER1 compare 3 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cmp3v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cmp3v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cmp3v`]
module"]
pub type ST1CMP3V = crate::Reg<st1cmp3v::ST1CMP3V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 compare 3 value register"]
pub mod st1cmp3v;
#[doc = "ST1CAP0V (rw) register accessor: SHRTIMER Slave_TIMER1 capture 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cap0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cap0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cap0v`]
module"]
pub type ST1CAP0V = crate::Reg<st1cap0v::ST1CAP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 capture 0 value register"]
pub mod st1cap0v;
#[doc = "ST1CAP1V (rw) register accessor: SHRTIMER Slave_TIMER1 capture 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cap1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cap1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cap1v`]
module"]
pub type ST1CAP1V = crate::Reg<st1cap1v::ST1CAP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 capture 1 value register"]
pub mod st1cap1v;
#[doc = "ST1DTCTL (rw) register accessor: SHRTIMER Slave_TIMER1 dead-time control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1dtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1dtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1dtctl`]
module"]
pub type ST1DTCTL = crate::Reg<st1dtctl::ST1DTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 dead-time control register"]
pub mod st1dtctl;
#[doc = "ST1CH0SET (rw) register accessor: SHRTIMER Slave_TIMER1 channel 0 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1ch0set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1ch0set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1ch0set`]
module"]
pub type ST1CH0SET = crate::Reg<st1ch0set::ST1CH0SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 channel 0 set request register"]
pub mod st1ch0set;
#[doc = "ST1CH0RST (rw) register accessor: SHRTIMER Slave_TIMER1 channel 0 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1ch0rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1ch0rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1ch0rst`]
module"]
pub type ST1CH0RST = crate::Reg<st1ch0rst::ST1CH0RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 channel 0 reset request register"]
pub mod st1ch0rst;
#[doc = "ST1CH1SET (rw) register accessor: SHRTIMER Slave_TIMER1 channel 1 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1ch1set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1ch1set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1ch1set`]
module"]
pub type ST1CH1SET = crate::Reg<st1ch1set::ST1CH1SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 channel 1 set request register"]
pub mod st1ch1set;
#[doc = "ST1CH1RST (rw) register accessor: SHRTIMER Slave_TIMER1 channel 1 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1ch1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1ch1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1ch1rst`]
module"]
pub type ST1CH1RST = crate::Reg<st1ch1rst::ST1CH1RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMER1 channel 1 reset request register"]
pub mod st1ch1rst;
#[doc = "ST1EXEVFCFG0 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1exevfcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1exevfcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1exevfcfg0`]
module"]
pub type ST1EXEVFCFG0 = crate::Reg<st1exevfcfg0::ST1EXEVFCFG0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 0"]
pub mod st1exevfcfg0;
#[doc = "ST1EXEVFCFG1 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1exevfcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1exevfcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1exevfcfg1`]
module"]
pub type ST1EXEVFCFG1 = crate::Reg<st1exevfcfg1::ST1EXEVFCFG1_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 1"]
pub mod st1exevfcfg1;
#[doc = "ST1CNTRST (rw) register accessor: SHRTIMER Slave_TIMERx counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cntrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cntrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cntrst`]
module"]
pub type ST1CNTRST = crate::Reg<st1cntrst::ST1CNTRST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx counter reset register"]
pub mod st1cntrst;
#[doc = "ST1CSCTL (rw) register accessor: SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1csctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1csctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1csctl`]
module"]
pub type ST1CSCTL = crate::Reg<st1csctl::ST1CSCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register"]
pub mod st1csctl;
#[doc = "ST1CAP0TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 0 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cap0trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cap0trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cap0trg`]
module"]
pub type ST1CAP0TRG = crate::Reg<st1cap0trg::ST1CAP0TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 0 trigger register"]
pub mod st1cap0trg;
#[doc = "ST1CAP1TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1cap1trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1cap1trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1cap1trg`]
module"]
pub type ST1CAP1TRG = crate::Reg<st1cap1trg::ST1CAP1TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 1 trigger register"]
pub mod st1cap1trg;
#[doc = "ST1CHOCTL (rw) register accessor: SHRTIMER Slave_TIMERx channel output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1choctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1choctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1choctl`]
module"]
pub type ST1CHOCTL = crate::Reg<st1choctl::ST1CHOCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel output control register"]
pub mod st1choctl;
#[doc = "ST1FLTCTL (rw) register accessor: SHRTIMER Slave_TIMERx fault control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1fltctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1fltctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1fltctl`]
module"]
pub type ST1FLTCTL = crate::Reg<st1fltctl::ST1FLTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx fault control register"]
pub mod st1fltctl;
#[doc = "ST1ACTL (rw) register accessor: SHRTIMER Slave_TIMERx additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1actl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1actl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st1actl`]
module"]
pub type ST1ACTL = crate::Reg<st1actl::ST1ACTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx additional control register"]
pub mod st1actl;
