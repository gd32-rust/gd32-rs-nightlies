#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - Control register 2"]
    pub ctl2: CTL2,
    #[doc = "0x0c - Baud rate generator register"]
    pub baud: BAUD,
    #[doc = "0x10 - Prescaler and guard time configuration register"]
    pub gp: GP,
    #[doc = "0x14 - Receiver timeout register"]
    pub rt: RT,
    #[doc = "0x18 - Request register"]
    pub cmd: CMD,
    #[doc = "0x1c - Interrupt &amp; status register"]
    pub stat: STAT,
    #[doc = "0x20 - Interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x24 - Receive data register"]
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
#[doc = "CTL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl2`]
module"]
pub type CTL2 = crate::Reg<ctl2::CTL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "BAUD (rw) register accessor: Baud rate generator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`baud`]
module"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud rate generator register"]
pub mod baud;
#[doc = "GP (rw) register accessor: Prescaler and guard time configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gp`]
module"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "Prescaler and guard time configuration register"]
pub mod gp;
#[doc = "RT (rw) register accessor: Receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rt`]
module"]
pub type RT = crate::Reg<rt::RT_SPEC>;
#[doc = "Receiver timeout register"]
pub mod rt;
#[doc = "CMD (w) register accessor: Request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Request register"]
pub mod cmd;
#[doc = "STAT (r) register accessor: Interrupt &amp; status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Interrupt &amp; status register"]
pub mod stat;
#[doc = "INTC (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "RDATA (r) register accessor: Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rdata`]
module"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "Receive data register"]
pub mod rdata;
#[doc = "TDATA (rw) register accessor: Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tdata`]
module"]
pub type TDATA = crate::Reg<tdata::TDATA_SPEC>;
#[doc = "Transmit data register"]
pub mod tdata;
