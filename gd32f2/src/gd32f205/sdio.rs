#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pwrctl: PWRCTL,
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCTL)"]
    pub clkctl: CLKCTL,
    #[doc = "0x08 - Command argument register"]
    pub cmdagmt: CMDAGMT,
    #[doc = "0x0c - Command control register"]
    pub cmdctl: CMDCTL,
    #[doc = "0x10 - Command index response register"]
    pub rspcmdidx: RSPCMDIDX,
    #[doc = "0x14 - Response register 0"]
    pub resp0: RESP0,
    #[doc = "0x18 - Response register 1"]
    pub resp1: RESP1,
    #[doc = "0x1c - Response register 2"]
    pub resp2: RESP2,
    #[doc = "0x20 - Response register 3"]
    pub resp3: RESP3,
    #[doc = "0x24 - Data timeout register"]
    pub datato: DATATO,
    #[doc = "0x28 - Data length register"]
    pub datalen: DATALEN,
    #[doc = "0x2c - Data control register"]
    pub datactl: DATACTL,
    #[doc = "0x30 - Data counter register"]
    pub datacnt: DATACNT,
    #[doc = "0x34 - SDIO status register (SDIO_STR)"]
    pub stat: STAT,
    #[doc = "0x38 - Interrupt clear register (SDIO_INTC)"]
    pub intc: INTC,
    #[doc = "0x3c - Interrupt Enable register"]
    pub inten: INTEN,
    _reserved16: [u8; 0x08],
    #[doc = "0x48 - FIFO counter register"]
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 0x34],
    #[doc = "0x80 - FIFO data register"]
    pub fifo: FIFO,
}
#[doc = "PWRCTL (rw) register accessor: Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwrctl`]
module"]
pub type PWRCTL = crate::Reg<pwrctl::PWRCTL_SPEC>;
#[doc = "Power control register"]
pub mod pwrctl;
#[doc = "CLKCTL (rw) register accessor: SDI clock control register (SDIO_CLKCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkctl`]
module"]
pub type CLKCTL = crate::Reg<clkctl::CLKCTL_SPEC>;
#[doc = "SDI clock control register (SDIO_CLKCTL)"]
pub mod clkctl;
#[doc = "CMDAGMT (rw) register accessor: Command argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdagmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdagmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmdagmt`]
module"]
pub type CMDAGMT = crate::Reg<cmdagmt::CMDAGMT_SPEC>;
#[doc = "Command argument register"]
pub mod cmdagmt;
#[doc = "CMDCTL (rw) register accessor: Command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmdctl`]
module"]
pub type CMDCTL = crate::Reg<cmdctl::CMDCTL_SPEC>;
#[doc = "Command control register"]
pub mod cmdctl;
#[doc = "RSPCMDIDX (r) register accessor: Command index response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspcmdidx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rspcmdidx`]
module"]
pub type RSPCMDIDX = crate::Reg<rspcmdidx::RSPCMDIDX_SPEC>;
#[doc = "Command index response register"]
pub mod rspcmdidx;
#[doc = "RESP0 (r) register accessor: Response register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp0`]
module"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Response register 0"]
pub mod resp0;
#[doc = "RESP1 (r) register accessor: Response register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp1`]
module"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "Response register 1"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: Response register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp2`]
module"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Response register 2"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: Response register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp3`]
module"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "Response register 3"]
pub mod resp3;
#[doc = "DATATO (rw) register accessor: Data timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datato::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datato::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datato`]
module"]
pub type DATATO = crate::Reg<datato::DATATO_SPEC>;
#[doc = "Data timeout register"]
pub mod datato;
#[doc = "DATALEN (rw) register accessor: Data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datalen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datalen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datalen`]
module"]
pub type DATALEN = crate::Reg<datalen::DATALEN_SPEC>;
#[doc = "Data length register"]
pub mod datalen;
#[doc = "DATACTL (rw) register accessor: Data control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datactl`]
module"]
pub type DATACTL = crate::Reg<datactl::DATACTL_SPEC>;
#[doc = "Data control register"]
pub mod datactl;
#[doc = "DATACNT (r) register accessor: Data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datacnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datacnt`]
module"]
pub type DATACNT = crate::Reg<datacnt::DATACNT_SPEC>;
#[doc = "Data counter register"]
pub mod datacnt;
#[doc = "STAT (r) register accessor: SDIO status register (SDIO_STR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SDIO status register (SDIO_STR)"]
pub mod stat;
#[doc = "INTC (w) register accessor: Interrupt clear register (SDIO_INTC)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt clear register (SDIO_INTC)"]
pub mod intc;
#[doc = "INTEN (rw) register accessor: Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable register"]
pub mod inten;
#[doc = "FIFOCNT (r) register accessor: FIFO counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifocnt`]
module"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "FIFO (rw) register accessor: FIFO data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO data register"]
pub mod fifo;
