#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x04 - control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x08 - control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    #[doc = "0x0c - Sampling time register 1"]
    pub sampt0: crate::Reg<sampt0::SAMPT0_SPEC>,
    #[doc = "0x10 - Sampling time register 2"]
    pub sampt1: crate::Reg<sampt1::SAMPT1_SPEC>,
    #[doc = "0x14 - Inserted channel data offset register 0"]
    pub ioff0: crate::Reg<ioff0::IOFF0_SPEC>,
    #[doc = "0x18 - Inserted channel data offset register 1"]
    pub ioff1: crate::Reg<ioff1::IOFF1_SPEC>,
    #[doc = "0x1c - Inserted channel data offset register 2"]
    pub ioff2: crate::Reg<ioff2::IOFF2_SPEC>,
    #[doc = "0x20 - Inserted channel data offset register 3"]
    pub ioff3: crate::Reg<ioff3::IOFF3_SPEC>,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub wdht: crate::Reg<wdht::WDHT_SPEC>,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub wdlt: crate::Reg<wdlt::WDLT_SPEC>,
    #[doc = "0x2c - regular sequence register 0"]
    pub rsq0: crate::Reg<rsq0::RSQ0_SPEC>,
    #[doc = "0x30 - regular sequence register 1"]
    pub rsq1: crate::Reg<rsq1::RSQ1_SPEC>,
    #[doc = "0x34 - regular sequence register 2"]
    pub rsq2: crate::Reg<rsq2::RSQ2_SPEC>,
    #[doc = "0x38 - Inserted sequence register"]
    pub isq: crate::Reg<isq::ISQ_SPEC>,
    #[doc = "0x3c - injected data register 0"]
    pub idata0: crate::Reg<idata0::IDATA0_SPEC>,
    #[doc = "0x40 - injected data register 1"]
    pub idata1: crate::Reg<idata1::IDATA1_SPEC>,
    #[doc = "0x44 - injected data register 2"]
    pub idata2: crate::Reg<idata2::IDATA2_SPEC>,
    #[doc = "0x48 - injected data register 3"]
    pub idata3: crate::Reg<idata3::IDATA3_SPEC>,
    #[doc = "0x4c - regular data register"]
    pub rdata: crate::Reg<rdata::RDATA_SPEC>,
}
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "status register"]
pub mod stat;
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "SAMPT0 register accessor: an alias for `Reg<SAMPT0_SPEC>`"]
pub type SAMPT0 = crate::Reg<sampt0::SAMPT0_SPEC>;
#[doc = "Sampling time register 1"]
pub mod sampt0;
#[doc = "SAMPT1 register accessor: an alias for `Reg<SAMPT1_SPEC>`"]
pub type SAMPT1 = crate::Reg<sampt1::SAMPT1_SPEC>;
#[doc = "Sampling time register 2"]
pub mod sampt1;
#[doc = "IOFF0 register accessor: an alias for `Reg<IOFF0_SPEC>`"]
pub type IOFF0 = crate::Reg<ioff0::IOFF0_SPEC>;
#[doc = "Inserted channel data offset register 0"]
pub mod ioff0;
#[doc = "IOFF1 register accessor: an alias for `Reg<IOFF1_SPEC>`"]
pub type IOFF1 = crate::Reg<ioff1::IOFF1_SPEC>;
#[doc = "Inserted channel data offset register 1"]
pub mod ioff1;
#[doc = "IOFF2 register accessor: an alias for `Reg<IOFF2_SPEC>`"]
pub type IOFF2 = crate::Reg<ioff2::IOFF2_SPEC>;
#[doc = "Inserted channel data offset register 2"]
pub mod ioff2;
#[doc = "IOFF3 register accessor: an alias for `Reg<IOFF3_SPEC>`"]
pub type IOFF3 = crate::Reg<ioff3::IOFF3_SPEC>;
#[doc = "Inserted channel data offset register 3"]
pub mod ioff3;
#[doc = "WDHT register accessor: an alias for `Reg<WDHT_SPEC>`"]
pub type WDHT = crate::Reg<wdht::WDHT_SPEC>;
#[doc = "watchdog higher threshold register"]
pub mod wdht;
#[doc = "WDLT register accessor: an alias for `Reg<WDLT_SPEC>`"]
pub type WDLT = crate::Reg<wdlt::WDLT_SPEC>;
#[doc = "watchdog lower threshold register"]
pub mod wdlt;
#[doc = "RSQ0 register accessor: an alias for `Reg<RSQ0_SPEC>`"]
pub type RSQ0 = crate::Reg<rsq0::RSQ0_SPEC>;
#[doc = "regular sequence register 0"]
pub mod rsq0;
#[doc = "RSQ1 register accessor: an alias for `Reg<RSQ1_SPEC>`"]
pub type RSQ1 = crate::Reg<rsq1::RSQ1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod rsq1;
#[doc = "RSQ2 register accessor: an alias for `Reg<RSQ2_SPEC>`"]
pub type RSQ2 = crate::Reg<rsq2::RSQ2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod rsq2;
#[doc = "ISQ register accessor: an alias for `Reg<ISQ_SPEC>`"]
pub type ISQ = crate::Reg<isq::ISQ_SPEC>;
#[doc = "Inserted sequence register"]
pub mod isq;
#[doc = "IDATA0 register accessor: an alias for `Reg<IDATA0_SPEC>`"]
pub type IDATA0 = crate::Reg<idata0::IDATA0_SPEC>;
#[doc = "injected data register 0"]
pub mod idata0;
#[doc = "IDATA1 register accessor: an alias for `Reg<IDATA1_SPEC>`"]
pub type IDATA1 = crate::Reg<idata1::IDATA1_SPEC>;
#[doc = "injected data register 1"]
pub mod idata1;
#[doc = "IDATA2 register accessor: an alias for `Reg<IDATA2_SPEC>`"]
pub type IDATA2 = crate::Reg<idata2::IDATA2_SPEC>;
#[doc = "injected data register 2"]
pub mod idata2;
#[doc = "IDATA3 register accessor: an alias for `Reg<IDATA3_SPEC>`"]
pub type IDATA3 = crate::Reg<idata3::IDATA3_SPEC>;
#[doc = "injected data register 3"]
pub mod idata3;
#[doc = "RDATA register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "regular data register"]
pub mod rdata;
