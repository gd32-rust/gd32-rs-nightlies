#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus control register"]
    pub dma_bctl: crate::Reg<dma_bctl::DMA_BCTL_SPEC>,
    #[doc = "0x04 - Ethernet DMA transmit poll enable register"]
    pub dma_tpen: crate::Reg<dma_tpen::DMA_TPEN_SPEC>,
    #[doc = "0x08 - Ethernet DMA receive poll enable register"]
    pub dma_rpen: crate::Reg<dma_rpen::DMA_RPEN_SPEC>,
    #[doc = "0x0c - Ethernet DMA receive descriptor table address register"]
    pub dma_rdtaddr: crate::Reg<dma_rdtaddr::DMA_RDTADDR_SPEC>,
    #[doc = "0x10 - Ethernet DMA transmit descriptor table address register"]
    pub dma_tdtaddr: crate::Reg<dma_tdtaddr::DMA_TDTADDR_SPEC>,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dma_stat: crate::Reg<dma_stat::DMA_STAT_SPEC>,
    #[doc = "0x18 - Ethernet DMA control register"]
    pub dma_ctl: crate::Reg<dma_ctl::DMA_CTL_SPEC>,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dma_inten: crate::Reg<dma_inten::DMA_INTEN_SPEC>,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dma_mfbocnt: crate::Reg<dma_mfbocnt::DMA_MFBOCNT_SPEC>,
    #[doc = "0x24 - Ethernet DMA receive state watchdog counter register"]
    pub dma_rswdc: crate::Reg<dma_rswdc::DMA_RSWDC_SPEC>,
    _reserved10: [u8; 0x20],
    #[doc = "0x48 - DMA current transmit descriptor address register"]
    pub dma_ctdaddr: crate::Reg<dma_ctdaddr::DMA_CTDADDR_SPEC>,
    #[doc = "0x4c - Ethernet DMA current receive descriptor address register"]
    pub dma_crdaddr: crate::Reg<dma_crdaddr::DMA_CRDADDR_SPEC>,
    #[doc = "0x50 - Ethernet DMA current transmit buffer address register"]
    pub dma_ctbaddr: crate::Reg<dma_ctbaddr::DMA_CTBADDR_SPEC>,
    #[doc = "0x54 - Ethernet DMA current receive buffer address register"]
    pub dma_crbaddr: crate::Reg<dma_crbaddr::DMA_CRBADDR_SPEC>,
}
#[doc = "DMA_BCTL register accessor: an alias for `Reg<DMA_BCTL_SPEC>`"]
pub type DMA_BCTL = crate::Reg<dma_bctl::DMA_BCTL_SPEC>;
#[doc = "Ethernet DMA bus control register"]
pub mod dma_bctl;
#[doc = "DMA_TPEN register accessor: an alias for `Reg<DMA_TPEN_SPEC>`"]
pub type DMA_TPEN = crate::Reg<dma_tpen::DMA_TPEN_SPEC>;
#[doc = "Ethernet DMA transmit poll enable register"]
pub mod dma_tpen;
#[doc = "DMA_RPEN register accessor: an alias for `Reg<DMA_RPEN_SPEC>`"]
pub type DMA_RPEN = crate::Reg<dma_rpen::DMA_RPEN_SPEC>;
#[doc = "Ethernet DMA receive poll enable register"]
pub mod dma_rpen;
#[doc = "DMA_RDTADDR register accessor: an alias for `Reg<DMA_RDTADDR_SPEC>`"]
pub type DMA_RDTADDR = crate::Reg<dma_rdtaddr::DMA_RDTADDR_SPEC>;
#[doc = "Ethernet DMA receive descriptor table address register"]
pub mod dma_rdtaddr;
#[doc = "DMA_TDTADDR register accessor: an alias for `Reg<DMA_TDTADDR_SPEC>`"]
pub type DMA_TDTADDR = crate::Reg<dma_tdtaddr::DMA_TDTADDR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor table address register"]
pub mod dma_tdtaddr;
#[doc = "DMA_STAT register accessor: an alias for `Reg<DMA_STAT_SPEC>`"]
pub type DMA_STAT = crate::Reg<dma_stat::DMA_STAT_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dma_stat;
#[doc = "DMA_CTL register accessor: an alias for `Reg<DMA_CTL_SPEC>`"]
pub type DMA_CTL = crate::Reg<dma_ctl::DMA_CTL_SPEC>;
#[doc = "Ethernet DMA control register"]
pub mod dma_ctl;
#[doc = "DMA_INTEN register accessor: an alias for `Reg<DMA_INTEN_SPEC>`"]
pub type DMA_INTEN = crate::Reg<dma_inten::DMA_INTEN_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dma_inten;
#[doc = "DMA_MFBOCNT register accessor: an alias for `Reg<DMA_MFBOCNT_SPEC>`"]
pub type DMA_MFBOCNT = crate::Reg<dma_mfbocnt::DMA_MFBOCNT_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dma_mfbocnt;
#[doc = "DMA_RSWDC register accessor: an alias for `Reg<DMA_RSWDC_SPEC>`"]
pub type DMA_RSWDC = crate::Reg<dma_rswdc::DMA_RSWDC_SPEC>;
#[doc = "Ethernet DMA receive state watchdog counter register"]
pub mod dma_rswdc;
#[doc = "DMA_CTDADDR register accessor: an alias for `Reg<DMA_CTDADDR_SPEC>`"]
pub type DMA_CTDADDR = crate::Reg<dma_ctdaddr::DMA_CTDADDR_SPEC>;
#[doc = "DMA current transmit descriptor address register"]
pub mod dma_ctdaddr;
#[doc = "DMA_CRDADDR register accessor: an alias for `Reg<DMA_CRDADDR_SPEC>`"]
pub type DMA_CRDADDR = crate::Reg<dma_crdaddr::DMA_CRDADDR_SPEC>;
#[doc = "Ethernet DMA current receive descriptor address register"]
pub mod dma_crdaddr;
#[doc = "DMA_CTBADDR register accessor: an alias for `Reg<DMA_CTBADDR_SPEC>`"]
pub type DMA_CTBADDR = crate::Reg<dma_ctbaddr::DMA_CTBADDR_SPEC>;
#[doc = "Ethernet DMA current transmit buffer address register"]
pub mod dma_ctbaddr;
#[doc = "DMA_CRBADDR register accessor: an alias for `Reg<DMA_CRBADDR_SPEC>`"]
pub type DMA_CRBADDR = crate::Reg<dma_crbaddr::DMA_CRBADDR_SPEC>;
#[doc = "Ethernet DMA current receive buffer address register"]
pub mod dma_crbaddr;
