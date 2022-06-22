#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Transmit status register"]
    pub tstat: crate::Reg<tstat::TSTAT_SPEC>,
    #[doc = "0x0c - Receive message FIFO0 register"]
    pub rfifo0: crate::Reg<rfifo0::RFIFO0_SPEC>,
    #[doc = "0x10 - Receive message FIFO1 register"]
    pub rfifo1: crate::Reg<rfifo1::RFIFO1_SPEC>,
    #[doc = "0x14 - Interrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x18 - Error register"]
    pub err: crate::Reg<err::ERR_SPEC>,
    #[doc = "0x1c - Bit timing register"]
    pub bt: crate::Reg<bt::BT_SPEC>,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - Transmit mailbox identifier register 0"]
    pub tmi0: crate::Reg<tmi0::TMI0_SPEC>,
    #[doc = "0x184 - Transmit mailbox property register 0"]
    pub tmp0: crate::Reg<tmp0::TMP0_SPEC>,
    #[doc = "0x188 - Transmit mailbox data0 register"]
    pub tmdata00: crate::Reg<tmdata00::TMDATA00_SPEC>,
    #[doc = "0x18c - Transmit mailbox data1 register"]
    pub tmdata10: crate::Reg<tmdata10::TMDATA10_SPEC>,
    #[doc = "0x190 - Transmit mailbox identifier register"]
    pub tmi1: crate::Reg<tmi1::TMI1_SPEC>,
    #[doc = "0x194 - Transmit mailbox property register"]
    pub tmp1: crate::Reg<tmp1::TMP1_SPEC>,
    #[doc = "0x198 - Transmit mailbox data0 register"]
    pub tmdata01: crate::Reg<tmdata01::TMDATA01_SPEC>,
    #[doc = "0x19c - Transmit mailbox data1 register"]
    pub tmdata11: crate::Reg<tmdata11::TMDATA11_SPEC>,
    #[doc = "0x1a0 - Transmit mailbox identifier register"]
    pub tmi2: crate::Reg<tmi2::TMI2_SPEC>,
    #[doc = "0x1a4 - Transmit mailbox property register"]
    pub tmp2: crate::Reg<tmp2::TMP2_SPEC>,
    #[doc = "0x1a8 - Transmit mailbox data0 register"]
    pub tmdata02: crate::Reg<tmdata02::TMDATA02_SPEC>,
    #[doc = "0x1ac - Transmit mailbox data1 register"]
    pub tmdata12: crate::Reg<tmdata12::TMDATA12_SPEC>,
    #[doc = "0x1b0 - Receive FIFO mailbox identifier register"]
    pub rfifomi0: crate::Reg<rfifomi0::RFIFOMI0_SPEC>,
    #[doc = "0x1b4 - Receive FIFO mailbox property register"]
    pub rfifomp0: crate::Reg<rfifomp0::RFIFOMP0_SPEC>,
    #[doc = "0x1b8 - Receive FIFO mailbox data0 register"]
    pub rfifomdata00: crate::Reg<rfifomdata00::RFIFOMDATA00_SPEC>,
    #[doc = "0x1bc - Receive FIFO mailbox data1 register"]
    pub rfifomdata10: crate::Reg<rfifomdata10::RFIFOMDATA10_SPEC>,
    #[doc = "0x1c0 - Receive FIFO mailbox identifier register"]
    pub rfifomi1: crate::Reg<rfifomi1::RFIFOMI1_SPEC>,
    #[doc = "0x1c4 - Receive FIFO mailbox property register"]
    pub rfifomp1: crate::Reg<rfifomp1::RFIFOMP1_SPEC>,
    #[doc = "0x1c8 - Receive FIFO mailbox data0 register"]
    pub rfifomdata01: crate::Reg<rfifomdata01::RFIFOMDATA01_SPEC>,
    #[doc = "0x1cc - Receive FIFO mailbox data1 register"]
    pub rfifomdata11: crate::Reg<rfifomdata11::RFIFOMDATA11_SPEC>,
    _reserved28: [u8; 0x30],
    #[doc = "0x200 - Filter control register"]
    pub fctl: crate::Reg<fctl::FCTL_SPEC>,
    #[doc = "0x204 - Filter mode configuration register"]
    pub fmcfg: crate::Reg<fmcfg::FMCFG_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x20c - Filter scale configuration register"]
    pub fscfg: crate::Reg<fscfg::FSCFG_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x214 - Filter associated FIFO register"]
    pub fafifo: crate::Reg<fafifo::FAFIFO_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x21c - Filter working register"]
    pub fw: crate::Reg<fw::FW_SPEC>,
    _reserved33: [u8; 0x20],
    #[doc = "0x240 - Filter 0 data 0 register"]
    pub f0data0: crate::Reg<f0data0::F0DATA0_SPEC>,
    #[doc = "0x244 - Filter 0 data 1 register"]
    pub f0data1: crate::Reg<f0data1::F0DATA1_SPEC>,
    #[doc = "0x248 - Filter 1 data 0 register"]
    pub f1data0: crate::Reg<f1data0::F1DATA0_SPEC>,
    #[doc = "0x24c - Filter 1 data 1 register"]
    pub f1data1: crate::Reg<f1data1::F1DATA1_SPEC>,
    #[doc = "0x250 - Filter 2 data 0 register"]
    pub f2data0: crate::Reg<f2data0::F2DATA0_SPEC>,
    #[doc = "0x254 - Filter 2 data 1 register"]
    pub f2data1: crate::Reg<f2data1::F2DATA1_SPEC>,
    #[doc = "0x258 - Filter 3 data 0 register"]
    pub f3data0: crate::Reg<f3data0::F3DATA0_SPEC>,
    #[doc = "0x25c - Filter 3 data 1 register"]
    pub f3data1: crate::Reg<f3data1::F3DATA1_SPEC>,
    #[doc = "0x260 - Filter 4 data 0 register"]
    pub f4data0: crate::Reg<f4data0::F4DATA0_SPEC>,
    #[doc = "0x264 - Filter 4 data 1 register"]
    pub f4data1: crate::Reg<f4data1::F4DATA1_SPEC>,
    #[doc = "0x268 - Filter 5 data 0 register"]
    pub f5data0: crate::Reg<f5data0::F5DATA0_SPEC>,
    #[doc = "0x26c - Filter 5 data 1 register"]
    pub f5data1: crate::Reg<f5data1::F5DATA1_SPEC>,
    #[doc = "0x270 - Filter 6 data 0 register"]
    pub f6data0: crate::Reg<f6data0::F6DATA0_SPEC>,
    #[doc = "0x274 - Filter 6 data 1 register"]
    pub f6data1: crate::Reg<f6data1::F6DATA1_SPEC>,
    #[doc = "0x278 - Filter 7 data 0 register"]
    pub f7data0: crate::Reg<f7data0::F7DATA0_SPEC>,
    #[doc = "0x27c - Filter 7 data 1 register"]
    pub f7data1: crate::Reg<f7data1::F7DATA1_SPEC>,
    #[doc = "0x280 - Filter 8 data 0 register"]
    pub f8data0: crate::Reg<f8data0::F8DATA0_SPEC>,
    #[doc = "0x284 - Filter 8 data 1 register"]
    pub f8data1: crate::Reg<f8data1::F8DATA1_SPEC>,
    #[doc = "0x288 - Filter 9 data 0 register"]
    pub f9data0: crate::Reg<f9data0::F9DATA0_SPEC>,
    #[doc = "0x28c - Filter 9 data 1 register"]
    pub f9data1: crate::Reg<f9data1::F9DATA1_SPEC>,
    #[doc = "0x290 - Filter 10 data 0 register"]
    pub f10data0: crate::Reg<f10data0::F10DATA0_SPEC>,
    #[doc = "0x294 - Filter 10 data 1 register"]
    pub f10data1: crate::Reg<f10data1::F10DATA1_SPEC>,
    #[doc = "0x298 - Filter 11 data 0 register"]
    pub f11data0: crate::Reg<f11data0::F11DATA0_SPEC>,
    #[doc = "0x29c - Filter 11 data 1 register"]
    pub f11data1: crate::Reg<f11data1::F11DATA1_SPEC>,
    #[doc = "0x2a0 - Filter 12 data 0 register"]
    pub f12data0: crate::Reg<f12data0::F12DATA0_SPEC>,
    #[doc = "0x2a4 - Filter 12 data 1 register"]
    pub f12data1: crate::Reg<f12data1::F12DATA1_SPEC>,
    #[doc = "0x2a8 - Filter 13 data 0 register"]
    pub f13data0: crate::Reg<f13data0::F13DATA0_SPEC>,
    #[doc = "0x2ac - Filter 13 data 1 register"]
    pub f13data1: crate::Reg<f13data1::F13DATA1_SPEC>,
    #[doc = "0x2b0 - Filter 14 data 0 register"]
    pub f14data0: crate::Reg<f14data0::F14DATA0_SPEC>,
    #[doc = "0x2b4 - Filter 14 data 1 register"]
    pub f14data1: crate::Reg<f14data1::F14DATA1_SPEC>,
    #[doc = "0x2b8 - Filter 15 data 0 register"]
    pub f15data0: crate::Reg<f15data0::F15DATA0_SPEC>,
    #[doc = "0x2bc - Filter 15 data 1 register"]
    pub f15data1: crate::Reg<f15data1::F15DATA1_SPEC>,
    #[doc = "0x2c0 - Filter 16 data 0 register"]
    pub f16data0: crate::Reg<f16data0::F16DATA0_SPEC>,
    #[doc = "0x2c4 - Filter 16 data 1 register"]
    pub f16data1: crate::Reg<f16data1::F16DATA1_SPEC>,
    #[doc = "0x2c8 - Filter 17 data 0 register"]
    pub f17data0: crate::Reg<f17data0::F17DATA0_SPEC>,
    #[doc = "0x2cc - Filter 17 data 1 register"]
    pub f17data1: crate::Reg<f17data1::F17DATA1_SPEC>,
    #[doc = "0x2d0 - Filter 18 data 0 register"]
    pub f18data0: crate::Reg<f18data0::F18DATA0_SPEC>,
    #[doc = "0x2d4 - Filter 18 data 1 register"]
    pub f18data1: crate::Reg<f18data1::F18DATA1_SPEC>,
    #[doc = "0x2d8 - Filter 19 data 0 register"]
    pub f19data0: crate::Reg<f19data0::F19DATA0_SPEC>,
    #[doc = "0x2dc - Filter 19 data 1 register"]
    pub f19data1: crate::Reg<f19data1::F19DATA1_SPEC>,
    #[doc = "0x2e0 - Filter 20 data 0 register"]
    pub f20data0: crate::Reg<f20data0::F20DATA0_SPEC>,
    #[doc = "0x2e4 - Filter 20 data 1 register"]
    pub f20data1: crate::Reg<f20data1::F20DATA1_SPEC>,
    #[doc = "0x2e8 - Filter 21 data 0 register"]
    pub f21data0: crate::Reg<f21data0::F21DATA0_SPEC>,
    #[doc = "0x2ec - Filter 21 data 1 register"]
    pub f21data1: crate::Reg<f21data1::F21DATA1_SPEC>,
    #[doc = "0x2f0 - Filter 22 data 0 register"]
    pub f22data0: crate::Reg<f22data0::F22DATA0_SPEC>,
    #[doc = "0x2f4 - Filter 22 data 1 register"]
    pub f22data1: crate::Reg<f22data1::F22DATA1_SPEC>,
    #[doc = "0x2f8 - Filter 23 data 0 register"]
    pub f23data0: crate::Reg<f23data0::F23DATA0_SPEC>,
    #[doc = "0x2fc - Filter 23 data 1 register"]
    pub f23data1: crate::Reg<f23data1::F23DATA1_SPEC>,
    #[doc = "0x300 - Filter 24 data 0 register"]
    pub f24data0: crate::Reg<f24data0::F24DATA0_SPEC>,
    #[doc = "0x304 - Filter 24 data 1 register"]
    pub f24data1: crate::Reg<f24data1::F24DATA1_SPEC>,
    #[doc = "0x308 - Filter 25 data 0 register"]
    pub f25data0: crate::Reg<f25data0::F25DATA0_SPEC>,
    #[doc = "0x30c - Filter 25 data 1 register"]
    pub f25data1: crate::Reg<f25data1::F25DATA1_SPEC>,
    #[doc = "0x310 - Filter 26 data 0 register"]
    pub f26data0: crate::Reg<f26data0::F26DATA0_SPEC>,
    #[doc = "0x314 - Filter 26 data 1 register"]
    pub f26data1: crate::Reg<f26data1::F26DATA1_SPEC>,
    #[doc = "0x318 - Filter 27 data 0 register"]
    pub f27data0: crate::Reg<f27data0::F27DATA0_SPEC>,
    #[doc = "0x31c - Filter 27 data 1 register"]
    pub f27data1: crate::Reg<f27data1::F27DATA1_SPEC>,
    _reserved89: [u8; 0xdc],
    #[doc = "0x3fc - PHY control register"]
    pub phyctl: crate::Reg<phyctl::PHYCTL_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "TSTAT register accessor: an alias for `Reg<TSTAT_SPEC>`"]
