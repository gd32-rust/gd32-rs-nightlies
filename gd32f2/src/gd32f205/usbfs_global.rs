#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global OTG control and status register (USBFS_GOTGCS)"]
    pub gotgcs: GOTGCS,
    #[doc = "0x04 - Global OTG interrupt flag register (USBFS_GOTGINTF)"]
    pub gotgintf: GOTGINTF,
    #[doc = "0x08 - Global AHB control and status register (USBFS_GAHBCS)"]
    pub gahbcs: GAHBCS,
    #[doc = "0x0c - Global USB control and status register (OTG_FS_GUSBCSR)"]
    pub gusbcs: GUSBCS,
    #[doc = "0x10 - Global reset control register (USBFS_GRSTCTL)"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - Global interrupt flag register (USBFS_GINTF)"]
    pub gintf: GINTF,
    #[doc = "0x18 - Global interrupt enable register (USBFS_GINTEN)"]
    pub ginten: GINTEN,
    _reserved_7_grstatr: [u8; 0x04],
    _reserved_8_grstatp: [u8; 0x04],
    #[doc = "0x24 - Global Receive FIFO size register (USBFS_GRFLEN)"]
    pub grflen: GRFLEN,
    _reserved_10_hnptflen: [u8; 0x04],
    #[doc = "0x2c - Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
    pub hnptfqstat: HNPTFQSTAT,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - Global core configuration register (USBFS_GCCFG)"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - core ID register"]
    pub cid: CID,
    _reserved14: [u8; 0xc0],
    #[doc = "0x100 - Host periodic transmit FIFO length register (HPTFLEN)"]
    pub hptflen: HPTFLEN,
    #[doc = "0x104 - device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
    pub diep1tflen: DIEP1TFLEN,
    #[doc = "0x108 - device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
    pub diep2tflen: DIEP2TFLEN,
    #[doc = "0x10c - device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
    pub diep3tflen: DIEP3TFLEN,
}
impl RegisterBlock {
    #[doc = "0x1c - Global Receive status read(Host mode)"]
    #[inline(always)]
    pub const fn grstatr_host(&self) -> &GRSTATR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Global Receive status read(Device mode)"]
    #[inline(always)]
    pub const fn grstatr_device(&self) -> &GRSTATR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x20 - Global Receive status pop(Host mode)"]
    #[inline(always)]
    pub const fn grstatp_host(&self) -> &GRSTATP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - Global Receive status pop(Device mode)"]
    #[inline(always)]
    pub const fn grstatp_device(&self) -> &GRSTATP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x28 - Device IN endpoint 0 transmit FIFO length (Device mode)"]
    #[inline(always)]
    pub const fn diep0tflen(&self) -> &DIEP0TFLEN {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Host non-periodic transmit FIFO length register (Host mode)"]
    #[inline(always)]
    pub const fn hnptflen(&self) -> &HNPTFLEN {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "GOTGCS (rw) register accessor: Global OTG control and status register (USBFS_GOTGCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgcs`]
module"]
pub type GOTGCS = crate::Reg<gotgcs::GOTGCS_SPEC>;
#[doc = "Global OTG control and status register (USBFS_GOTGCS)"]
pub mod gotgcs;
#[doc = "GOTGINTF (rw) register accessor: Global OTG interrupt flag register (USBFS_GOTGINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgintf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgintf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgintf`]
module"]
pub type GOTGINTF = crate::Reg<gotgintf::GOTGINTF_SPEC>;
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)"]
pub mod gotgintf;
#[doc = "GAHBCS (rw) register accessor: Global AHB control and status register (USBFS_GAHBCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gahbcs`]
module"]
pub type GAHBCS = crate::Reg<gahbcs::GAHBCS_SPEC>;
#[doc = "Global AHB control and status register (USBFS_GAHBCS)"]
pub mod gahbcs;
#[doc = "GUSBCS (rw) register accessor: Global USB control and status register (OTG_FS_GUSBCSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gusbcs`]
module"]
pub type GUSBCS = crate::Reg<gusbcs::GUSBCS_SPEC>;
#[doc = "Global USB control and status register (OTG_FS_GUSBCSR)"]
pub mod gusbcs;
#[doc = "GRSTCTL (rw) register accessor: Global reset control register (USBFS_GRSTCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Global reset control register (USBFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTF (rw) register accessor: Global interrupt flag register (USBFS_GINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gintf`]
module"]
pub type GINTF = crate::Reg<gintf::GINTF_SPEC>;
#[doc = "Global interrupt flag register (USBFS_GINTF)"]
pub mod gintf;
#[doc = "GINTEN (rw) register accessor: Global interrupt enable register (USBFS_GINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ginten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ginten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ginten`]
module"]
pub type GINTEN = crate::Reg<ginten::GINTEN_SPEC>;
#[doc = "Global interrupt enable register (USBFS_GINTEN)"]
pub mod ginten;
#[doc = "GRSTATR_Device (r) register accessor: Global Receive status read(Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstatr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstatr_device`]
module"]
pub type GRSTATR_DEVICE = crate::Reg<grstatr_device::GRSTATR_DEVICE_SPEC>;
#[doc = "Global Receive status read(Device mode)"]
pub mod grstatr_device;
#[doc = "GRSTATR_Host (r) register accessor: Global Receive status read(Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstatr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstatr_host`]
module"]
pub type GRSTATR_HOST = crate::Reg<grstatr_host::GRSTATR_HOST_SPEC>;
#[doc = "Global Receive status read(Host mode)"]
pub mod grstatr_host;
#[doc = "GRSTATP_Device (r) register accessor: Global Receive status pop(Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstatp_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstatp_device`]
module"]
pub type GRSTATP_DEVICE = crate::Reg<grstatp_device::GRSTATP_DEVICE_SPEC>;
#[doc = "Global Receive status pop(Device mode)"]
pub mod grstatp_device;
#[doc = "GRSTATP_Host (r) register accessor: Global Receive status pop(Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstatp_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstatp_host`]
module"]
pub type GRSTATP_HOST = crate::Reg<grstatp_host::GRSTATP_HOST_SPEC>;
#[doc = "Global Receive status pop(Host mode)"]
pub mod grstatp_host;
#[doc = "GRFLEN (rw) register accessor: Global Receive FIFO size register (USBFS_GRFLEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grflen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grflen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grflen`]
module"]
pub type GRFLEN = crate::Reg<grflen::GRFLEN_SPEC>;
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)"]
pub mod grflen;
#[doc = "HNPTFLEN (rw) register accessor: Host non-periodic transmit FIFO length register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptflen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hnptflen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hnptflen`]
module"]
pub type HNPTFLEN = crate::Reg<hnptflen::HNPTFLEN_SPEC>;
#[doc = "Host non-periodic transmit FIFO length register (Host mode)"]
pub mod hnptflen;
#[doc = "DIEP0TFLEN (rw) register accessor: Device IN endpoint 0 transmit FIFO length (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0tflen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0tflen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0tflen`]
module"]
pub type DIEP0TFLEN = crate::Reg<diep0tflen::DIEP0TFLEN_SPEC>;
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)"]
pub mod diep0tflen;
#[doc = "HNPTFQSTAT (r) register accessor: Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptfqstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hnptfqstat`]
module"]
pub type HNPTFQSTAT = crate::Reg<hnptfqstat::HNPTFQSTAT_SPEC>;
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
pub mod hnptfqstat;
#[doc = "GCCFG (rw) register accessor: Global core configuration register (USBFS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gccfg`]
module"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "Global core configuration register (USBFS_GCCFG)"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cid`]
module"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "core ID register"]
pub mod cid;
#[doc = "HPTFLEN (rw) register accessor: Host periodic transmit FIFO length register (HPTFLEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptflen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptflen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hptflen`]
module"]
pub type HPTFLEN = crate::Reg<hptflen::HPTFLEN_SPEC>;
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)"]
pub mod hptflen;
#[doc = "DIEP1TFLEN (rw) register accessor: device IN endpoint transmit FIFO size register (DIEP1TFLEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1tflen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1tflen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1tflen`]
module"]
pub type DIEP1TFLEN = crate::Reg<diep1tflen::DIEP1TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
pub mod diep1tflen;
#[doc = "DIEP2TFLEN (rw) register accessor: device IN endpoint transmit FIFO size register (DIEP2TFLEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2tflen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2tflen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2tflen`]
module"]
pub type DIEP2TFLEN = crate::Reg<diep2tflen::DIEP2TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
pub mod diep2tflen;
#[doc = "DIEP3TFLEN (rw) register accessor: device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3tflen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3tflen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3tflen`]
module"]
pub type DIEP3TFLEN = crate::Reg<diep3tflen::DIEP3TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
pub mod diep3tflen;
