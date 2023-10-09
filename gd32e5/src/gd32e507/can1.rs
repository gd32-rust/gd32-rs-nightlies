#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Transmit status register"]
    pub tstat: TSTAT,
    #[doc = "0x0c - Receive message FIFO0 register"]
    pub rfifo0: RFIFO0,
    #[doc = "0x10 - Receive message FIFO1 register"]
    pub rfifo1: RFIFO1,
    #[doc = "0x14 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x18 - Error register"]
    pub err: ERR,
    #[doc = "0x1c - Bit timing register"]
    pub bt: BT,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - Transmit mailbox identifier register 0"]
    pub tmi0: TMI0,
    #[doc = "0x184 - Transmit mailbox property register 0"]
    pub tmp0: TMP0,
    #[doc = "0x188 - Transmit mailbox data0 register"]
    pub tmdata00: TMDATA00,
    #[doc = "0x18c - Transmit mailbox data1 register"]
    pub tmdata10: TMDATA10,
    #[doc = "0x190 - Transmit mailbox identifier register 1"]
    pub tmi1: TMI1,
    #[doc = "0x194 - Transmit mailbox property register 1"]
    pub tmp1: TMP1,
    #[doc = "0x198 - Transmit mailbox data0 register"]
    pub tmdata01: TMDATA01,
    #[doc = "0x19c - Transmit mailbox data1 register"]
    pub tmdata11: TMDATA11,
    #[doc = "0x1a0 - Transmit mailbox identifier register 2"]
    pub tmi2: TMI2,
    #[doc = "0x1a4 - Transmit mailbox property register 2"]
    pub tmp2: TMP2,
    #[doc = "0x1a8 - Transmit mailbox data0 register"]
    pub tmdata02: TMDATA02,
    #[doc = "0x1ac - Transmit mailbox data1 register"]
    pub tmdata12: TMDATA12,
    #[doc = "0x1b0 - Receive FIFO mailbox identifier register"]
    pub rfifomi0: RFIFOMI0,
    #[doc = "0x1b4 - Receive FIFO0 mailbox property register"]
    pub rfifomp0: RFIFOMP0,
    #[doc = "0x1b8 - Receive FIFO0 mailbox data0 register"]
    pub rfifomdata00: RFIFOMDATA00,
    #[doc = "0x1bc - Receive FIFO0 mailbox data1 register"]
    pub rfifomdata10: RFIFOMDATA10,
    #[doc = "0x1c0 - Receive FIFO1 mailbox identifier register"]
    pub rfifomi1: RFIFOMI1,
    #[doc = "0x1c4 - Receive FIFO1 mailbox property register"]
    pub rfifomp1: RFIFOMP1,
    #[doc = "0x1c8 - Receive FIFO1 mailbox data0 register"]
    pub rfifomdata01: RFIFOMDATA01,
    #[doc = "0x1cc - Receive FIFO1 mailbox data1 register"]
    pub rfifomdata11: RFIFOMDATA11,
}
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "TSTAT (rw) register accessor: Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tstat`]
module"]
pub type TSTAT = crate::Reg<tstat::TSTAT_SPEC>;
#[doc = "Transmit status register"]
pub mod tstat;
#[doc = "RFIFO0 (rw) register accessor: Receive message FIFO0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifo0`]
module"]
pub type RFIFO0 = crate::Reg<rfifo0::RFIFO0_SPEC>;
#[doc = "Receive message FIFO0 register"]
pub mod rfifo0;
#[doc = "RFIFO1 (rw) register accessor: Receive message FIFO1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifo1`]
module"]
pub type RFIFO1 = crate::Reg<rfifo1::RFIFO1_SPEC>;
#[doc = "Receive message FIFO1 register"]
pub mod rfifo1;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "ERR (rw) register accessor: Error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`err`]
module"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error register"]
pub mod err;
#[doc = "BT (rw) register accessor: Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt`]
module"]
pub type BT = crate::Reg<bt::BT_SPEC>;
#[doc = "Bit timing register"]
pub mod bt;
#[doc = "TMI0 (rw) register accessor: Transmit mailbox identifier register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmi0`]
module"]
pub type TMI0 = crate::Reg<tmi0::TMI0_SPEC>;
#[doc = "Transmit mailbox identifier register 0"]
pub mod tmi0;
#[doc = "TMP0 (rw) register accessor: Transmit mailbox property register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmp0`]
module"]
pub type TMP0 = crate::Reg<tmp0::TMP0_SPEC>;
#[doc = "Transmit mailbox property register 0"]
pub mod tmp0;
#[doc = "TMDATA00 (rw) register accessor: Transmit mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdata00`]
module"]
pub type TMDATA00 = crate::Reg<tmdata00::TMDATA00_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata00;
#[doc = "TMDATA10 (rw) register accessor: Transmit mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdata10`]
module"]
pub type TMDATA10 = crate::Reg<tmdata10::TMDATA10_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata10;
#[doc = "TMI1 (rw) register accessor: Transmit mailbox identifier register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmi1`]
module"]
pub type TMI1 = crate::Reg<tmi1::TMI1_SPEC>;
#[doc = "Transmit mailbox identifier register 1"]
pub mod tmi1;
#[doc = "TMP1 (rw) register accessor: Transmit mailbox property register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmp1`]
module"]
pub type TMP1 = crate::Reg<tmp1::TMP1_SPEC>;
#[doc = "Transmit mailbox property register 1"]
pub mod tmp1;
#[doc = "TMDATA01 (rw) register accessor: Transmit mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdata01`]
module"]
pub type TMDATA01 = crate::Reg<tmdata01::TMDATA01_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata01;
#[doc = "TMDATA11 (rw) register accessor: Transmit mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdata11`]
module"]
pub type TMDATA11 = crate::Reg<tmdata11::TMDATA11_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata11;
#[doc = "TMI2 (rw) register accessor: Transmit mailbox identifier register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmi2`]
module"]
pub type TMI2 = crate::Reg<tmi2::TMI2_SPEC>;
#[doc = "Transmit mailbox identifier register 2"]
pub mod tmi2;
#[doc = "TMP2 (rw) register accessor: Transmit mailbox property register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmp2`]
module"]
pub type TMP2 = crate::Reg<tmp2::TMP2_SPEC>;
#[doc = "Transmit mailbox property register 2"]
pub mod tmp2;
#[doc = "TMDATA02 (rw) register accessor: Transmit mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdata02`]
module"]
pub type TMDATA02 = crate::Reg<tmdata02::TMDATA02_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata02;
#[doc = "TMDATA12 (rw) register accessor: Transmit mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdata12`]
module"]
pub type TMDATA12 = crate::Reg<tmdata12::TMDATA12_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata12;
#[doc = "RFIFOMI0 (r) register accessor: Receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomi0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomi0`]
module"]
pub type RFIFOMI0 = crate::Reg<rfifomi0::RFIFOMI0_SPEC>;
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfifomi0;
#[doc = "RFIFOMP0 (r) register accessor: Receive FIFO0 mailbox property register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomp0`]
module"]
pub type RFIFOMP0 = crate::Reg<rfifomp0::RFIFOMP0_SPEC>;
#[doc = "Receive FIFO0 mailbox property register"]
pub mod rfifomp0;
#[doc = "RFIFOMDATA00 (r) register accessor: Receive FIFO0 mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata00::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomdata00`]
module"]
pub type RFIFOMDATA00 = crate::Reg<rfifomdata00::RFIFOMDATA00_SPEC>;
#[doc = "Receive FIFO0 mailbox data0 register"]
pub mod rfifomdata00;
#[doc = "RFIFOMDATA10 (r) register accessor: Receive FIFO0 mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomdata10`]
module"]
pub type RFIFOMDATA10 = crate::Reg<rfifomdata10::RFIFOMDATA10_SPEC>;
#[doc = "Receive FIFO0 mailbox data1 register"]
pub mod rfifomdata10;
#[doc = "RFIFOMI1 (r) register accessor: Receive FIFO1 mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomi1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomi1`]
module"]
pub type RFIFOMI1 = crate::Reg<rfifomi1::RFIFOMI1_SPEC>;
#[doc = "Receive FIFO1 mailbox identifier register"]
pub mod rfifomi1;
#[doc = "RFIFOMP1 (r) register accessor: Receive FIFO1 mailbox property register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomp1`]
module"]
pub type RFIFOMP1 = crate::Reg<rfifomp1::RFIFOMP1_SPEC>;
#[doc = "Receive FIFO1 mailbox property register"]
pub mod rfifomp1;
#[doc = "RFIFOMDATA01 (r) register accessor: Receive FIFO1 mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata01::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomdata01`]
module"]
pub type RFIFOMDATA01 = crate::Reg<rfifomdata01::RFIFOMDATA01_SPEC>;
#[doc = "Receive FIFO1 mailbox data0 register"]
pub mod rfifomdata01;
#[doc = "RFIFOMDATA11 (r) register accessor: Receive FIFO1 mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfifomdata11`]
module"]
pub type RFIFOMDATA11 = crate::Reg<rfifomdata11::RFIFOMDATA11_SPEC>;
#[doc = "Receive FIFO1 mailbox data1 register"]
pub mod rfifomdata11;