pub type TSTAT = crate::Reg<tstat::TSTAT_SPEC>;
#[doc = "Transmit status register"]
pub mod tstat;
#[doc = "RFIFO0 register accessor: an alias for `Reg<RFIFO0_SPEC>`"]
pub type RFIFO0 = crate::Reg<rfifo0::RFIFO0_SPEC>;
#[doc = "Receive message FIFO0 register"]
pub mod rfifo0;
#[doc = "RFIFO1 register accessor: an alias for `Reg<RFIFO1_SPEC>`"]
pub type RFIFO1 = crate::Reg<rfifo1::RFIFO1_SPEC>;
#[doc = "Receive message FIFO1 register"]
pub mod rfifo1;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "ERR register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error register"]
pub mod err;
#[doc = "BT register accessor: an alias for `Reg<BT_SPEC>`"]
pub type BT = crate::Reg<bt::BT_SPEC>;
#[doc = "Bit timing register"]
pub mod bt;
#[doc = "TMI0 register accessor: an alias for `Reg<TMI0_SPEC>`"]
pub type TMI0 = crate::Reg<tmi0::TMI0_SPEC>;
#[doc = "Transmit mailbox identifier register 0"]
pub mod tmi0;
#[doc = "TMP0 register accessor: an alias for `Reg<TMP0_SPEC>`"]
pub type TMP0 = crate::Reg<tmp0::TMP0_SPEC>;
#[doc = "Transmit mailbox property register 0"]
pub mod tmp0;
#[doc = "TMDATA00 register accessor: an alias for `Reg<TMDATA00_SPEC>`"]
pub type TMDATA00 = crate::Reg<tmdata00::TMDATA00_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata00;
#[doc = "TMDATA10 register accessor: an alias for `Reg<TMDATA10_SPEC>`"]
pub type TMDATA10 = crate::Reg<tmdata10::TMDATA10_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata10;
#[doc = "TMI1 register accessor: an alias for `Reg<TMI1_SPEC>`"]
pub type TMI1 = crate::Reg<tmi1::TMI1_SPEC>;
#[doc = "Transmit mailbox identifier register"]
pub mod tmi1;
#[doc = "TMP1 register accessor: an alias for `Reg<TMP1_SPEC>`"]
pub type TMP1 = crate::Reg<tmp1::TMP1_SPEC>;
#[doc = "Transmit mailbox property register"]
pub mod tmp1;
#[doc = "TMDATA01 register accessor: an alias for `Reg<TMDATA01_SPEC>`"]
pub type TMDATA01 = crate::Reg<tmdata01::TMDATA01_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata01;
#[doc = "TMDATA11 register accessor: an alias for `Reg<TMDATA11_SPEC>`"]
pub type TMDATA11 = crate::Reg<tmdata11::TMDATA11_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata11;
#[doc = "TMI2 register accessor: an alias for `Reg<TMI2_SPEC>`"]
pub type TMI2 = crate::Reg<tmi2::TMI2_SPEC>;
#[doc = "Transmit mailbox identifier register"]
pub mod tmi2;
#[doc = "TMP2 register accessor: an alias for `Reg<TMP2_SPEC>`"]
pub type TMP2 = crate::Reg<tmp2::TMP2_SPEC>;
#[doc = "Transmit mailbox property register"]
pub mod tmp2;
#[doc = "TMDATA02 register accessor: an alias for `Reg<TMDATA02_SPEC>`"]
pub type TMDATA02 = crate::Reg<tmdata02::TMDATA02_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata02;
#[doc = "TMDATA12 register accessor: an alias for `Reg<TMDATA12_SPEC>`"]
pub type TMDATA12 = crate::Reg<tmdata12::TMDATA12_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata12;
#[doc = "RFIFOMI0 register accessor: an alias for `Reg<RFIFOMI0_SPEC>`"]
pub type RFIFOMI0 = crate::Reg<rfifomi0::RFIFOMI0_SPEC>;
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfifomi0;
#[doc = "RFIFOMP0 register accessor: an alias for `Reg<RFIFOMP0_SPEC>`"]
pub type RFIFOMP0 = crate::Reg<rfifomp0::RFIFOMP0_SPEC>;
#[doc = "Receive FIFO mailbox property register"]
pub mod rfifomp0;
#[doc = "RFIFOMDATA00 register accessor: an alias for `Reg<RFIFOMDATA00_SPEC>`"]
pub type RFIFOMDATA00 = crate::Reg<rfifomdata00::RFIFOMDATA00_SPEC>;
#[doc = "Receive FIFO mailbox data0 register"]
pub mod rfifomdata00;
#[doc = "RFIFOMDATA10 register accessor: an alias for `Reg<RFIFOMDATA10_SPEC>`"]
pub type RFIFOMDATA10 = crate::Reg<rfifomdata10::RFIFOMDATA10_SPEC>;
#[doc = "Receive FIFO mailbox data1 register"]
pub mod rfifomdata10;
#[doc = "RFIFOMI1 register accessor: an alias for `Reg<RFIFOMI1_SPEC>`"]
pub type RFIFOMI1 = crate::Reg<rfifomi1::RFIFOMI1_SPEC>;
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfifomi1;
#[doc = "RFIFOMP1 register accessor: an alias for `Reg<RFIFOMP1_SPEC>`"]
pub type RFIFOMP1 = crate::Reg<rfifomp1::RFIFOMP1_SPEC>;
#[doc = "Receive FIFO mailbox property register"]
pub mod rfifomp1;
#[doc = "RFIFOMDATA01 register accessor: an alias for `Reg<RFIFOMDATA01_SPEC>`"]
pub type RFIFOMDATA01 = crate::Reg<rfifomdata01::RFIFOMDATA01_SPEC>;
#[doc = "Receive FIFO mailbox data0 register"]
pub mod rfifomdata01;
#[doc = "RFIFOMDATA11 register accessor: an alias for `Reg<RFIFOMDATA11_SPEC>`"]
pub type RFIFOMDATA11 = crate::Reg<rfifomdata11::RFIFOMDATA11_SPEC>;
#[doc = "Receive FIFO mailbox data1 register"]
pub mod rfifomdata11;
#[doc = "FCTL register accessor: an alias for `Reg<FCTL_SPEC>`"]
pub type FCTL = crate::Reg<fctl::FCTL_SPEC>;
#[doc = "Filter control register"]
pub mod fctl;
#[doc = "FMCFG register accessor: an alias for `Reg<FMCFG_SPEC>`"]
pub type FMCFG = crate::Reg<fmcfg::FMCFG_SPEC>;
#[doc = "Filter mode configuration register"]
pub mod fmcfg;
#[doc = "FSCFG register accessor: an alias for `Reg<FSCFG_SPEC>`"]
pub type FSCFG = crate::Reg<fscfg::FSCFG_SPEC>;
#[doc = "Filter scale configuration register"]
pub mod fscfg;
#[doc = "FAFIFO register accessor: an alias for `Reg<FAFIFO_SPEC>`"]
pub type FAFIFO = crate::Reg<fafifo::FAFIFO_SPEC>;
#[doc = "Filter associated FIFO register"]
pub mod fafifo;
#[doc = "FW register accessor: an alias for `Reg<FW_SPEC>`"]
pub type FW = crate::Reg<fw::FW_SPEC>;
#[doc = "Filter working register"]
pub mod fw;
#[doc = "F0DATA0 register accessor: an alias for `Reg<F0DATA0_SPEC>`"]
pub type F0DATA0 = crate::Reg<f0data0::F0DATA0_SPEC>;
#[doc = "Filter 0 data 0 register"]
pub mod f0data0;
#[doc = "F0DATA1 register accessor: an alias for `Reg<F0DATA1_SPEC>`"]
pub type F0DATA1 = crate::Reg<f0data1::F0DATA1_SPEC>;
#[doc = "Filter 0 data 1 register"]
pub mod f0data1;
#[doc = "F1DATA0 register accessor: an alias for `Reg<F1DATA0_SPEC>`"]
pub type F1DATA0 = crate::Reg<f1data0::F1DATA0_SPEC>;
#[doc = "Filter 1 data 0 register"]
pub mod f1data0;
#[doc = "F1DATA1 register accessor: an alias for `Reg<F1DATA1_SPEC>`"]
pub type F1DATA1 = crate::Reg<f1data1::F1DATA1_SPEC>;
#[doc = "Filter 1 data 1 register"]
pub mod f1data1;
#[doc = "F2DATA0 register accessor: an alias for `Reg<F2DATA0_SPEC>`"]
pub type F2DATA0 = crate::Reg<f2data0::F2DATA0_SPEC>;
#[doc = "Filter 2 data 0 register"]
pub mod f2data0;
#[doc = "F2DATA1 register accessor: an alias for `Reg<F2DATA1_SPEC>`"]
pub type F2DATA1 = crate::Reg<f2data1::F2DATA1_SPEC>;
#[doc = "Filter 2 data 1 register"]
pub mod f2data1;
#[doc = "F3DATA0 register accessor: an alias for `Reg<F3DATA0_SPEC>`"]
pub type F3DATA0 = crate::Reg<f3data0::F3DATA0_SPEC>;
#[doc = "Filter 3 data 0 register"]
pub mod f3data0;
#[doc = "F3DATA1 register accessor: an alias for `Reg<F3DATA1_SPEC>`"]
pub type F3DATA1 = crate::Reg<f3data1::F3DATA1_SPEC>;
#[doc = "Filter 3 data 1 register"]
pub mod f3data1;
#[doc = "F4DATA0 register accessor: an alias for `Reg<F4DATA0_SPEC>`"]
pub type F4DATA0 = crate::Reg<f4data0::F4DATA0_SPEC>;
#[doc = "Filter 4 data 0 register"]
pub mod f4data0;
#[doc = "F4DATA1 register accessor: an alias for `Reg<F4DATA1_SPEC>`"]
pub type F4DATA1 = crate::Reg<f4data1::F4DATA1_SPEC>;
#[doc = "Filter 4 data 1 register"]
pub mod f4data1;
#[doc = "F5DATA0 register accessor: an alias for `Reg<F5DATA0_SPEC>`"]
pub type F5DATA0 = crate::Reg<f5data0::F5DATA0_SPEC>;
#[doc = "Filter 5 data 0 register"]
pub mod f5data0;
#[doc = "F5DATA1 register accessor: an alias for `Reg<F5DATA1_SPEC>`"]
pub type F5DATA1 = crate::Reg<f5data1::F5DATA1_SPEC>;
#[doc = "Filter 5 data 1 register"]
pub mod f5data1;
#[doc = "F6DATA0 register accessor: an alias for `Reg<F6DATA0_SPEC>`"]
pub type F6DATA0 = crate::Reg<f6data0::F6DATA0_SPEC>;
#[doc = "Filter 6 data 0 register"]
pub mod f6data0;
#[doc = "F6DATA1 register accessor: an alias for `Reg<F6DATA1_SPEC>`"]
pub type F6DATA1 = crate::Reg<f6data1::F6DATA1_SPEC>;
#[doc = "Filter 6 data 1 register"]
pub mod f6data1;
#[doc = "F7DATA0 register accessor: an alias for `Reg<F7DATA0_SPEC>`"]
pub type F7DATA0 = crate::Reg<f7data0::F7DATA0_SPEC>;
#[doc = "Filter 7 data 0 register"]
pub mod f7data0;
#[doc = "F7DATA1 register accessor: an alias for `Reg<F7DATA1_SPEC>`"]
pub type F7DATA1 = crate::Reg<f7data1::F7DATA1_SPEC>;
#[doc = "Filter 7 data 1 register"]
pub mod f7data1;
#[doc = "F8DATA0 register accessor: an alias for `Reg<F8DATA0_SPEC>`"]
pub type F8DATA0 = crate::Reg<f8data0::F8DATA0_SPEC>;
#[doc = "Filter 8 data 0 register"]
pub mod f8data0;
#[doc = "F8DATA1 register accessor: an alias for `Reg<F8DATA1_SPEC>`"]
pub type F8DATA1 = crate::Reg<f8data1::F8DATA1_SPEC>;
#[doc = "Filter 8 data 1 register"]
pub mod f8data1;
#[doc = "F9DATA0 register accessor: an alias for `Reg<F9DATA0_SPEC>`"]
pub type F9DATA0 = crate::Reg<f9data0::F9DATA0_SPEC>;
#[doc = "Filter 9 data 0 register"]
pub mod f9data0;
#[doc = "F9DATA1 register accessor: an alias for `Reg<F9DATA1_SPEC>`"]
pub type F9DATA1 = crate::Reg<f9data1::F9DATA1_SPEC>;
#[doc = "Filter 9 data 1 register"]
pub mod f9data1;
#[doc = "F10DATA0 register accessor: an alias for `Reg<F10DATA0_SPEC>`"]
pub type F10DATA0 = crate::Reg<f10data0::F10DATA0_SPEC>;
#[doc = "Filter 10 data 0 register"]
pub mod f10data0;
#[doc = "F10DATA1 register accessor: an alias for `Reg<F10DATA1_SPEC>`"]
pub type F10DATA1 = crate::Reg<f10data1::F10DATA1_SPEC>;
#[doc = "Filter 10 data 1 register"]
pub mod f10data1;
#[doc = "F11DATA0 register accessor: an alias for `Reg<F11DATA0_SPEC>`"]
pub type F11DATA0 = crate::Reg<f11data0::F11DATA0_SPEC>;
#[doc = "Filter 11 data 0 register"]
pub mod f11data0;
#[doc = "F11DATA1 register accessor: an alias for `Reg<F11DATA1_SPEC>`"]
pub type F11DATA1 = crate::Reg<f11data1::F11DATA1_SPEC>;
#[doc = "Filter 11 data 1 register"]
pub mod f11data1;
#[doc = "F12DATA0 register accessor: an alias for `Reg<F12DATA0_SPEC>`"]
pub type F12DATA0 = crate::Reg<f12data0::F12DATA0_SPEC>;
#[doc = "Filter 12 data 0 register"]
pub mod f12data0;
#[doc = "F12DATA1 register accessor: an alias for `Reg<F12DATA1_SPEC>`"]
pub type F12DATA1 = crate::Reg<f12data1::F12DATA1_SPEC>;
#[doc = "Filter 12 data 1 register"]
pub mod f12data1;
#[doc = "F13DATA0 register accessor: an alias for `Reg<F13DATA0_SPEC>`"]
pub type F13DATA0 = crate::Reg<f13data0::F13DATA0_SPEC>;
#[doc = "Filter 13 data 0 register"]
pub mod f13data0;
#[doc = "F13DATA1 register accessor: an alias for `Reg<F13DATA1_SPEC>`"]
pub type F13DATA1 = crate::Reg<f13data1::F13DATA1_SPEC>;
#[doc = "Filter 13 data 1 register"]
pub mod f13data1;
#[doc = "F14DATA0 register accessor: an alias for `Reg<F14DATA0_SPEC>`"]
pub type F14DATA0 = crate::Reg<f14data0::F14DATA0_SPEC>;
#[doc = "Filter 14 data 0 register"]
pub mod f14data0;
#[doc = "F14DATA1 register accessor: an alias for `Reg<F14DATA1_SPEC>`"]
pub type F14DATA1 = crate::Reg<f14data1::F14DATA1_SPEC>;
#[doc = "Filter 14 data 1 register"]
pub mod f14data1;
#[doc = "F15DATA0 register accessor: an alias for `Reg<F15DATA0_SPEC>`"]
pub type F15DATA0 = crate::Reg<f15data0::F15DATA0_SPEC>;
#[doc = "Filter 15 data 0 register"]
pub mod f15data0;
#[doc = "F15DATA1 register accessor: an alias for `Reg<F15DATA1_SPEC>`"]
pub type F15DATA1 = crate::Reg<f15data1::F15DATA1_SPEC>;
#[doc = "Filter 15 data 1 register"]
pub mod f15data1;
#[doc = "F16DATA0 register accessor: an alias for `Reg<F16DATA0_SPEC>`"]
pub type F16DATA0 = crate::Reg<f16data0::F16DATA0_SPEC>;
#[doc = "Filter 16 data 0 register"]
pub mod f16data0;
#[doc = "F16DATA1 register accessor: an alias for `Reg<F16DATA1_SPEC>`"]
pub type F16DATA1 = crate::Reg<f16data1::F16DATA1_SPEC>;
#[doc = "Filter 16 data 1 register"]
pub mod f16data1;
#[doc = "F17DATA0 register accessor: an alias for `Reg<F17DATA0_SPEC>`"]
pub type F17DATA0 = crate::Reg<f17data0::F17DATA0_SPEC>;
#[doc = "Filter 17 data 0 register"]
pub mod f17data0;
#[doc = "F17DATA1 register accessor: an alias for `Reg<F17DATA1_SPEC>`"]
pub type F17DATA1 = crate::Reg<f17data1::F17DATA1_SPEC>;
#[doc = "Filter 17 data 1 register"]
pub mod f17data1;
#[doc = "F18DATA0 register accessor: an alias for `Reg<F18DATA0_SPEC>`"]
pub type F18DATA0 = crate::Reg<f18data0::F18DATA0_SPEC>;
#[doc = "Filter 18 data 0 register"]
pub mod f18data0;
#[doc = "F18DATA1 register accessor: an alias for `Reg<F18DATA1_SPEC>`"]
pub type F18DATA1 = crate::Reg<f18data1::F18DATA1_SPEC>;
#[doc = "Filter 18 data 1 register"]
pub mod f18data1;
#[doc = "F19DATA0 register accessor: an alias for `Reg<F19DATA0_SPEC>`"]
pub type F19DATA0 = crate::Reg<f19data0::F19DATA0_SPEC>;
#[doc = "Filter 19 data 0 register"]
pub mod f19data0;
#[doc = "F19DATA1 register accessor: an alias for `Reg<F19DATA1_SPEC>`"]
pub type F19DATA1 = crate::Reg<f19data1::F19DATA1_SPEC>;
#[doc = "Filter 19 data 1 register"]
pub mod f19data1;
#[doc = "F20DATA0 register accessor: an alias for `Reg<F20DATA0_SPEC>`"]
pub type F20DATA0 = crate::Reg<f20data0::F20DATA0_SPEC>;
#[doc = "Filter 20 data 0 register"]
pub mod f20data0;
#[doc = "F20DATA1 register accessor: an alias for `Reg<F20DATA1_SPEC>`"]
pub type F20DATA1 = crate::Reg<f20data1::F20DATA1_SPEC>;
#[doc = "Filter 20 data 1 register"]
pub mod f20data1;
#[doc = "F21DATA0 register accessor: an alias for `Reg<F21DATA0_SPEC>`"]
pub type F21DATA0 = crate::Reg<f21data0::F21DATA0_SPEC>;
#[doc = "Filter 21 data 0 register"]
pub mod f21data0;
#[doc = "F21DATA1 register accessor: an alias for `Reg<F21DATA1_SPEC>`"]
pub type F21DATA1 = crate::Reg<f21data1::F21DATA1_SPEC>;
#[doc = "Filter 21 data 1 register"]
pub mod f21data1;
#[doc = "F22DATA0 register accessor: an alias for `Reg<F22DATA0_SPEC>`"]
pub type F22DATA0 = crate::Reg<f22data0::F22DATA0_SPEC>;
#[doc = "Filter 22 data 0 register"]
pub mod f22data0;
#[doc = "F22DATA1 register accessor: an alias for `Reg<F22DATA1_SPEC>`"]
pub type F22DATA1 = crate::Reg<f22data1::F22DATA1_SPEC>;
#[doc = "Filter 22 data 1 register"]
pub mod f22data1;
#[doc = "F23DATA0 register accessor: an alias for `Reg<F23DATA0_SPEC>`"]
pub type F23DATA0 = crate::Reg<f23data0::F23DATA0_SPEC>;
#[doc = "Filter 23 data 0 register"]
pub mod f23data0;
#[doc = "F23DATA1 register accessor: an alias for `Reg<F23DATA1_SPEC>`"]
pub type F23DATA1 = crate::Reg<f23data1::F23DATA1_SPEC>;
#[doc = "Filter 23 data 1 register"]
pub mod f23data1;
#[doc = "F24DATA0 register accessor: an alias for `Reg<F24DATA0_SPEC>`"]
pub type F24DATA0 = crate::Reg<f24data0::F24DATA0_SPEC>;
#[doc = "Filter 24 data 0 register"]
pub mod f24data0;
#[doc = "F24DATA1 register accessor: an alias for `Reg<F24DATA1_SPEC>`"]
pub type F24DATA1 = crate::Reg<f24data1::F24DATA1_SPEC>;
#[doc = "Filter 24 data 1 register"]
pub mod f24data1;
#[doc = "F25DATA0 register accessor: an alias for `Reg<F25DATA0_SPEC>`"]
pub type F25DATA0 = crate::Reg<f25data0::F25DATA0_SPEC>;
#[doc = "Filter 25 data 0 register"]
pub mod f25data0;
#[doc = "F25DATA1 register accessor: an alias for `Reg<F25DATA1_SPEC>`"]
pub type F25DATA1 = crate::Reg<f25data1::F25DATA1_SPEC>;
#[doc = "Filter 25 data 1 register"]
pub mod f25data1;
#[doc = "F26DATA0 register accessor: an alias for `Reg<F26DATA0_SPEC>`"]
pub type F26DATA0 = crate::Reg<f26data0::F26DATA0_SPEC>;
#[doc = "Filter 26 data 0 register"]
pub mod f26data0;
#[doc = "F26DATA1 register accessor: an alias for `Reg<F26DATA1_SPEC>`"]
pub type F26DATA1 = crate::Reg<f26data1::F26DATA1_SPEC>;
#[doc = "Filter 26 data 1 register"]
pub mod f26data1;
#[doc = "F27DATA0 register accessor: an alias for `Reg<F27DATA0_SPEC>`"]
pub type F27DATA0 = crate::Reg<f27data0::F27DATA0_SPEC>;
#[doc = "Filter 27 data 0 register"]
pub mod f27data0;
#[doc = "F27DATA1 register accessor: an alias for `Reg<F27DATA1_SPEC>`"]
pub type F27DATA1 = crate::Reg<f27data1::F27DATA1_SPEC>;
#[doc = "Filter 27 data 1 register"]
pub mod f27data1;
#[doc = "PHYCTL register accessor: an alias for `Reg<PHYCTL_SPEC>`"]
pub type PHYCTL = crate::Reg<phyctl::PHYCTL_SPEC>;
#[doc = "PHY control register"]
pub mod phyctl;
