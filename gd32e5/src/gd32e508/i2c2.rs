#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - Slave address register 0"]
    pub saddr0: SADDR0,
    #[doc = "0x0c - Slave address register 1"]
    pub saddr1: SADDR1,
    #[doc = "0x10 - Timing register"]
    pub timing: TIMING,
    #[doc = "0x14 - timeout register"]
    pub timeout: TIMEOUT,
    #[doc = "0x18 - Transfer status register"]
    pub stat: STAT,
    #[doc = "0x1c - Status clear register"]
    pub statc: STATC,
    #[doc = "0x20 - Packet Error Checking"]
    pub pec: PEC,
    #[doc = "0x24 - receive data register"]
    pub rdata: RDATA,
    #[doc = "0x28 - Transmit data register"]
    pub tdata: TDATA,
}
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "SADDR0 (rw) register accessor: Slave address register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`saddr0`]
module"]
pub type SADDR0 = crate::Reg<saddr0::SADDR0_SPEC>;
#[doc = "Slave address register 0"]
pub mod saddr0;
#[doc = "SADDR1 (rw) register accessor: Slave address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`saddr1`]
module"]
pub type SADDR1 = crate::Reg<saddr1::SADDR1_SPEC>;
#[doc = "Slave address register 1"]
pub mod saddr1;
#[doc = "TIMING (rw) register accessor: Timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timing`]
module"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "Timing register"]
pub mod timing;
#[doc = "TIMEOUT (rw) register accessor: timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timeout`]
module"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "timeout register"]
pub mod timeout;
#[doc = "STAT (rw) register accessor: Transfer status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Transfer status register"]
pub mod stat;
#[doc = "STATC (w) register accessor: Status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statc`]
module"]
pub type STATC = crate::Reg<statc::STATC_SPEC>;
#[doc = "Status clear register"]
pub mod statc;
#[doc = "PEC (r) register accessor: Packet Error Checking\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pec`]
module"]
pub type PEC = crate::Reg<pec::PEC_SPEC>;
#[doc = "Packet Error Checking"]
pub mod pec;
#[doc = "RDATA (r) register accessor: receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rdata`]
module"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "receive data register"]
pub mod rdata;
#[doc = "TDATA (rw) register accessor: Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tdata`]
module"]
pub type TDATA = crate::Reg<tdata::TDATA_SPEC>;
#[doc = "Transmit data register"]
pub mod tdata;
