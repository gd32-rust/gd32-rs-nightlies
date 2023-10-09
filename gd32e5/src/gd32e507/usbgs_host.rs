#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - host configuration register (HCTL)"]
    pub hctl: HCTL,
    #[doc = "0x04 - Host frame interval register"]
    pub hft: HFT,
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (HFINFR)"]
    pub hfinfr: HFINFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
    pub hptfqstat: HPTFQSTAT,
    #[doc = "0x14 - Host all channels interrupt register"]
    pub hachint: HACHINT,
    #[doc = "0x18 - host all channels interrupt mask register"]
    pub hachinten: HACHINTEN,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - Host port control and status register (USBFS_HPCS)"]
    pub hpcs: HPCS,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - host channel-0 characteristics register (HCH0CTL)"]
    pub hch0ctl: HCH0CTL,
    _reserved8: [u8; 0x04],
    #[doc = "0x108 - host channel-0 interrupt register (USBHS_HCHxINTF)"]
    pub hch0intf: HCH0INTF,
    #[doc = "0x10c - host channel-0 interrupt enable register (HCH0INTEN)"]
    pub hch0inten: HCH0INTEN,
    #[doc = "0x110 - host channel-0 transfer length register"]
    pub hch0len: HCH0LEN,
    #[doc = "0x114 - Host channel 0 DMA address register"]
    pub hch0dmaaddr: HCH0DMAADDR,
    _reserved12: [u8; 0x08],
    #[doc = "0x120 - host channel-1 characteristics register (HCH1CTL)"]
    pub hch1ctl: HCH1CTL,
    _reserved13: [u8; 0x04],
    #[doc = "0x128 - host channel-1 interrupt register (HCH1INTF)"]
    pub hch1intf: HCH1INTF,
    #[doc = "0x12c - host channel-1 interrupt enable register (HCH1INTEN)"]
    pub hch1inten: HCH1INTEN,
    #[doc = "0x130 - host channel-1 transfer length register"]
    pub hch1len: HCH1LEN,
    #[doc = "0x134 - Host channel 1 DMA address register"]
    pub hch1dmaaddr: HCH1DMAADDR,
    _reserved17: [u8; 0x08],
    #[doc = "0x140 - host channel-2 characteristics register (HCH2CTL)"]
    pub hch2ctl: HCH2CTL,
    _reserved18: [u8; 0x04],
    #[doc = "0x148 - host channel-2 interrupt register (HCH2INTF)"]
    pub hch2intf: HCH2INTF,
    #[doc = "0x14c - host channel-2 interrupt enable register (HCH2INTEN)"]
    pub hch2inten: HCH2INTEN,
    #[doc = "0x150 - host channel-2 transfer length register"]
    pub hch2len: HCH2LEN,
    #[doc = "0x154 - Host channel 2 DMA address register"]
    pub hch2dmaaddr: HCH2DMAADDR,
    _reserved22: [u8; 0x08],
    #[doc = "0x160 - host channel-3 characteristics register (HCH3CTL)"]
    pub hch3ctl: HCH3CTL,
    _reserved23: [u8; 0x04],
    #[doc = "0x168 - host channel-3 interrupt register (HCH3INTF)"]
    pub hch3intf: HCH3INTF,
    #[doc = "0x16c - host channel-3 interrupt enable register (HCH3INTEN)"]
    pub hch3inten: HCH3INTEN,
    #[doc = "0x170 - host channel-3 transfer length register"]
    pub hch3len: HCH3LEN,
    #[doc = "0x174 - Host channel 3 DMA address register"]
    pub hch3dmaaddr: HCH3DMAADDR,
    _reserved27: [u8; 0x08],
    #[doc = "0x180 - host channel-4 characteristics register (HCH4CTL)"]
    pub hch4ctl: HCH4CTL,
    _reserved28: [u8; 0x04],
    #[doc = "0x188 - host channel-4 interrupt register (HCH4INTF)"]
    pub hch4intf: HCH4INTF,
    #[doc = "0x18c - host channel-4 interrupt enable register (HCH4INTEN)"]
    pub hch4inten: HCH4INTEN,
    #[doc = "0x190 - host channel-4 transfer length register"]
    pub hch4len: HCH4LEN,
    #[doc = "0x194 - Host channel 4 DMA address register"]
    pub hch4dmaaddr: HCH4DMAADDR,
    _reserved32: [u8; 0x08],
    #[doc = "0x1a0 - host channel-5 characteristics register (HCH5CTL)"]
    pub hch5ctl: HCH5CTL,
    _reserved33: [u8; 0x04],
    #[doc = "0x1a8 - host channel-5 interrupt register (HCH5INTF)"]
    pub hch5intf: HCH5INTF,
    #[doc = "0x1ac - host channel-5 interrupt enable register (HCH5INTEN)"]
    pub hch5inten: HCH5INTEN,
    #[doc = "0x1b0 - host channel-5 transfer length register"]
    pub hch5len: HCH5LEN,
    #[doc = "0x1b4 - Host channel 5 DMA address register"]
    pub hch5dmaaddr: HCH5DMAADDR,
    _reserved37: [u8; 0x08],
    #[doc = "0x1c0 - host channel-6 characteristics register (HCH6CTL)"]
    pub hch6ctl: HCH6CTL,
    _reserved38: [u8; 0x04],
    #[doc = "0x1c8 - host channel-6 interrupt register (HCH6INTF)"]
    pub hch6intf: HCH6INTF,
    #[doc = "0x1cc - host channel-6 interrupt enable register (HCH6INTEN)"]
    pub hch6inten: HCH6INTEN,
    #[doc = "0x1d0 - host channel-6 transfer length register"]
    pub hch6len: HCH6LEN,
    #[doc = "0x1d4 - Host channel 6 DMA address register"]
    pub hch6dmaaddr: HCH6DMAADDR,
    _reserved42: [u8; 0x08],
    #[doc = "0x1e0 - host channel-7 characteristics register (HCH7CTL)"]
    pub hch7ctl: HCH7CTL,
    _reserved43: [u8; 0x04],
    #[doc = "0x1e8 - host channel-7 interrupt register (HCH7INTF)"]
    pub hch7intf: HCH7INTF,
    #[doc = "0x1ec - host channel-7 interrupt enable register (HCH7INTEN)"]
    pub hch7inten: HCH7INTEN,
    #[doc = "0x1f0 - host channel-7 transfer length register"]
    pub hch7len: HCH7LEN,
    #[doc = "0x1f4 - Host channel 7 DMA address register"]
    pub hch7dmaaddr: HCH7DMAADDR,
    _reserved47: [u8; 0x08],
    #[doc = "0x200 - host channel-8 characteristics register (HCH8CTL)"]
    pub hch8ctl: HCH8CTL,
    _reserved48: [u8; 0x04],
    #[doc = "0x208 - host channel-8 interrupt register (HCH8INTF)"]
    pub hch8intf: HCH8INTF,
    #[doc = "0x20c - host channel-8 interrupt enable register (HCH7INTEN)"]
    pub hch8inten: HCH8INTEN,
    #[doc = "0x210 - host channel-8 transfer length register"]
    pub hch8len: HCH8LEN,
    #[doc = "0x214 - Host channel 8 DMA address register"]
    pub hch8dmaaddr: HCH8DMAADDR,
    _reserved52: [u8; 0x08],
    #[doc = "0x220 - host channel-9 characteristics register (HCH9CTL)"]
    pub hch9ctl: HCH9CTL,
    _reserved53: [u8; 0x04],
    #[doc = "0x228 - host channel-9 interrupt register (HCH9INTF)"]
    pub hch9intf: HCH9INTF,
    #[doc = "0x22c - host channel-9 interrupt enable register (HCH9INTEN)"]
    pub hch9inten: HCH9INTEN,
    #[doc = "0x230 - host channel-9 transfer length register"]
    pub hch9len: HCH9LEN,
    #[doc = "0x234 - Host channel 9 DMA address register"]
    pub hch9dmaaddr: HCH9DMAADDR,
    _reserved57: [u8; 0x08],
    #[doc = "0x240 - host channel-10 characteristics register (HCH10CTL)"]
    pub hch10ctl: HCH10CTL,
    _reserved58: [u8; 0x04],
    #[doc = "0x248 - host channel-10 interrupt register (HCH10INTF)"]
    pub hch10intf: HCH10INTF,
    #[doc = "0x24c - host channel-10 interrupt enable register (HCH10INTEN)"]
    pub hch10inten: HCH10INTEN,
    #[doc = "0x250 - host channel-10 transfer length register"]
    pub hch10len: HCH10LEN,
    #[doc = "0x254 - Host channel 10 DMA address register"]
    pub hch10dmaaddr: HCH10DMAADDR,
    _reserved62: [u8; 0x08],
    #[doc = "0x260 - host channel-11 characteristics register (HCH11CTL)"]
    pub hch11ctl: HCH11CTL,
    _reserved63: [u8; 0x04],
    #[doc = "0x268 - host channel-11 interrupt register (HCH11INTF)"]
    pub hch11intf: HCH11INTF,
    #[doc = "0x26c - host channel-11 interrupt enable register (HCH11INTEN)"]
    pub hch11inten: HCH11INTEN,
    #[doc = "0x270 - host channel-11 transfer length register"]
    pub hch11len: HCH11LEN,
    #[doc = "0x274 - Host channel 11 DMA address register"]
    pub hch11dmaaddr: HCH11DMAADDR,
}
#[doc = "HCTL (rw) register accessor: host configuration register (HCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hctl`]
module"]
pub type HCTL = crate::Reg<hctl::HCTL_SPEC>;
#[doc = "host configuration register (HCTL)"]
pub mod hctl;
#[doc = "HFT (rw) register accessor: Host frame interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hft::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hft::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hft`]
module"]
pub type HFT = crate::Reg<hft::HFT_SPEC>;
#[doc = "Host frame interval register"]
pub mod hft;
#[doc = "HFINFR (r) register accessor: OTG_FS host frame number/frame time remaining register (HFINFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfinfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hfinfr`]
module"]
pub type HFINFR = crate::Reg<hfinfr::HFINFR_SPEC>;
#[doc = "OTG_FS host frame number/frame time remaining register (HFINFR)"]
pub mod hfinfr;
#[doc = "HPTFQSTAT (r) register accessor: Host periodic transmit FIFO/queue status register (HPTFQSTAT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptfqstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hptfqstat`]
module"]
pub type HPTFQSTAT = crate::Reg<hptfqstat::HPTFQSTAT_SPEC>;
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
pub mod hptfqstat;
#[doc = "HACHINT (r) register accessor: Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hachint`]
module"]
pub type HACHINT = crate::Reg<hachint::HACHINT_SPEC>;
#[doc = "Host all channels interrupt register"]
pub mod hachint;
#[doc = "HACHINTEN (rw) register accessor: host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hachinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hachinten`]
module"]
pub type HACHINTEN = crate::Reg<hachinten::HACHINTEN_SPEC>;
#[doc = "host all channels interrupt mask register"]
pub mod hachinten;
#[doc = "HPCS (rw) register accessor: Host port control and status register (USBFS_HPCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hpcs`]
module"]
pub type HPCS = crate::Reg<hpcs::HPCS_SPEC>;
#[doc = "Host port control and status register (USBFS_HPCS)"]
pub mod hpcs;
#[doc = "HCH0CTL (rw) register accessor: host channel-0 characteristics register (HCH0CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch0ctl`]
module"]
pub type HCH0CTL = crate::Reg<hch0ctl::HCH0CTL_SPEC>;
#[doc = "host channel-0 characteristics register (HCH0CTL)"]
pub mod hch0ctl;
#[doc = "HCH1CTL (rw) register accessor: host channel-1 characteristics register (HCH1CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch1ctl`]
module"]
pub type HCH1CTL = crate::Reg<hch1ctl::HCH1CTL_SPEC>;
#[doc = "host channel-1 characteristics register (HCH1CTL)"]
pub mod hch1ctl;
#[doc = "HCH2CTL (rw) register accessor: host channel-2 characteristics register (HCH2CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch2ctl`]
module"]
pub type HCH2CTL = crate::Reg<hch2ctl::HCH2CTL_SPEC>;
#[doc = "host channel-2 characteristics register (HCH2CTL)"]
pub mod hch2ctl;
#[doc = "HCH3CTL (rw) register accessor: host channel-3 characteristics register (HCH3CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch3ctl`]
module"]
pub type HCH3CTL = crate::Reg<hch3ctl::HCH3CTL_SPEC>;
#[doc = "host channel-3 characteristics register (HCH3CTL)"]
pub mod hch3ctl;
#[doc = "HCH4CTL (rw) register accessor: host channel-4 characteristics register (HCH4CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch4ctl`]
module"]
pub type HCH4CTL = crate::Reg<hch4ctl::HCH4CTL_SPEC>;
#[doc = "host channel-4 characteristics register (HCH4CTL)"]
pub mod hch4ctl;
#[doc = "HCH5CTL (rw) register accessor: host channel-5 characteristics register (HCH5CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch5ctl`]
module"]
pub type HCH5CTL = crate::Reg<hch5ctl::HCH5CTL_SPEC>;
#[doc = "host channel-5 characteristics register (HCH5CTL)"]
pub mod hch5ctl;
#[doc = "HCH6CTL (rw) register accessor: host channel-6 characteristics register (HCH6CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch6ctl`]
module"]
pub type HCH6CTL = crate::Reg<hch6ctl::HCH6CTL_SPEC>;
#[doc = "host channel-6 characteristics register (HCH6CTL)"]
pub mod hch6ctl;
#[doc = "HCH7CTL (rw) register accessor: host channel-7 characteristics register (HCH7CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch7ctl`]
module"]
pub type HCH7CTL = crate::Reg<hch7ctl::HCH7CTL_SPEC>;
#[doc = "host channel-7 characteristics register (HCH7CTL)"]
pub mod hch7ctl;
#[doc = "HCH8CTL (rw) register accessor: host channel-8 characteristics register (HCH8CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch8ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch8ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch8ctl`]
module"]
pub type HCH8CTL = crate::Reg<hch8ctl::HCH8CTL_SPEC>;
#[doc = "host channel-8 characteristics register (HCH8CTL)"]
pub mod hch8ctl;
#[doc = "HCH9CTL (rw) register accessor: host channel-9 characteristics register (HCH9CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch9ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch9ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch9ctl`]
module"]
pub type HCH9CTL = crate::Reg<hch9ctl::HCH9CTL_SPEC>;
#[doc = "host channel-9 characteristics register (HCH9CTL)"]
pub mod hch9ctl;
#[doc = "HCH10CTL (rw) register accessor: host channel-10 characteristics register (HCH10CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch10ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch10ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch10ctl`]
module"]
pub type HCH10CTL = crate::Reg<hch10ctl::HCH10CTL_SPEC>;
#[doc = "host channel-10 characteristics register (HCH10CTL)"]
pub mod hch10ctl;
#[doc = "HCH11CTL (rw) register accessor: host channel-11 characteristics register (HCH11CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch11ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch11ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch11ctl`]
module"]
pub type HCH11CTL = crate::Reg<hch11ctl::HCH11CTL_SPEC>;
#[doc = "host channel-11 characteristics register (HCH11CTL)"]
pub mod hch11ctl;
#[doc = "HCH0INTF (rw) register accessor: host channel-0 interrupt register (USBHS_HCHxINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch0intf`]
module"]
pub type HCH0INTF = crate::Reg<hch0intf::HCH0INTF_SPEC>;
#[doc = "host channel-0 interrupt register (USBHS_HCHxINTF)"]
pub mod hch0intf;
#[doc = "HCH1INTF (rw) register accessor: host channel-1 interrupt register (HCH1INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch1intf`]
module"]
pub type HCH1INTF = crate::Reg<hch1intf::HCH1INTF_SPEC>;
#[doc = "host channel-1 interrupt register (HCH1INTF)"]
pub mod hch1intf;
#[doc = "HCH2INTF (rw) register accessor: host channel-2 interrupt register (HCH2INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch2intf`]
module"]
pub type HCH2INTF = crate::Reg<hch2intf::HCH2INTF_SPEC>;
#[doc = "host channel-2 interrupt register (HCH2INTF)"]
pub mod hch2intf;
#[doc = "HCH3INTF (rw) register accessor: host channel-3 interrupt register (HCH3INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch3intf`]
module"]
pub type HCH3INTF = crate::Reg<hch3intf::HCH3INTF_SPEC>;
#[doc = "host channel-3 interrupt register (HCH3INTF)"]
pub mod hch3intf;
#[doc = "HCH4INTF (rw) register accessor: host channel-4 interrupt register (HCH4INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch4intf`]
module"]
pub type HCH4INTF = crate::Reg<hch4intf::HCH4INTF_SPEC>;
#[doc = "host channel-4 interrupt register (HCH4INTF)"]
pub mod hch4intf;
#[doc = "HCH5INTF (rw) register accessor: host channel-5 interrupt register (HCH5INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch5intf`]
module"]
pub type HCH5INTF = crate::Reg<hch5intf::HCH5INTF_SPEC>;
#[doc = "host channel-5 interrupt register (HCH5INTF)"]
pub mod hch5intf;
#[doc = "HCH6INTF (rw) register accessor: host channel-6 interrupt register (HCH6INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch6intf`]
module"]
pub type HCH6INTF = crate::Reg<hch6intf::HCH6INTF_SPEC>;
#[doc = "host channel-6 interrupt register (HCH6INTF)"]
pub mod hch6intf;
#[doc = "HCH7INTF (rw) register accessor: host channel-7 interrupt register (HCH7INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch7intf`]
module"]
pub type HCH7INTF = crate::Reg<hch7intf::HCH7INTF_SPEC>;
#[doc = "host channel-7 interrupt register (HCH7INTF)"]
pub mod hch7intf;
#[doc = "HCH8INTF (rw) register accessor: host channel-8 interrupt register (HCH8INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch8intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch8intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch8intf`]
module"]
pub type HCH8INTF = crate::Reg<hch8intf::HCH8INTF_SPEC>;
#[doc = "host channel-8 interrupt register (HCH8INTF)"]
pub mod hch8intf;
#[doc = "HCH9INTF (rw) register accessor: host channel-9 interrupt register (HCH9INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch9intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch9intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch9intf`]
module"]
pub type HCH9INTF = crate::Reg<hch9intf::HCH9INTF_SPEC>;
#[doc = "host channel-9 interrupt register (HCH9INTF)"]
pub mod hch9intf;
#[doc = "HCH10INTF (rw) register accessor: host channel-10 interrupt register (HCH10INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch10intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch10intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch10intf`]
module"]
pub type HCH10INTF = crate::Reg<hch10intf::HCH10INTF_SPEC>;
#[doc = "host channel-10 interrupt register (HCH10INTF)"]
pub mod hch10intf;
#[doc = "HCH11INTF (rw) register accessor: host channel-11 interrupt register (HCH11INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch11intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch11intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch11intf`]
module"]
pub type HCH11INTF = crate::Reg<hch11intf::HCH11INTF_SPEC>;
#[doc = "host channel-11 interrupt register (HCH11INTF)"]
pub mod hch11intf;
#[doc = "HCH0INTEN (rw) register accessor: host channel-0 interrupt enable register (HCH0INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch0inten`]
module"]
pub type HCH0INTEN = crate::Reg<hch0inten::HCH0INTEN_SPEC>;
#[doc = "host channel-0 interrupt enable register (HCH0INTEN)"]
pub mod hch0inten;
#[doc = "HCH1INTEN (rw) register accessor: host channel-1 interrupt enable register (HCH1INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch1inten`]
module"]
pub type HCH1INTEN = crate::Reg<hch1inten::HCH1INTEN_SPEC>;
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)"]
pub mod hch1inten;
#[doc = "HCH2INTEN (rw) register accessor: host channel-2 interrupt enable register (HCH2INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch2inten`]
module"]
pub type HCH2INTEN = crate::Reg<hch2inten::HCH2INTEN_SPEC>;
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)"]
pub mod hch2inten;
#[doc = "HCH3INTEN (rw) register accessor: host channel-3 interrupt enable register (HCH3INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch3inten`]
module"]
pub type HCH3INTEN = crate::Reg<hch3inten::HCH3INTEN_SPEC>;
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)"]
pub mod hch3inten;
#[doc = "HCH4INTEN (rw) register accessor: host channel-4 interrupt enable register (HCH4INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch4inten`]
module"]
pub type HCH4INTEN = crate::Reg<hch4inten::HCH4INTEN_SPEC>;
#[doc = "host channel-4 interrupt enable register (HCH4INTEN)"]
pub mod hch4inten;
#[doc = "HCH5INTEN (rw) register accessor: host channel-5 interrupt enable register (HCH5INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch5inten`]
module"]
pub type HCH5INTEN = crate::Reg<hch5inten::HCH5INTEN_SPEC>;
#[doc = "host channel-5 interrupt enable register (HCH5INTEN)"]
pub mod hch5inten;
#[doc = "HCH6INTEN (rw) register accessor: host channel-6 interrupt enable register (HCH6INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch6inten`]
module"]
pub type HCH6INTEN = crate::Reg<hch6inten::HCH6INTEN_SPEC>;
#[doc = "host channel-6 interrupt enable register (HCH6INTEN)"]
pub mod hch6inten;
#[doc = "HCH7INTEN (rw) register accessor: host channel-7 interrupt enable register (HCH7INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch7inten`]
module"]
pub type HCH7INTEN = crate::Reg<hch7inten::HCH7INTEN_SPEC>;
#[doc = "host channel-7 interrupt enable register (HCH7INTEN)"]
pub mod hch7inten;
#[doc = "HCH8INTEN (rw) register accessor: host channel-8 interrupt enable register (HCH7INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch8inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch8inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch8inten`]
module"]
pub type HCH8INTEN = crate::Reg<hch8inten::HCH8INTEN_SPEC>;
#[doc = "host channel-8 interrupt enable register (HCH7INTEN)"]
pub mod hch8inten;
#[doc = "HCH9INTEN (rw) register accessor: host channel-9 interrupt enable register (HCH9INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch9inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch9inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch9inten`]
module"]
pub type HCH9INTEN = crate::Reg<hch9inten::HCH9INTEN_SPEC>;
#[doc = "host channel-9 interrupt enable register (HCH9INTEN)"]
pub mod hch9inten;
#[doc = "HCH10INTEN (rw) register accessor: host channel-10 interrupt enable register (HCH10INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch10inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch10inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch10inten`]
module"]
pub type HCH10INTEN = crate::Reg<hch10inten::HCH10INTEN_SPEC>;
#[doc = "host channel-10 interrupt enable register (HCH10INTEN)"]
pub mod hch10inten;
#[doc = "HCH11INTEN (rw) register accessor: host channel-11 interrupt enable register (HCH11INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch11inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch11inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch11inten`]
module"]
pub type HCH11INTEN = crate::Reg<hch11inten::HCH11INTEN_SPEC>;
#[doc = "host channel-11 interrupt enable register (HCH11INTEN)"]
pub mod hch11inten;
#[doc = "HCH0LEN (rw) register accessor: host channel-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch0len`]
module"]
pub type HCH0LEN = crate::Reg<hch0len::HCH0LEN_SPEC>;
#[doc = "host channel-0 transfer length register"]
pub mod hch0len;
#[doc = "HCH1LEN (rw) register accessor: host channel-1 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch1len`]
module"]
pub type HCH1LEN = crate::Reg<hch1len::HCH1LEN_SPEC>;
#[doc = "host channel-1 transfer length register"]
pub mod hch1len;
#[doc = "HCH2LEN (rw) register accessor: host channel-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch2len`]
module"]
pub type HCH2LEN = crate::Reg<hch2len::HCH2LEN_SPEC>;
#[doc = "host channel-2 transfer length register"]
pub mod hch2len;
#[doc = "HCH3LEN (rw) register accessor: host channel-3 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch3len`]
module"]
pub type HCH3LEN = crate::Reg<hch3len::HCH3LEN_SPEC>;
#[doc = "host channel-3 transfer length register"]
pub mod hch3len;
#[doc = "HCH4LEN (rw) register accessor: host channel-4 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch4len`]
module"]
pub type HCH4LEN = crate::Reg<hch4len::HCH4LEN_SPEC>;
#[doc = "host channel-4 transfer length register"]
pub mod hch4len;
#[doc = "HCH5LEN (rw) register accessor: host channel-5 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch5len`]
module"]
pub type HCH5LEN = crate::Reg<hch5len::HCH5LEN_SPEC>;
#[doc = "host channel-5 transfer length register"]
pub mod hch5len;
#[doc = "HCH6LEN (rw) register accessor: host channel-6 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch6len`]
module"]
pub type HCH6LEN = crate::Reg<hch6len::HCH6LEN_SPEC>;
#[doc = "host channel-6 transfer length register"]
pub mod hch6len;
#[doc = "HCH7LEN (rw) register accessor: host channel-7 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch7len`]
module"]
pub type HCH7LEN = crate::Reg<hch7len::HCH7LEN_SPEC>;
#[doc = "host channel-7 transfer length register"]
pub mod hch7len;
#[doc = "HCH8LEN (rw) register accessor: host channel-8 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch8len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch8len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch8len`]
module"]
pub type HCH8LEN = crate::Reg<hch8len::HCH8LEN_SPEC>;
#[doc = "host channel-8 transfer length register"]
pub mod hch8len;
#[doc = "HCH9LEN (rw) register accessor: host channel-9 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch9len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch9len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch9len`]
module"]
pub type HCH9LEN = crate::Reg<hch9len::HCH9LEN_SPEC>;
#[doc = "host channel-9 transfer length register"]
pub mod hch9len;
#[doc = "HCH10LEN (rw) register accessor: host channel-10 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch10len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch10len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch10len`]
module"]
pub type HCH10LEN = crate::Reg<hch10len::HCH10LEN_SPEC>;
#[doc = "host channel-10 transfer length register"]
pub mod hch10len;
#[doc = "HCH11LEN (rw) register accessor: host channel-11 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch11len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch11len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch11len`]
module"]
pub type HCH11LEN = crate::Reg<hch11len::HCH11LEN_SPEC>;
#[doc = "host channel-11 transfer length register"]
pub mod hch11len;
#[doc = "HCH0DMAADDR (rw) register accessor: Host channel 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch0dmaaddr`]
module"]
pub type HCH0DMAADDR = crate::Reg<hch0dmaaddr::HCH0DMAADDR_SPEC>;
#[doc = "Host channel 0 DMA address register"]
pub mod hch0dmaaddr;
#[doc = "HCH1DMAADDR (rw) register accessor: Host channel 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch1dmaaddr`]
module"]
pub type HCH1DMAADDR = crate::Reg<hch1dmaaddr::HCH1DMAADDR_SPEC>;
#[doc = "Host channel 1 DMA address register"]
pub mod hch1dmaaddr;
#[doc = "HCH2DMAADDR (rw) register accessor: Host channel 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch2dmaaddr`]
module"]
pub type HCH2DMAADDR = crate::Reg<hch2dmaaddr::HCH2DMAADDR_SPEC>;
#[doc = "Host channel 2 DMA address register"]
pub mod hch2dmaaddr;
#[doc = "HCH3DMAADDR (rw) register accessor: Host channel 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch3dmaaddr`]
module"]
pub type HCH3DMAADDR = crate::Reg<hch3dmaaddr::HCH3DMAADDR_SPEC>;
#[doc = "Host channel 3 DMA address register"]
pub mod hch3dmaaddr;
#[doc = "HCH4DMAADDR (rw) register accessor: Host channel 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch4dmaaddr`]
module"]
pub type HCH4DMAADDR = crate::Reg<hch4dmaaddr::HCH4DMAADDR_SPEC>;
#[doc = "Host channel 4 DMA address register"]
pub mod hch4dmaaddr;
#[doc = "HCH5DMAADDR (rw) register accessor: Host channel 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch5dmaaddr`]
module"]
pub type HCH5DMAADDR = crate::Reg<hch5dmaaddr::HCH5DMAADDR_SPEC>;
#[doc = "Host channel 5 DMA address register"]
pub mod hch5dmaaddr;
#[doc = "HCH6DMAADDR (rw) register accessor: Host channel 6 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch6dmaaddr`]
module"]
pub type HCH6DMAADDR = crate::Reg<hch6dmaaddr::HCH6DMAADDR_SPEC>;
#[doc = "Host channel 6 DMA address register"]
pub mod hch6dmaaddr;
#[doc = "HCH7DMAADDR (rw) register accessor: Host channel 7 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch7dmaaddr`]
module"]
pub type HCH7DMAADDR = crate::Reg<hch7dmaaddr::HCH7DMAADDR_SPEC>;
#[doc = "Host channel 7 DMA address register"]
pub mod hch7dmaaddr;
#[doc = "HCH8DMAADDR (rw) register accessor: Host channel 8 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch8dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch8dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch8dmaaddr`]
module"]
pub type HCH8DMAADDR = crate::Reg<hch8dmaaddr::HCH8DMAADDR_SPEC>;
#[doc = "Host channel 8 DMA address register"]
pub mod hch8dmaaddr;
#[doc = "HCH9DMAADDR (rw) register accessor: Host channel 9 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch9dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch9dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch9dmaaddr`]
module"]
pub type HCH9DMAADDR = crate::Reg<hch9dmaaddr::HCH9DMAADDR_SPEC>;
#[doc = "Host channel 9 DMA address register"]
pub mod hch9dmaaddr;
#[doc = "HCH10DMAADDR (rw) register accessor: Host channel 10 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch10dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch10dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch10dmaaddr`]
module"]
pub type HCH10DMAADDR = crate::Reg<hch10dmaaddr::HCH10DMAADDR_SPEC>;
#[doc = "Host channel 10 DMA address register"]
pub mod hch10dmaaddr;
#[doc = "HCH11DMAADDR (rw) register accessor: Host channel 11 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch11dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch11dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hch11dmaaddr`]
module"]
pub type HCH11DMAADDR = crate::Reg<hch11dmaaddr::HCH11DMAADDR_SPEC>;
#[doc = "Host channel 11 DMA address register"]
pub mod hch11dmaaddr;
