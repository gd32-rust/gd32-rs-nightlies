#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub time: TIME,
    #[doc = "0x04 - date register"]
    pub date: DATE,
    #[doc = "0x08 - control register"]
    pub ctl: CTL,
    #[doc = "0x0c - initialization and status register"]
    pub stat: STAT,
    #[doc = "0x10 - prescaler register"]
    pub psc: PSC,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - alarm A register"]
    pub alrm0td: ALRM0TD,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - write protection register"]
    pub wpk: WPK,
    #[doc = "0x28 - sub second register"]
    pub ss: SS,
    #[doc = "0x2c - shift control register"]
    pub shiftctl: SHIFTCTL,
    #[doc = "0x30 - timestamp time register"]
    pub tts: TTS,
    #[doc = "0x34 - Date of time stamp register"]
    pub dts: DTS,
    #[doc = "0x38 - time-stamp sub second register"]
    pub ssts: SSTS,
    #[doc = "0x3c - High resolution frequency compensation register"]
    pub hrfc: HRFC,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tamp: TAMP,
    #[doc = "0x44 - alarm 0 sub second register"]
    pub alrm0ss: ALRM0SS,
    _reserved15: [u8; 0x08],
    #[doc = "0x50 - backup register"]
    pub bkp0: BKP0,
    #[doc = "0x54 - backup register"]
    pub bkp1: BKP1,
    #[doc = "0x58 - backup register"]
    pub bkp2: BKP2,
    #[doc = "0x5c - backup register"]
    pub bkp3: BKP3,
    #[doc = "0x60 - backup register"]
    pub bkp4: BKP4,
}
#[doc = "TIME (rw) register accessor: time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time`]
module"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "time register"]
pub mod time;
#[doc = "DATE (rw) register accessor: date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`]
module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "date register"]
pub mod date;
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "initialization and status register"]
pub mod stat;
#[doc = "PSC (rw) register accessor: prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psc`]
module"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler register"]
pub mod psc;
#[doc = "ALRM0TD (rw) register accessor: alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0td::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0td::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`alrm0td`]
module"]
pub type ALRM0TD = crate::Reg<alrm0td::ALRM0TD_SPEC>;
#[doc = "alarm A register"]
pub mod alrm0td;
#[doc = "WPK (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpk`]
module"]
pub type WPK = crate::Reg<wpk::WPK_SPEC>;
#[doc = "write protection register"]
pub mod wpk;
#[doc = "SS (r) register accessor: sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ss`]
module"]
pub type SS = crate::Reg<ss::SS_SPEC>;
#[doc = "sub second register"]
pub mod ss;
#[doc = "SHIFTCTL (w) register accessor: shift control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`shiftctl`]
module"]
pub type SHIFTCTL = crate::Reg<shiftctl::SHIFTCTL_SPEC>;
#[doc = "shift control register"]
pub mod shiftctl;
#[doc = "TTS (r) register accessor: timestamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tts`]
module"]
pub type TTS = crate::Reg<tts::TTS_SPEC>;
#[doc = "timestamp time register"]
pub mod tts;
#[doc = "DTS (r) register accessor: Date of time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dts`]
module"]
pub type DTS = crate::Reg<dts::DTS_SPEC>;
#[doc = "Date of time stamp register"]
pub mod dts;
#[doc = "SSTS (r) register accessor: time-stamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ssts`]
module"]
pub type SSTS = crate::Reg<ssts::SSTS_SPEC>;
#[doc = "time-stamp sub second register"]
pub mod ssts;
#[doc = "HRFC (rw) register accessor: High resolution frequency compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hrfc`]
module"]
pub type HRFC = crate::Reg<hrfc::HRFC_SPEC>;
#[doc = "High resolution frequency compensation register"]
pub mod hrfc;
#[doc = "TAMP (rw) register accessor: tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tamp`]
module"]
pub type TAMP = crate::Reg<tamp::TAMP_SPEC>;
#[doc = "tamper and alternate function configuration register"]
pub mod tamp;
#[doc = "ALRM0SS (rw) register accessor: alarm 0 sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0ss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0ss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`alrm0ss`]
module"]
pub type ALRM0SS = crate::Reg<alrm0ss::ALRM0SS_SPEC>;
#[doc = "alarm 0 sub second register"]
pub mod alrm0ss;
#[doc = "BKP0 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bkp0`]
module"]
pub type BKP0 = crate::Reg<bkp0::BKP0_SPEC>;
#[doc = "backup register"]
pub mod bkp0;
#[doc = "BKP1 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bkp1`]
module"]
pub type BKP1 = crate::Reg<bkp1::BKP1_SPEC>;
#[doc = "backup register"]
pub mod bkp1;
#[doc = "BKP2 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bkp2`]
module"]
pub type BKP2 = crate::Reg<bkp2::BKP2_SPEC>;
#[doc = "backup register"]
pub mod bkp2;
#[doc = "BKP3 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bkp3`]
module"]
pub type BKP3 = crate::Reg<bkp3::BKP3_SPEC>;
#[doc = "backup register"]
pub mod bkp3;
#[doc = "BKP4 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bkp4`]
module"]
pub type BKP4 = crate::Reg<bkp4::BKP4_SPEC>;
#[doc = "backup register"]
pub mod bkp4;
