#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Time of day register"]
    pub time: crate::Reg<time::TIME_SPEC>,
    #[doc = "0x04 - date register"]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0x08 - control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x0c - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x10 - Time prescaler register"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - Alarm 0 Time and date register"]
    pub alrm0td: crate::Reg<alrm0td::ALRM0TD_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - Write protection key register"]
    pub wpk: crate::Reg<wpk::WPK_SPEC>,
    #[doc = "0x28 - sub second register"]
    pub ss: crate::Reg<ss::SS_SPEC>,
    #[doc = "0x2c - Shift function control register"]
    pub shiftctl: crate::Reg<shiftctl::SHIFTCTL_SPEC>,
    #[doc = "0x30 - Time of time stamp register"]
    pub tts: crate::Reg<tts::TTS_SPEC>,
    #[doc = "0x34 - Date of time stamp register"]
    pub dts: crate::Reg<dts::DTS_SPEC>,
    #[doc = "0x38 - Sub second of time stamp register"]
    pub ssts: crate::Reg<ssts::SSTS_SPEC>,
    #[doc = "0x3c - High resolution frequency compensation register"]
    pub hrfc: crate::Reg<hrfc::HRFC_SPEC>,
    #[doc = "0x40 - Tamper register"]
    pub tamp: crate::Reg<tamp::TAMP_SPEC>,
    #[doc = "0x44 - Alarm 0 sub second register"]
    pub alrm0ss: crate::Reg<alrm0ss::ALRM0SS_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x50 - backup register"]
    pub bkp0: crate::Reg<bkp0::BKP0_SPEC>,
    #[doc = "0x54 - backup register"]
    pub bkp1: crate::Reg<bkp1::BKP1_SPEC>,
    #[doc = "0x58 - backup register"]
    pub bkp2: crate::Reg<bkp2::BKP2_SPEC>,
    #[doc = "0x5c - backup register"]
    pub bkp3: crate::Reg<bkp3::BKP3_SPEC>,
    #[doc = "0x60 - backup register"]
    pub bkp4: crate::Reg<bkp4::BKP4_SPEC>,
}
#[doc = "TIME register accessor: an alias for `Reg<TIME_SPEC>`"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "Time of day register"]
pub mod time;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "date register"]
pub mod date;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Time prescaler register"]
pub mod psc;
#[doc = "ALRM0TD register accessor: an alias for `Reg<ALRM0TD_SPEC>`"]
pub type ALRM0TD = crate::Reg<alrm0td::ALRM0TD_SPEC>;
#[doc = "Alarm 0 Time and date register"]
pub mod alrm0td;
#[doc = "WPK register accessor: an alias for `Reg<WPK_SPEC>`"]
pub type WPK = crate::Reg<wpk::WPK_SPEC>;
#[doc = "Write protection key register"]
pub mod wpk;
#[doc = "SS register accessor: an alias for `Reg<SS_SPEC>`"]
pub type SS = crate::Reg<ss::SS_SPEC>;
#[doc = "sub second register"]
pub mod ss;
#[doc = "SHIFTCTL register accessor: an alias for `Reg<SHIFTCTL_SPEC>`"]
pub type SHIFTCTL = crate::Reg<shiftctl::SHIFTCTL_SPEC>;
#[doc = "Shift function control register"]
pub mod shiftctl;
#[doc = "TTS register accessor: an alias for `Reg<TTS_SPEC>`"]
pub type TTS = crate::Reg<tts::TTS_SPEC>;
#[doc = "Time of time stamp register"]
pub mod tts;
#[doc = "DTS register accessor: an alias for `Reg<DTS_SPEC>`"]
pub type DTS = crate::Reg<dts::DTS_SPEC>;
#[doc = "Date of time stamp register"]
pub mod dts;
#[doc = "SSTS register accessor: an alias for `Reg<SSTS_SPEC>`"]
pub type SSTS = crate::Reg<ssts::SSTS_SPEC>;
#[doc = "Sub second of time stamp register"]
pub mod ssts;
#[doc = "HRFC register accessor: an alias for `Reg<HRFC_SPEC>`"]
pub type HRFC = crate::Reg<hrfc::HRFC_SPEC>;
#[doc = "High resolution frequency compensation register"]
pub mod hrfc;
#[doc = "TAMP register accessor: an alias for `Reg<TAMP_SPEC>`"]
pub type TAMP = crate::Reg<tamp::TAMP_SPEC>;
#[doc = "Tamper register"]
pub mod tamp;
#[doc = "ALRM0SS register accessor: an alias for `Reg<ALRM0SS_SPEC>`"]
pub type ALRM0SS = crate::Reg<alrm0ss::ALRM0SS_SPEC>;
#[doc = "Alarm 0 sub second register"]
pub mod alrm0ss;
#[doc = "BKP0 register accessor: an alias for `Reg<BKP0_SPEC>`"]
pub type BKP0 = crate::Reg<bkp0::BKP0_SPEC>;
#[doc = "backup register"]
pub mod bkp0;
#[doc = "BKP1 register accessor: an alias for `Reg<BKP1_SPEC>`"]
pub type BKP1 = crate::Reg<bkp1::BKP1_SPEC>;
#[doc = "backup register"]
pub mod bkp1;
#[doc = "BKP2 register accessor: an alias for `Reg<BKP2_SPEC>`"]
pub type BKP2 = crate::Reg<bkp2::BKP2_SPEC>;
#[doc = "backup register"]
pub mod bkp2;
#[doc = "BKP3 register accessor: an alias for `Reg<BKP3_SPEC>`"]
pub type BKP3 = crate::Reg<bkp3::BKP3_SPEC>;
#[doc = "backup register"]
pub mod bkp3;
#[doc = "BKP4 register accessor: an alias for `Reg<BKP4_SPEC>`"]
pub type BKP4 = crate::Reg<bkp4::BKP4_SPEC>;
#[doc = "backup register"]
pub mod bkp4;
