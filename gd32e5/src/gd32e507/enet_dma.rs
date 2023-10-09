#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus control register"]
    pub dma_bctl: DMA_BCTL,
    #[doc = "0x04 - Ethernet DMA transmit poll enable register"]
    pub dma_tpen: DMA_TPEN,
    #[doc = "0x08 - Ethernet DMA receive poll enable register"]
    pub dma_rpen: DMA_RPEN,
    #[doc = "0x0c - Ethernet DMA receive descriptor table address register"]
    pub dma_rdtaddr: DMA_RDTADDR,
    #[doc = "0x10 - Ethernet DMA transmit descriptor table address register"]
    pub dma_tdtaddr: DMA_TDTADDR,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dma_stat: DMA_STAT,
    #[doc = "0x18 - Ethernet DMA control register"]
    pub dma_ctl: DMA_CTL,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dma_inten: DMA_INTEN,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dma_mfbocnt: DMA_MFBOCNT,
    #[doc = "0x24 - Ethernet DMA receive state watchdog counter register"]
    pub dma_rswdc: DMA_RSWDC,
    _reserved10: [u8; 0x20],
    #[doc = "0x48 - DMA current transmit descriptor address register"]
    pub dma_ctdaddr: DMA_CTDADDR,
    #[doc = "0x4c - Ethernet DMA current receive descriptor address register"]
    pub dma_crdaddr: DMA_CRDADDR,
    #[doc = "0x50 - Ethernet DMA current transmit buffer address register"]
    pub dma_ctbaddr: DMA_CTBADDR,
    #[doc = "0x54 - Ethernet DMA current receive buffer address register"]
    pub dma_crbaddr: DMA_CRBADDR,
}
#[doc = "DMA_BCTL (rw) register accessor: Ethernet DMA bus control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_bctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_bctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_bctl`]
module"]
pub type DMA_BCTL = crate::Reg<dma_bctl::DMA_BCTL_SPEC>;
#[doc = "Ethernet DMA bus control register"]
pub mod dma_bctl;
#[doc = "DMA_TPEN (rw) register accessor: Ethernet DMA transmit poll enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_tpen`]
module"]
pub type DMA_TPEN = crate::Reg<dma_tpen::DMA_TPEN_SPEC>;
#[doc = "Ethernet DMA transmit poll enable register"]
pub mod dma_tpen;
#[doc = "DMA_RPEN (rw) register accessor: Ethernet DMA receive poll enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_rpen`]
module"]
pub type DMA_RPEN = crate::Reg<dma_rpen::DMA_RPEN_SPEC>;
#[doc = "Ethernet DMA receive poll enable register"]
pub mod dma_rpen;
#[doc = "DMA_RDTADDR (rw) register accessor: Ethernet DMA receive descriptor table address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rdtaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rdtaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_rdtaddr`]
module"]
pub type DMA_RDTADDR = crate::Reg<dma_rdtaddr::DMA_RDTADDR_SPEC>;
#[doc = "Ethernet DMA receive descriptor table address register"]
pub mod dma_rdtaddr;
#[doc = "DMA_TDTADDR (rw) register accessor: Ethernet DMA transmit descriptor table address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tdtaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tdtaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_tdtaddr`]
module"]
pub type DMA_TDTADDR = crate::Reg<dma_tdtaddr::DMA_TDTADDR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor table address register"]
pub mod dma_tdtaddr;
#[doc = "DMA_STAT (rw) register accessor: Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_stat`]
module"]
pub type DMA_STAT = crate::Reg<dma_stat::DMA_STAT_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dma_stat;
#[doc = "DMA_CTL (rw) register accessor: Ethernet DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_ctl`]
module"]
pub type DMA_CTL = crate::Reg<dma_ctl::DMA_CTL_SPEC>;
#[doc = "Ethernet DMA control register"]
pub mod dma_ctl;
#[doc = "DMA_INTEN (rw) register accessor: Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_inten`]
module"]
pub type DMA_INTEN = crate::Reg<dma_inten::DMA_INTEN_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dma_inten;
#[doc = "DMA_MFBOCNT (r) register accessor: Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_mfbocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_mfbocnt`]
module"]
pub type DMA_MFBOCNT = crate::Reg<dma_mfbocnt::DMA_MFBOCNT_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dma_mfbocnt;
#[doc = "DMA_RSWDC (rw) register accessor: Ethernet DMA receive state watchdog counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rswdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rswdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_rswdc`]
module"]
pub type DMA_RSWDC = crate::Reg<dma_rswdc::DMA_RSWDC_SPEC>;
#[doc = "Ethernet DMA receive state watchdog counter register"]
pub mod dma_rswdc;
#[doc = "DMA_CTDADDR (r) register accessor: DMA current transmit descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctdaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_ctdaddr`]
module"]
pub type DMA_CTDADDR = crate::Reg<dma_ctdaddr::DMA_CTDADDR_SPEC>;
#[doc = "DMA current transmit descriptor address register"]
pub mod dma_ctdaddr;
#[doc = "DMA_CRDADDR (r) register accessor: Ethernet DMA current receive descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_crdaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_crdaddr`]
module"]
pub type DMA_CRDADDR = crate::Reg<dma_crdaddr::DMA_CRDADDR_SPEC>;
#[doc = "Ethernet DMA current receive descriptor address register"]
pub mod dma_crdaddr;
#[doc = "DMA_CTBADDR (r) register accessor: Ethernet DMA current transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctbaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_ctbaddr`]
module"]
pub type DMA_CTBADDR = crate::Reg<dma_ctbaddr::DMA_CTBADDR_SPEC>;
#[doc = "Ethernet DMA current transmit buffer address register"]
pub mod dma_ctbaddr;
#[doc = "DMA_CRBADDR (r) register accessor: Ethernet DMA current receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_crbaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_crbaddr`]
module"]
pub type DMA_CRBADDR = crate::Reg<dma_crbaddr::DMA_CRBADDR_SPEC>;
#[doc = "Ethernet DMA current receive buffer address register"]
pub mod dma_crbaddr;
