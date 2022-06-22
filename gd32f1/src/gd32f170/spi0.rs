#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - data register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - CRC polynomial register"]
    pub crcpoly: crate::Reg<crcpoly::CRCPOLY_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - Receive CRC register"]
    pub rcrc: crate::Reg<rcrc::RCRC_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - Transmit CRC register"]
    pub tcrc: crate::Reg<tcrc::TCRC_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - I2S control register"]
    pub i2sctl: crate::Reg<i2sctl::I2SCTL_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x20 - I2S prescaler register"]
    pub i2spsc: crate::Reg<i2spsc::I2SPSC_SPEC>,
    _reserved9: [u8; 0x5e],
    #[doc = "0x80 - Quad wire control register"]
    pub qctl: crate::Reg<qctl::QCTL_SPEC>,
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "status register"]
pub mod stat;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "data register"]
pub mod data;
#[doc = "CRCPOLY register accessor: an alias for `Reg<CRCPOLY_SPEC>`"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
#[doc = "CRC polynomial register"]
pub mod crcpoly;
#[doc = "RCRC register accessor: an alias for `Reg<RCRC_SPEC>`"]
pub type RCRC = crate::Reg<rcrc::RCRC_SPEC>;
#[doc = "Receive CRC register"]
pub mod rcrc;
#[doc = "TCRC register accessor: an alias for `Reg<TCRC_SPEC>`"]
pub type TCRC = crate::Reg<tcrc::TCRC_SPEC>;
#[doc = "Transmit CRC register"]
pub mod tcrc;
#[doc = "I2SCTL register accessor: an alias for `Reg<I2SCTL_SPEC>`"]
pub type I2SCTL = crate::Reg<i2sctl::I2SCTL_SPEC>;
#[doc = "I2S control register"]
pub mod i2sctl;
#[doc = "I2SPSC register accessor: an alias for `Reg<I2SPSC_SPEC>`"]
pub type I2SPSC = crate::Reg<i2spsc::I2SPSC_SPEC>;
#[doc = "I2S prescaler register"]
pub mod i2spsc;
#[doc = "QCTL register accessor: an alias for `Reg<QCTL_SPEC>`"]
pub type QCTL = crate::Reg<qctl::QCTL_SPEC>;
#[doc = "Quad wire control register"]
pub mod qctl;
