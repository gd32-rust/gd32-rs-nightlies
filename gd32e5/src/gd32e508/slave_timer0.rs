#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHRTIMER Slave_TIMER0 control register 0"]
    pub st0ctl0: ST0CTL0,
    #[doc = "0x04 - SHRTIMER Slave_TIMER0 interrupt flag register"]
    pub st0intf: ST0INTF,
    #[doc = "0x08 - SHRTIMER Slave_TIMER0 interrupt flag clear register"]
    pub st0intc: ST0INTC,
    #[doc = "0x0c - SHRTIMER Slave_TIMER0 DMA and interrupt enable register"]
    pub st0dmainten: ST0DMAINTEN,
    #[doc = "0x10 - SHRTIMER Slave_TIMER0 counter register"]
    pub st0cnt: ST0CNT,
    #[doc = "0x14 - SHRTIMER Slave_TIMER0 counter auto reload register"]
    pub st0car: ST0CAR,
    #[doc = "0x18 - SHRTIMER Slave_TIMER0 counter repetition register"]
    pub st0crep: ST0CREP,
    #[doc = "0x1c - SHRTIMER Slave_TIMER0 compare 0 value register"]
    pub st0cmp0v: ST0CMP0V,
    #[doc = "0x20 - SHRTIMER Slave_TIMER0 compare 0 composite register"]
    pub st0cmp0cp: ST0CMP0CP,
    #[doc = "0x24 - SHRTIMER Slave_TIMER0 compare 1 value register"]
    pub st0cmp1v: ST0CMP1V,
    #[doc = "0x28 - SHRTIMER Slave_TIMER0 compare 2 value register"]
    pub st0cmp2v: ST0CMP2V,
    #[doc = "0x2c - SHRTIMER Slave_TIMER0 compare 3 value register"]
    pub st0cmp3v: ST0CMP3V,
    #[doc = "0x30 - SHRTIMER Slave_TIMER0 capture 0 value register"]
    pub st0cap0v: ST0CAP0V,
    #[doc = "0x34 - SHRTIMER Slave_TIMER0 capture 1 value register"]
    pub st0cap1v: ST0CAP1V,
    #[doc = "0x38 - SHRTIMER Slave_TIMER0 dead-time control register"]
    pub st0dtctl: ST0DTCTL,
    #[doc = "0x3c - SHRTIMER Slave_TIMERx channel 0 set request register"]
    pub st0ch0set: ST0CH0SET,
    #[doc = "0x40 - SHRTIMER Slave_TIMER0 channel 0 reset request register"]
    pub st0ch0rst: ST0CH0RST,
    #[doc = "0x44 - SHRTIMER Slave_TIMER0 channel 1 set request register"]
    pub st0ch1set: ST0CH1SET,
    #[doc = "0x48 - SHRTIMER Slave_TIMER0 channel 1 reset request register"]
    pub st0ch1rst: ST0CH1RST,
    #[doc = "0x4c - SHRTIMER Slave_TIMERx external event filter configuration register 0"]
    pub st0exevfcfg0: ST0EXEVFCFG0,
    #[doc = "0x50 - SHRTIMER Slave_TIMERx external event filter configuration register 1"]
    pub st0exevfcfg1: ST0EXEVFCFG1,
    #[doc = "0x54 - SHRTIMER Slave_TIMER0 counter reset register"]
    pub st0cntrst: ST0CNTRST,
    #[doc = "0x58 - SHRTIMER Slave_TIMERx carrier-signal control register"]
    pub st0csctl: ST0CSCTL,
    #[doc = "0x5c - SHRTIMER Slave_TIMER0 capture 0 trigger register"]
    pub st0cap0trg: ST0CAP0TRG,
    #[doc = "0x60 - SHRTIMER Slave_TIMER0 capture 1 trigger register"]
    pub st0cap1trg: ST0CAP1TRG,
    #[doc = "0x64 - SHRTIMER Slave_TIMERx channel output control register"]
    pub st0choctl: ST0CHOCTL,
    #[doc = "0x68 - SHRTIMER Slave_TIMERx fault control register"]
    pub st0fltctl: ST0FLTCTL,
    _reserved27: [u8; 0x10],
    #[doc = "0x7c - SHRTIMER Slave_TIMERx additional control register"]
    pub st0actl: ST0ACTL,
}
#[doc = "ST0CTL0 (rw) register accessor: SHRTIMER Slave_TIMER0 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0ctl0`]
module"]
pub type ST0CTL0 = crate::Reg<st0ctl0::ST0CTL0_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 control register 0"]
pub mod st0ctl0;
#[doc = "ST0INTF (r) register accessor: SHRTIMER Slave_TIMER0 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0intf`]
module"]
pub type ST0INTF = crate::Reg<st0intf::ST0INTF_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 interrupt flag register"]
pub mod st0intf;
#[doc = "ST0INTC (w) register accessor: SHRTIMER Slave_TIMER0 interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0intc`]
module"]
pub type ST0INTC = crate::Reg<st0intc::ST0INTC_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 interrupt flag clear register"]
pub mod st0intc;
#[doc = "ST0DMAINTEN (rw) register accessor: SHRTIMER Slave_TIMER0 DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0dmainten`]
module"]
pub type ST0DMAINTEN = crate::Reg<st0dmainten::ST0DMAINTEN_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 DMA and interrupt enable register"]
pub mod st0dmainten;
#[doc = "ST0CNT (rw) register accessor: SHRTIMER Slave_TIMER0 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cnt`]
module"]
pub type ST0CNT = crate::Reg<st0cnt::ST0CNT_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 counter register"]
pub mod st0cnt;
#[doc = "ST0CAR (rw) register accessor: SHRTIMER Slave_TIMER0 counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0car`]
module"]
pub type ST0CAR = crate::Reg<st0car::ST0CAR_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 counter auto reload register"]
pub mod st0car;
#[doc = "ST0CREP (rw) register accessor: SHRTIMER Slave_TIMER0 counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0crep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0crep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0crep`]
module"]
pub type ST0CREP = crate::Reg<st0crep::ST0CREP_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 counter repetition register"]
pub mod st0crep;
#[doc = "ST0CMP0V (rw) register accessor: SHRTIMER Slave_TIMER0 compare 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cmp0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cmp0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cmp0v`]
module"]
pub type ST0CMP0V = crate::Reg<st0cmp0v::ST0CMP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 compare 0 value register"]
pub mod st0cmp0v;
#[doc = "ST0CMP0CP (rw) register accessor: SHRTIMER Slave_TIMER0 compare 0 composite register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cmp0cp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cmp0cp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cmp0cp`]
module"]
pub type ST0CMP0CP = crate::Reg<st0cmp0cp::ST0CMP0CP_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 compare 0 composite register"]
pub mod st0cmp0cp;
#[doc = "ST0CMP1V (rw) register accessor: SHRTIMER Slave_TIMER0 compare 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cmp1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cmp1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cmp1v`]
module"]
pub type ST0CMP1V = crate::Reg<st0cmp1v::ST0CMP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 compare 1 value register"]
pub mod st0cmp1v;
#[doc = "ST0CMP2V (rw) register accessor: SHRTIMER Slave_TIMER0 compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cmp2v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cmp2v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cmp2v`]
module"]
pub type ST0CMP2V = crate::Reg<st0cmp2v::ST0CMP2V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 compare 2 value register"]
pub mod st0cmp2v;
#[doc = "ST0CMP3V (rw) register accessor: SHRTIMER Slave_TIMER0 compare 3 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cmp3v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cmp3v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cmp3v`]
module"]
pub type ST0CMP3V = crate::Reg<st0cmp3v::ST0CMP3V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 compare 3 value register"]
pub mod st0cmp3v;
#[doc = "ST0CAP0V (rw) register accessor: SHRTIMER Slave_TIMER0 capture 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cap0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cap0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cap0v`]
module"]
pub type ST0CAP0V = crate::Reg<st0cap0v::ST0CAP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 capture 0 value register"]
pub mod st0cap0v;
#[doc = "ST0CAP1V (rw) register accessor: SHRTIMER Slave_TIMER0 capture 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cap1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cap1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cap1v`]
module"]
pub type ST0CAP1V = crate::Reg<st0cap1v::ST0CAP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 capture 1 value register"]
pub mod st0cap1v;
#[doc = "ST0DTCTL (rw) register accessor: SHRTIMER Slave_TIMER0 dead-time control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0dtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0dtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0dtctl`]
module"]
pub type ST0DTCTL = crate::Reg<st0dtctl::ST0DTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 dead-time control register"]
pub mod st0dtctl;
#[doc = "ST0CH0SET (rw) register accessor: SHRTIMER Slave_TIMERx channel 0 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0ch0set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0ch0set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0ch0set`]
module"]
pub type ST0CH0SET = crate::Reg<st0ch0set::ST0CH0SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 0 set request register"]
pub mod st0ch0set;
#[doc = "ST0CH0RST (rw) register accessor: SHRTIMER Slave_TIMER0 channel 0 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0ch0rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0ch0rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0ch0rst`]
module"]
pub type ST0CH0RST = crate::Reg<st0ch0rst::ST0CH0RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 channel 0 reset request register"]
pub mod st0ch0rst;
#[doc = "ST0CH1SET (rw) register accessor: SHRTIMER Slave_TIMER0 channel 1 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0ch1set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0ch1set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0ch1set`]
module"]
pub type ST0CH1SET = crate::Reg<st0ch1set::ST0CH1SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 channel 1 set request register"]
pub mod st0ch1set;
#[doc = "ST0CH1RST (rw) register accessor: SHRTIMER Slave_TIMER0 channel 1 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0ch1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0ch1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0ch1rst`]
module"]
pub type ST0CH1RST = crate::Reg<st0ch1rst::ST0CH1RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 channel 1 reset request register"]
pub mod st0ch1rst;
#[doc = "ST0EXEVFCFG0 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0exevfcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0exevfcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0exevfcfg0`]
module"]
pub type ST0EXEVFCFG0 = crate::Reg<st0exevfcfg0::ST0EXEVFCFG0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 0"]
pub mod st0exevfcfg0;
#[doc = "ST0EXEVFCFG1 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0exevfcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0exevfcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0exevfcfg1`]
module"]
pub type ST0EXEVFCFG1 = crate::Reg<st0exevfcfg1::ST0EXEVFCFG1_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 1"]
pub mod st0exevfcfg1;
#[doc = "ST0CNTRST (rw) register accessor: SHRTIMER Slave_TIMER0 counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cntrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cntrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cntrst`]
module"]
pub type ST0CNTRST = crate::Reg<st0cntrst::ST0CNTRST_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 counter reset register"]
pub mod st0cntrst;
#[doc = "ST0CSCTL (rw) register accessor: SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0csctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0csctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0csctl`]
module"]
pub type ST0CSCTL = crate::Reg<st0csctl::ST0CSCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register"]
pub mod st0csctl;
#[doc = "ST0CAP0TRG (rw) register accessor: SHRTIMER Slave_TIMER0 capture 0 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cap0trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cap0trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cap0trg`]
module"]
pub type ST0CAP0TRG = crate::Reg<st0cap0trg::ST0CAP0TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 capture 0 trigger register"]
pub mod st0cap0trg;
#[doc = "ST0CAP1TRG (rw) register accessor: SHRTIMER Slave_TIMER0 capture 1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cap1trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cap1trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0cap1trg`]
module"]
pub type ST0CAP1TRG = crate::Reg<st0cap1trg::ST0CAP1TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMER0 capture 1 trigger register"]
pub mod st0cap1trg;
#[doc = "ST0CHOCTL (rw) register accessor: SHRTIMER Slave_TIMERx channel output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0choctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0choctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0choctl`]
module"]
pub type ST0CHOCTL = crate::Reg<st0choctl::ST0CHOCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel output control register"]
pub mod st0choctl;
#[doc = "ST0FLTCTL (rw) register accessor: SHRTIMER Slave_TIMERx fault control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0fltctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0fltctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0fltctl`]
module"]
pub type ST0FLTCTL = crate::Reg<st0fltctl::ST0FLTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx fault control register"]
pub mod st0fltctl;
#[doc = "ST0ACTL (rw) register accessor: SHRTIMER Slave_TIMERx additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0actl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0actl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st0actl`]
module"]
pub type ST0ACTL = crate::Reg<st0actl::ST0ACTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx additional control register"]
pub mod st0actl;
