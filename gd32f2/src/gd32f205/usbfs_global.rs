#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global OTG control and status register (USBFS_GOTGCS)"]
    pub gotgcs: crate::Reg<gotgcs::GOTGCS_SPEC>,
    #[doc = "0x04 - Global OTG interrupt flag register (USBFS_GOTGINTF)"]
    pub gotgintf: crate::Reg<gotgintf::GOTGINTF_SPEC>,
    #[doc = "0x08 - Global AHB control and status register (USBFS_GAHBCS)"]
    pub gahbcs: crate::Reg<gahbcs::GAHBCS_SPEC>,
    #[doc = "0x0c - Global USB control and status register (OTG_FS_GUSBCSR)"]
    pub gusbcs: crate::Reg<gusbcs::GUSBCS_SPEC>,
    #[doc = "0x10 - Global reset control register (USBFS_GRSTCTL)"]
    pub grstctl: crate::Reg<grstctl::GRSTCTL_SPEC>,
    #[doc = "0x14 - Global interrupt flag register (USBFS_GINTF)"]
    pub gintf: crate::Reg<gintf::GINTF_SPEC>,
    #[doc = "0x18 - Global interrupt enable register (USBFS_GINTEN)"]
    pub ginten: crate::Reg<ginten::GINTEN_SPEC>,
    _reserved_7_grstatr: [u8; 0x04],
    _reserved_8_grstatp: [u8; 0x04],
    #[doc = "0x24 - Global Receive FIFO size register (USBFS_GRFLEN)"]
    pub grflen: crate::Reg<grflen::GRFLEN_SPEC>,
    _reserved_10_hnptflen: [u8; 0x04],
    #[doc = "0x2c - Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
    pub hnptfqstat: crate::Reg<hnptfqstat::HNPTFQSTAT_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - Global core configuration register (USBFS_GCCFG)"]
    pub gccfg: crate::Reg<gccfg::GCCFG_SPEC>,
    #[doc = "0x3c - core ID register"]
    pub cid: crate::Reg<cid::CID_SPEC>,
    _reserved14: [u8; 0xc0],
    #[doc = "0x100 - Host periodic transmit FIFO length register (HPTFLEN)"]
    pub hptflen: crate::Reg<hptflen::HPTFLEN_SPEC>,
    #[doc = "0x104 - device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
    pub diep1tflen: crate::Reg<diep1tflen::DIEP1TFLEN_SPEC>,
    #[doc = "0x108 - device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
    pub diep2tflen: crate::Reg<diep2tflen::DIEP2TFLEN_SPEC>,
    #[doc = "0x10c - device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
    pub diep3tflen: crate::Reg<diep3tflen::DIEP3TFLEN_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x1c - Global Receive status read(Host mode)"]
    #[inline(always)]
    pub fn grstatr_host(&self) -> &crate::Reg<grstatr_host::GRSTATR_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<grstatr_host::GRSTATR_HOST_SPEC>)
        }
    }
    #[doc = "0x1c - Global Receive status read(Device mode)"]
    #[inline(always)]
    pub fn grstatr_device(&self) -> &crate::Reg<grstatr_device::GRSTATR_DEVICE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<grstatr_device::GRSTATR_DEVICE_SPEC>)
        }
    }
    #[doc = "0x20 - Global Receive status pop(Host mode)"]
    #[inline(always)]
    pub fn grstatp_host(&self) -> &crate::Reg<grstatp_host::GRSTATP_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<grstatp_host::GRSTATP_HOST_SPEC>)
        }
    }
    #[doc = "0x20 - Global Receive status pop(Device mode)"]
    #[inline(always)]
    pub fn grstatp_device(&self) -> &crate::Reg<grstatp_device::GRSTATP_DEVICE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<grstatp_device::GRSTATP_DEVICE_SPEC>)
        }
    }
    #[doc = "0x28 - Device IN endpoint 0 transmit FIFO length (Device mode)"]
    #[inline(always)]
    pub fn diep0tflen(&self) -> &crate::Reg<diep0tflen::DIEP0TFLEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<diep0tflen::DIEP0TFLEN_SPEC>)
        }
    }
    #[doc = "0x28 - Host non-periodic transmit FIFO length register (Host mode)"]
    #[inline(always)]
    pub fn hnptflen(&self) -> &crate::Reg<hnptflen::HNPTFLEN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<hnptflen::HNPTFLEN_SPEC>)
        }
    }
}
#[doc = "GOTGCS register accessor: an alias for `Reg<GOTGCS_SPEC>`"]
pub type GOTGCS = crate::Reg<gotgcs::GOTGCS_SPEC>;
#[doc = "Global OTG control and status register (USBFS_GOTGCS)"]
pub mod gotgcs;
#[doc = "GOTGINTF register accessor: an alias for `Reg<GOTGINTF_SPEC>`"]
pub type GOTGINTF = crate::Reg<gotgintf::GOTGINTF_SPEC>;
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)"]
pub mod gotgintf;
#[doc = "GAHBCS register accessor: an alias for `Reg<GAHBCS_SPEC>`"]
pub type GAHBCS = crate::Reg<gahbcs::GAHBCS_SPEC>;
#[doc = "Global AHB control and status register (USBFS_GAHBCS)"]
pub mod gahbcs;
#[doc = "GUSBCS register accessor: an alias for `Reg<GUSBCS_SPEC>`"]
pub type GUSBCS = crate::Reg<gusbcs::GUSBCS_SPEC>;
#[doc = "Global USB control and status register (OTG_FS_GUSBCSR)"]
pub mod gusbcs;
#[doc = "GRSTCTL register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Global reset control register (USBFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTF register accessor: an alias for `Reg<GINTF_SPEC>`"]
pub type GINTF = crate::Reg<gintf::GINTF_SPEC>;
#[doc = "Global interrupt flag register (USBFS_GINTF)"]
pub mod gintf;
#[doc = "GINTEN register accessor: an alias for `Reg<GINTEN_SPEC>`"]
pub type GINTEN = crate::Reg<ginten::GINTEN_SPEC>;
#[doc = "Global interrupt enable register (USBFS_GINTEN)"]
pub mod ginten;
#[doc = "GRSTATR_Device register accessor: an alias for `Reg<GRSTATR_DEVICE_SPEC>`"]
pub type GRSTATR_DEVICE = crate::Reg<grstatr_device::GRSTATR_DEVICE_SPEC>;
#[doc = "Global Receive status read(Device mode)"]
pub mod grstatr_device;
#[doc = "GRSTATR_Host register accessor: an alias for `Reg<GRSTATR_HOST_SPEC>`"]
pub type GRSTATR_HOST = crate::Reg<grstatr_host::GRSTATR_HOST_SPEC>;
#[doc = "Global Receive status read(Host mode)"]
pub mod grstatr_host;
#[doc = "GRSTATP_Device register accessor: an alias for `Reg<GRSTATP_DEVICE_SPEC>`"]
pub type GRSTATP_DEVICE = crate::Reg<grstatp_device::GRSTATP_DEVICE_SPEC>;
#[doc = "Global Receive status pop(Device mode)"]
pub mod grstatp_device;
#[doc = "GRSTATP_Host register accessor: an alias for `Reg<GRSTATP_HOST_SPEC>`"]
pub type GRSTATP_HOST = crate::Reg<grstatp_host::GRSTATP_HOST_SPEC>;
#[doc = "Global Receive status pop(Host mode)"]
pub mod grstatp_host;
#[doc = "GRFLEN register accessor: an alias for `Reg<GRFLEN_SPEC>`"]
pub type GRFLEN = crate::Reg<grflen::GRFLEN_SPEC>;
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)"]
pub mod grflen;
#[doc = "HNPTFLEN register accessor: an alias for `Reg<HNPTFLEN_SPEC>`"]
pub type HNPTFLEN = crate::Reg<hnptflen::HNPTFLEN_SPEC>;
#[doc = "Host non-periodic transmit FIFO length register (Host mode)"]
pub mod hnptflen;
#[doc = "DIEP0TFLEN register accessor: an alias for `Reg<DIEP0TFLEN_SPEC>`"]
pub type DIEP0TFLEN = crate::Reg<diep0tflen::DIEP0TFLEN_SPEC>;
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)"]
pub mod diep0tflen;
#[doc = "HNPTFQSTAT register accessor: an alias for `Reg<HNPTFQSTAT_SPEC>`"]
pub type HNPTFQSTAT = crate::Reg<hnptfqstat::HNPTFQSTAT_SPEC>;
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
pub mod hnptfqstat;
#[doc = "GCCFG register accessor: an alias for `Reg<GCCFG_SPEC>`"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "Global core configuration register (USBFS_GCCFG)"]
pub mod gccfg;
#[doc = "CID register accessor: an alias for `Reg<CID_SPEC>`"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "core ID register"]
pub mod cid;
#[doc = "HPTFLEN register accessor: an alias for `Reg<HPTFLEN_SPEC>`"]
pub type HPTFLEN = crate::Reg<hptflen::HPTFLEN_SPEC>;
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)"]
pub mod hptflen;
#[doc = "DIEP1TFLEN register accessor: an alias for `Reg<DIEP1TFLEN_SPEC>`"]
pub type DIEP1TFLEN = crate::Reg<diep1tflen::DIEP1TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
pub mod diep1tflen;
#[doc = "DIEP2TFLEN register accessor: an alias for `Reg<DIEP2TFLEN_SPEC>`"]
pub type DIEP2TFLEN = crate::Reg<diep2tflen::DIEP2TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
pub mod diep2tflen;
#[doc = "DIEP3TFLEN register accessor: an alias for `Reg<DIEP3TFLEN_SPEC>`"]
pub type DIEP3TFLEN = crate::Reg<diep3tflen::DIEP3TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
pub mod diep3tflen;
