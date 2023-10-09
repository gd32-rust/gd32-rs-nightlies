#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHRTIMER Slave_TIMERx control register 0"]
    pub st2ctl0: ST2CTL0,
    #[doc = "0x04 - SHRTIMER Slave_TIMERx interrupt flag register"]
    pub st2intf: ST2INTF,
    #[doc = "0x08 - SHRTIMER Slave_TIMERx interrupt flag clear register"]
    pub st2intc: ST2INTC,
    #[doc = "0x0c - SHRTIMER Slave_TIMERx DMA and interrupt enable register"]
    pub st2dmainten: ST2DMAINTEN,
    #[doc = "0x10 - SHRTIMER Slave_TIMERx counter register"]
    pub st2cnt: ST2CNT,
    #[doc = "0x14 - SHRTIMER Slave_TIMER2 counter auto reload register"]
    pub st2car: ST2CAR,
    #[doc = "0x18 - SHRTIMER Slave_TIMER2 counter repetition register"]
    pub st2crep: ST2CREP,
    #[doc = "0x1c - SHRTIMER Slave_TIMER2 compare 0 value register"]
    pub st2cmp0v: ST2CMP0V,
    #[doc = "0x20 - SHRTIMER Slave_TIMERx compare 0 composite register"]
    pub st2cmp0cp: ST2CMP0CP,
    #[doc = "0x24 - SHRTIMER Slave_TIMERx compare 1 value register"]
    pub st2cmp1v: ST2CMP1V,
    #[doc = "0x28 - SHRTIMER Slave_TIMERx compare 2 value register"]
    pub st2cmp2v: ST2CMP2V,
    #[doc = "0x2c - SHRTIMER Slave_TIMERx compare 3 value register"]
    pub st2cmp3v: ST2CMP3V,
    #[doc = "0x30 - SHRTIMER Slave_TIMERx capture 0 value register"]
    pub st2cap0v: ST2CAP0V,
    #[doc = "0x34 - SHRTIMER Slave_TIMERx capture 1 value register"]
    pub st2cap1v: ST2CAP1V,
    #[doc = "0x38 - SHRTIMER Slave_TIMERx dead-time control register"]
    pub st2dtctl: ST2DTCTL,
    #[doc = "0x3c - SHRTIMER Slave_TIMERx channel 0 set request register"]
    pub st2ch0set: ST2CH0SET,
    #[doc = "0x40 - SHRTIMER Slave_TIMERx channel 0 reset request register"]
    pub st2ch0rst: ST2CH0RST,
    #[doc = "0x44 - SHRTIMER Slave_TIMERx channel 1 set request register"]
    pub st2ch1set: ST2CH1SET,
    #[doc = "0x48 - SHRTIMER Slave_TIMERx channel 1 reset request register"]
    pub st2ch1rst: ST2CH1RST,
    #[doc = "0x4c - SHRTIMER Slave_TIMERx external event filter configuration register 0"]
    pub st2exevfcfg0: ST2EXEVFCFG0,
    #[doc = "0x50 - SHRTIMER Slave_TIMERx external event filter configuration register 1"]
    pub st2exevfcfg1: ST2EXEVFCFG1,
    #[doc = "0x54 - SHRTIMER Slave_TIMERx counter reset register"]
    pub st2cntrst: ST2CNTRST,
    #[doc = "0x58 - SHRTIMER Slave_TIMERx carrier-signal control register"]
    pub st2csctl: ST2CSCTL,
    #[doc = "0x5c - SHRTIMER Slave_TIMERx capture 0 trigger register"]
    pub st2cap0trg: ST2CAP0TRG,
    #[doc = "0x60 - SHRTIMER Slave_TIMERx capture 1 trigger register"]
    pub st2cap1trg: ST2CAP1TRG,
    #[doc = "0x64 - SHRTIMER Slave_TIMERx channel output control register"]
    pub st2choctl: ST2CHOCTL,
    #[doc = "0x68 - SHRTIMER Slave_TIMERx fault control register"]
    pub st2fltctl: ST2FLTCTL,
    _reserved27: [u8; 0x10],
    #[doc = "0x7c - SHRTIMER Slave_TIMERx additional control register"]
    pub st2actl: ST2ACTL,
}
#[doc = "ST2CTL0 (rw) register accessor: SHRTIMER Slave_TIMERx control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2ctl0`]
module"]
pub type ST2CTL0 = crate::Reg<st2ctl0::ST2CTL0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx control register 0"]
pub mod st2ctl0;
#[doc = "ST2INTF (r) register accessor: SHRTIMER Slave_TIMERx interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2intf`]
module"]
pub type ST2INTF = crate::Reg<st2intf::ST2INTF_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx interrupt flag register"]
pub mod st2intf;
#[doc = "ST2INTC (w) register accessor: SHRTIMER Slave_TIMERx interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2intc`]
module"]
pub type ST2INTC = crate::Reg<st2intc::ST2INTC_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx interrupt flag clear register"]
pub mod st2intc;
#[doc = "ST2DMAINTEN (rw) register accessor: SHRTIMER Slave_TIMERx DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2dmainten`]
module"]
pub type ST2DMAINTEN = crate::Reg<st2dmainten::ST2DMAINTEN_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx DMA and interrupt enable register"]
pub mod st2dmainten;
#[doc = "ST2CNT (rw) register accessor: SHRTIMER Slave_TIMERx counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cnt`]
module"]
pub type ST2CNT = crate::Reg<st2cnt::ST2CNT_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx counter register"]
pub mod st2cnt;
#[doc = "ST2CAR (rw) register accessor: SHRTIMER Slave_TIMER2 counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2car`]
module"]
pub type ST2CAR = crate::Reg<st2car::ST2CAR_SPEC>;
#[doc = "SHRTIMER Slave_TIMER2 counter auto reload register"]
pub mod st2car;
#[doc = "ST2CREP (rw) register accessor: SHRTIMER Slave_TIMER2 counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2crep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2crep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2crep`]
module"]
pub type ST2CREP = crate::Reg<st2crep::ST2CREP_SPEC>;
#[doc = "SHRTIMER Slave_TIMER2 counter repetition register"]
pub mod st2crep;
#[doc = "ST2CMP0V (rw) register accessor: SHRTIMER Slave_TIMER2 compare 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cmp0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cmp0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cmp0v`]
module"]
pub type ST2CMP0V = crate::Reg<st2cmp0v::ST2CMP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMER2 compare 0 value register"]
pub mod st2cmp0v;
#[doc = "ST2CMP0CP (rw) register accessor: SHRTIMER Slave_TIMERx compare 0 composite register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cmp0cp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cmp0cp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cmp0cp`]
module"]
pub type ST2CMP0CP = crate::Reg<st2cmp0cp::ST2CMP0CP_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 0 composite register"]
pub mod st2cmp0cp;
#[doc = "ST2CMP1V (rw) register accessor: SHRTIMER Slave_TIMERx compare 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cmp1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cmp1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cmp1v`]
module"]
pub type ST2CMP1V = crate::Reg<st2cmp1v::ST2CMP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 1 value register"]
pub mod st2cmp1v;
#[doc = "ST2CMP2V (rw) register accessor: SHRTIMER Slave_TIMERx compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cmp2v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cmp2v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cmp2v`]
module"]
pub type ST2CMP2V = crate::Reg<st2cmp2v::ST2CMP2V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 2 value register"]
pub mod st2cmp2v;
#[doc = "ST2CMP3V (rw) register accessor: SHRTIMER Slave_TIMERx compare 3 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cmp3v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cmp3v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cmp3v`]
module"]
pub type ST2CMP3V = crate::Reg<st2cmp3v::ST2CMP3V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx compare 3 value register"]
pub mod st2cmp3v;
#[doc = "ST2CAP0V (rw) register accessor: SHRTIMER Slave_TIMERx capture 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cap0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cap0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cap0v`]
module"]
pub type ST2CAP0V = crate::Reg<st2cap0v::ST2CAP0V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 0 value register"]
pub mod st2cap0v;
#[doc = "ST2CAP1V (rw) register accessor: SHRTIMER Slave_TIMERx capture 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cap1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cap1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cap1v`]
module"]
pub type ST2CAP1V = crate::Reg<st2cap1v::ST2CAP1V_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 1 value register"]
pub mod st2cap1v;
#[doc = "ST2DTCTL (rw) register accessor: SHRTIMER Slave_TIMERx dead-time control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2dtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2dtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2dtctl`]
module"]
pub type ST2DTCTL = crate::Reg<st2dtctl::ST2DTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx dead-time control register"]
pub mod st2dtctl;
#[doc = "ST2CH0SET (rw) register accessor: SHRTIMER Slave_TIMERx channel 0 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2ch0set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2ch0set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2ch0set`]
module"]
pub type ST2CH0SET = crate::Reg<st2ch0set::ST2CH0SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 0 set request register"]
pub mod st2ch0set;
#[doc = "ST2CH0RST (rw) register accessor: SHRTIMER Slave_TIMERx channel 0 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2ch0rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2ch0rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2ch0rst`]
module"]
pub type ST2CH0RST = crate::Reg<st2ch0rst::ST2CH0RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 0 reset request register"]
pub mod st2ch0rst;
#[doc = "ST2CH1SET (rw) register accessor: SHRTIMER Slave_TIMERx channel 1 set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2ch1set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2ch1set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2ch1set`]
module"]
pub type ST2CH1SET = crate::Reg<st2ch1set::ST2CH1SET_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 1 set request register"]
pub mod st2ch1set;
#[doc = "ST2CH1RST (rw) register accessor: SHRTIMER Slave_TIMERx channel 1 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2ch1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2ch1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2ch1rst`]
module"]
pub type ST2CH1RST = crate::Reg<st2ch1rst::ST2CH1RST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel 1 reset request register"]
pub mod st2ch1rst;
#[doc = "ST2EXEVFCFG0 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2exevfcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2exevfcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2exevfcfg0`]
module"]
pub type ST2EXEVFCFG0 = crate::Reg<st2exevfcfg0::ST2EXEVFCFG0_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 0"]
pub mod st2exevfcfg0;
#[doc = "ST2EXEVFCFG1 (rw) register accessor: SHRTIMER Slave_TIMERx external event filter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2exevfcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2exevfcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2exevfcfg1`]
module"]
pub type ST2EXEVFCFG1 = crate::Reg<st2exevfcfg1::ST2EXEVFCFG1_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 1"]
pub mod st2exevfcfg1;
#[doc = "ST2CNTRST (rw) register accessor: SHRTIMER Slave_TIMERx counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cntrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cntrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cntrst`]
module"]
pub type ST2CNTRST = crate::Reg<st2cntrst::ST2CNTRST_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx counter reset register"]
pub mod st2cntrst;
#[doc = "ST2CSCTL (rw) register accessor: SHRTIMER Slave_TIMERx carrier-signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2csctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2csctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2csctl`]
module"]
pub type ST2CSCTL = crate::Reg<st2csctl::ST2CSCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx carrier-signal control register"]
pub mod st2csctl;
#[doc = "ST2CAP0TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 0 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cap0trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cap0trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cap0trg`]
module"]
pub type ST2CAP0TRG = crate::Reg<st2cap0trg::ST2CAP0TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 0 trigger register"]
pub mod st2cap0trg;
#[doc = "ST2CAP1TRG (rw) register accessor: SHRTIMER Slave_TIMERx capture 1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cap1trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cap1trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cap1trg`]
module"]
pub type ST2CAP1TRG = crate::Reg<st2cap1trg::ST2CAP1TRG_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx capture 1 trigger register"]
pub mod st2cap1trg;
#[doc = "ST2CHOCTL (rw) register accessor: SHRTIMER Slave_TIMERx channel output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2choctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2choctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2choctl`]
module"]
pub type ST2CHOCTL = crate::Reg<st2choctl::ST2CHOCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx channel output control register"]
pub mod st2choctl;
#[doc = "ST2FLTCTL (rw) register accessor: SHRTIMER Slave_TIMERx fault control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2fltctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2fltctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2fltctl`]
module"]
pub type ST2FLTCTL = crate::Reg<st2fltctl::ST2FLTCTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx fault control register"]
pub mod st2fltctl;
#[doc = "ST2ACTL (rw) register accessor: SHRTIMER Slave_TIMERx additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2actl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2actl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2actl`]
module"]
pub type ST2ACTL = crate::Reg<st2actl::ST2ACTL_SPEC>;
#[doc = "SHRTIMER Slave_TIMERx additional control register"]
pub mod st2actl;
