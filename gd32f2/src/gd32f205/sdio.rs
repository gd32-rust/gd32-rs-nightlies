#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pwrctl: crate::Reg<pwrctl::PWRCTL_SPEC>,
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCTL)"]
    pub clkctl: crate::Reg<clkctl::CLKCTL_SPEC>,
    #[doc = "0x08 - Command argument register"]
    pub cmdagmt: crate::Reg<cmdagmt::CMDAGMT_SPEC>,
    #[doc = "0x0c - Command control register"]
    pub cmdctl: crate::Reg<cmdctl::CMDCTL_SPEC>,
    #[doc = "0x10 - Command index response register"]
    pub rspcmdidx: crate::Reg<rspcmdidx::RSPCMDIDX_SPEC>,
    #[doc = "0x14 - Response register 0"]
    pub resp0: crate::Reg<resp0::RESP0_SPEC>,
    #[doc = "0x18 - Response register 1"]
    pub resp1: crate::Reg<resp1::RESP1_SPEC>,
    #[doc = "0x1c - Response register 2"]
    pub resp2: crate::Reg<resp2::RESP2_SPEC>,
    #[doc = "0x20 - Response register 3"]
    pub resp3: crate::Reg<resp3::RESP3_SPEC>,
    #[doc = "0x24 - Data timeout register"]
    pub datato: crate::Reg<datato::DATATO_SPEC>,
    #[doc = "0x28 - Data length register"]
    pub datalen: crate::Reg<datalen::DATALEN_SPEC>,
    #[doc = "0x2c - Data control register"]
    pub datactl: crate::Reg<datactl::DATACTL_SPEC>,
    #[doc = "0x30 - Data counter register"]
    pub datacnt: crate::Reg<datacnt::DATACNT_SPEC>,
    #[doc = "0x34 - SDIO status register (SDIO_STR)"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x38 - Interrupt clear register (SDIO_INTC)"]
    pub intc: crate::Reg<intc::INTC_SPEC>,
    #[doc = "0x3c - Interrupt Enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x48 - FIFO counter register"]
    pub fifocnt: crate::Reg<fifocnt::FIFOCNT_SPEC>,
    _reserved17: [u8; 0x34],
    #[doc = "0x80 - FIFO data register"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
}
#[doc = "PWRCTL register accessor: an alias for `Reg<PWRCTL_SPEC>`"]
pub type PWRCTL = crate::Reg<pwrctl::PWRCTL_SPEC>;
#[doc = "Power control register"]
pub mod pwrctl;
#[doc = "CLKCTL register accessor: an alias for `Reg<CLKCTL_SPEC>`"]
pub type CLKCTL = crate::Reg<clkctl::CLKCTL_SPEC>;
#[doc = "SDI clock control register (SDIO_CLKCTL)"]
pub mod clkctl;
#[doc = "CMDAGMT register accessor: an alias for `Reg<CMDAGMT_SPEC>`"]
pub type CMDAGMT = crate::Reg<cmdagmt::CMDAGMT_SPEC>;
#[doc = "Command argument register"]
pub mod cmdagmt;
#[doc = "CMDCTL register accessor: an alias for `Reg<CMDCTL_SPEC>`"]
pub type CMDCTL = crate::Reg<cmdctl::CMDCTL_SPEC>;
#[doc = "Command control register"]
pub mod cmdctl;
#[doc = "RSPCMDIDX register accessor: an alias for `Reg<RSPCMDIDX_SPEC>`"]
pub type RSPCMDIDX = crate::Reg<rspcmdidx::RSPCMDIDX_SPEC>;
#[doc = "Command index response register"]
pub mod rspcmdidx;
#[doc = "RESP0 register accessor: an alias for `Reg<RESP0_SPEC>`"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Response register 0"]
pub mod resp0;
#[doc = "RESP1 register accessor: an alias for `Reg<RESP1_SPEC>`"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "Response register 1"]
pub mod resp1;
#[doc = "RESP2 register accessor: an alias for `Reg<RESP2_SPEC>`"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Response register 2"]
pub mod resp2;
#[doc = "RESP3 register accessor: an alias for `Reg<RESP3_SPEC>`"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "Response register 3"]
pub mod resp3;
#[doc = "DATATO register accessor: an alias for `Reg<DATATO_SPEC>`"]
pub type DATATO = crate::Reg<datato::DATATO_SPEC>;
#[doc = "Data timeout register"]
pub mod datato;
#[doc = "DATALEN register accessor: an alias for `Reg<DATALEN_SPEC>`"]
pub type DATALEN = crate::Reg<datalen::DATALEN_SPEC>;
#[doc = "Data length register"]
pub mod datalen;
#[doc = "DATACTL register accessor: an alias for `Reg<DATACTL_SPEC>`"]
pub type DATACTL = crate::Reg<datactl::DATACTL_SPEC>;
#[doc = "Data control register"]
pub mod datactl;
#[doc = "DATACNT register accessor: an alias for `Reg<DATACNT_SPEC>`"]
pub type DATACNT = crate::Reg<datacnt::DATACNT_SPEC>;
#[doc = "Data counter register"]
pub mod datacnt;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SDIO status register (SDIO_STR)"]
pub mod stat;
#[doc = "INTC register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt clear register (SDIO_INTC)"]
pub mod intc;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable register"]
pub mod inten;
#[doc = "FIFOCNT register accessor: an alias for `Reg<FIFOCNT_SPEC>`"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO data register"]
pub mod fifo;
