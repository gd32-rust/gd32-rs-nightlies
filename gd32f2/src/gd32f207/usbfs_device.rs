#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - device configuration register (DCFG)"]
    pub dcfg: crate::Reg<dcfg::DCFG_SPEC>,
    #[doc = "0x04 - device control register (DCTL)"]
    pub dctl: crate::Reg<dctl::DCTL_SPEC>,
    #[doc = "0x08 - device status register (DSTAT)"]
    pub dstat: crate::Reg<dstat::DSTAT_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - device IN endpoint common interrupt mask register (DIEPINTEN)"]
    pub diepinten: crate::Reg<diepinten::DIEPINTEN_SPEC>,
    #[doc = "0x14 - device OUT endpoint common interrupt enable register (DOEPINTEN)"]
    pub doepinten: crate::Reg<doepinten::DOEPINTEN_SPEC>,
    #[doc = "0x18 - device all endpoints interrupt register (DAEPINT)"]
    pub daepint: crate::Reg<daepint::DAEPINT_SPEC>,
    #[doc = "0x1c - Device all endpoints interrupt enable register (DAEPINTEN)"]
    pub daepinten: crate::Reg<daepinten::DAEPINTEN_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - device VBUS discharge time register"]
    pub dvbusdt: crate::Reg<dvbusdt::DVBUSDT_SPEC>,
    #[doc = "0x2c - device VBUS pulsing time register"]
    pub dvbuspt: crate::Reg<dvbuspt::DVBUSPT_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - device IN endpoint FIFO empty interrupt enable register"]
    pub diepfeinten: crate::Reg<diepfeinten::DIEPFEINTEN_SPEC>,
    _reserved10: [u8; 0xc8],
    #[doc = "0x100 - device IN endpoint 0 control register (DIEP0CTL)"]
    pub diep0ctl: crate::Reg<diep0ctl::DIEP0CTL_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x108 - device endpoint-0 interrupt register"]
    pub diep0intf: crate::Reg<diep0intf::DIEP0INTF_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x110 - device IN endpoint-0 transfer length register"]
    pub diep0len: crate::Reg<diep0len::DIEP0LEN_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x118 - device IN endpoint 0 transmit FIFO status register"]
    pub diep0tfstat: crate::Reg<diep0tfstat::DIEP0TFSTAT_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x120 - device in endpoint-1 control register"]
    pub diep1ctl: crate::Reg<diep1ctl::DIEP1CTL_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diep1intf: crate::Reg<diep1intf::DIEP1INTF_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x130 - device IN endpoint-1 transfer length register"]
    pub diep1len: crate::Reg<diep1len::DIEP1LEN_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x138 - device IN endpoint 1 transmit FIFO status register"]
    pub diep1tfstat: crate::Reg<diep1tfstat::DIEP1TFSTAT_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x140 - device endpoint-2 control register"]
    pub diep2ctl: crate::Reg<diep2ctl::DIEP2CTL_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub diep2intf: crate::Reg<diep2intf::DIEP2INTF_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x150 - device IN endpoint-2 transfer length register"]
    pub diep2len: crate::Reg<diep2len::DIEP2LEN_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x158 - device IN endpoint 2 transmit FIFO status register"]
    pub diep2tfstat: crate::Reg<diep2tfstat::DIEP2TFSTAT_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x160 - device endpoint-3 control register"]
    pub diep3ctl: crate::Reg<diep3ctl::DIEP3CTL_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub diep3intf: crate::Reg<diep3intf::DIEP3INTF_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x170 - device IN endpoint-3 transfer length register"]
    pub diep3len: crate::Reg<diep3len::DIEP3LEN_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x178 - device IN endpoint 3 transmit FIFO status register"]
    pub diep3tfstat: crate::Reg<diep3tfstat::DIEP3TFSTAT_SPEC>,
    _reserved26: [u8; 0x0184],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doep0ctl: crate::Reg<doep0ctl::DOEP0CTL_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x308 - device out endpoint-0 interrupt flag register"]
    pub doep0intf: crate::Reg<doep0intf::DOEP0INTF_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x310 - device OUT endpoint-0 transfer length register"]
    pub doep0len: crate::Reg<doep0len::DOEP0LEN_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doep1ctl: crate::Reg<doep1ctl::DOEP1CTL_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x328 - device out endpoint-1 interrupt flag register"]
    pub doep1intf: crate::Reg<doep1intf::DOEP1INTF_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x330 - device OUT endpoint-1 transfer length register"]
    pub doep1len: crate::Reg<doep1len::DOEP1LEN_SPEC>,
    _reserved32: [u8; 0x0c],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub doep2ctl: crate::Reg<doep2ctl::DOEP2CTL_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x348 - device out endpoint-2 interrupt flag register"]
    pub doep2intf: crate::Reg<doep2intf::DOEP2INTF_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x350 - device OUT endpoint-2 transfer length register"]
    pub doep2len: crate::Reg<doep2len::DOEP2LEN_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub doep3ctl: crate::Reg<doep3ctl::DOEP3CTL_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x368 - device out endpoint-3 interrupt flag register"]
    pub doep3intf: crate::Reg<doep3intf::DOEP3INTF_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0x370 - device OUT endpoint-3 transfer length register"]
    pub doep3len: crate::Reg<doep3len::DOEP3LEN_SPEC>,
}
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "device configuration register (DCFG)"]
pub mod dcfg;
#[doc = "DCTL register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "device control register (DCTL)"]
pub mod dctl;
#[doc = "DSTAT register accessor: an alias for `Reg<DSTAT_SPEC>`"]
pub type DSTAT = crate::Reg<dstat::DSTAT_SPEC>;
#[doc = "device status register (DSTAT)"]
pub mod dstat;
#[doc = "DIEPINTEN register accessor: an alias for `Reg<DIEPINTEN_SPEC>`"]
pub type DIEPINTEN = crate::Reg<diepinten::DIEPINTEN_SPEC>;
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)"]
pub mod diepinten;
#[doc = "DOEPINTEN register accessor: an alias for `Reg<DOEPINTEN_SPEC>`"]
pub type DOEPINTEN = crate::Reg<doepinten::DOEPINTEN_SPEC>;
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)"]
pub mod doepinten;
#[doc = "DAEPINT register accessor: an alias for `Reg<DAEPINT_SPEC>`"]
pub type DAEPINT = crate::Reg<daepint::DAEPINT_SPEC>;
#[doc = "device all endpoints interrupt register (DAEPINT)"]
pub mod daepint;
#[doc = "DAEPINTEN register accessor: an alias for `Reg<DAEPINTEN_SPEC>`"]
pub type DAEPINTEN = crate::Reg<daepinten::DAEPINTEN_SPEC>;
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)"]
pub mod daepinten;
#[doc = "DVBUSDT register accessor: an alias for `Reg<DVBUSDT_SPEC>`"]
pub type DVBUSDT = crate::Reg<dvbusdt::DVBUSDT_SPEC>;
#[doc = "device VBUS discharge time register"]
pub mod dvbusdt;
#[doc = "DVBUSPT register accessor: an alias for `Reg<DVBUSPT_SPEC>`"]
pub type DVBUSPT = crate::Reg<dvbuspt::DVBUSPT_SPEC>;
#[doc = "device VBUS pulsing time register"]
pub mod dvbuspt;
#[doc = "DIEPFEINTEN register accessor: an alias for `Reg<DIEPFEINTEN_SPEC>`"]
pub type DIEPFEINTEN = crate::Reg<diepfeinten::DIEPFEINTEN_SPEC>;
#[doc = "device IN endpoint FIFO empty interrupt enable register"]
pub mod diepfeinten;
#[doc = "DIEP0CTL register accessor: an alias for `Reg<DIEP0CTL_SPEC>`"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "device IN endpoint 0 control register (DIEP0CTL)"]
pub mod diep0ctl;
#[doc = "DIEP1CTL register accessor: an alias for `Reg<DIEP1CTL_SPEC>`"]
pub type DIEP1CTL = crate::Reg<diep1ctl::DIEP1CTL_SPEC>;
#[doc = "device in endpoint-1 control register"]
pub mod diep1ctl;
#[doc = "DIEP2CTL register accessor: an alias for `Reg<DIEP2CTL_SPEC>`"]
pub type DIEP2CTL = crate::Reg<diep2ctl::DIEP2CTL_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod diep2ctl;
#[doc = "DIEP3CTL register accessor: an alias for `Reg<DIEP3CTL_SPEC>`"]
pub type DIEP3CTL = crate::Reg<diep3ctl::DIEP3CTL_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod diep3ctl;
#[doc = "DOEP0CTL register accessor: an alias for `Reg<DOEP0CTL_SPEC>`"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "device endpoint-0 control register"]
pub mod doep0ctl;
#[doc = "DOEP1CTL register accessor: an alias for `Reg<DOEP1CTL_SPEC>`"]
pub type DOEP1CTL = crate::Reg<doep1ctl::DOEP1CTL_SPEC>;
#[doc = "device endpoint-1 control register"]
pub mod doep1ctl;
#[doc = "DOEP2CTL register accessor: an alias for `Reg<DOEP2CTL_SPEC>`"]
pub type DOEP2CTL = crate::Reg<doep2ctl::DOEP2CTL_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod doep2ctl;
#[doc = "DOEP3CTL register accessor: an alias for `Reg<DOEP3CTL_SPEC>`"]
pub type DOEP3CTL = crate::Reg<doep3ctl::DOEP3CTL_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod doep3ctl;
#[doc = "DIEP0INTF register accessor: an alias for `Reg<DIEP0INTF_SPEC>`"]
pub type DIEP0INTF = crate::Reg<diep0intf::DIEP0INTF_SPEC>;
#[doc = "device endpoint-0 interrupt register"]
pub mod diep0intf;
#[doc = "DIEP1INTF register accessor: an alias for `Reg<DIEP1INTF_SPEC>`"]
pub type DIEP1INTF = crate::Reg<diep1intf::DIEP1INTF_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod diep1intf;
#[doc = "DIEP2INTF register accessor: an alias for `Reg<DIEP2INTF_SPEC>`"]
pub type DIEP2INTF = crate::Reg<diep2intf::DIEP2INTF_SPEC>;
#[doc = "device endpoint-2 interrupt register"]
pub mod diep2intf;
#[doc = "DIEP3INTF register accessor: an alias for `Reg<DIEP3INTF_SPEC>`"]
pub type DIEP3INTF = crate::Reg<diep3intf::DIEP3INTF_SPEC>;
#[doc = "device endpoint-3 interrupt register"]
pub mod diep3intf;
#[doc = "DOEP0INTF register accessor: an alias for `Reg<DOEP0INTF_SPEC>`"]
pub type DOEP0INTF = crate::Reg<doep0intf::DOEP0INTF_SPEC>;
#[doc = "device out endpoint-0 interrupt flag register"]
pub mod doep0intf;
#[doc = "DOEP1INTF register accessor: an alias for `Reg<DOEP1INTF_SPEC>`"]
pub type DOEP1INTF = crate::Reg<doep1intf::DOEP1INTF_SPEC>;
#[doc = "device out endpoint-1 interrupt flag register"]
pub mod doep1intf;
#[doc = "DOEP2INTF register accessor: an alias for `Reg<DOEP2INTF_SPEC>`"]
pub type DOEP2INTF = crate::Reg<doep2intf::DOEP2INTF_SPEC>;
#[doc = "device out endpoint-2 interrupt flag register"]
pub mod doep2intf;
#[doc = "DOEP3INTF register accessor: an alias for `Reg<DOEP3INTF_SPEC>`"]
pub type DOEP3INTF = crate::Reg<doep3intf::DOEP3INTF_SPEC>;
#[doc = "device out endpoint-3 interrupt flag register"]
pub mod doep3intf;
#[doc = "DIEP0LEN register accessor: an alias for `Reg<DIEP0LEN_SPEC>`"]
pub type DIEP0LEN = crate::Reg<diep0len::DIEP0LEN_SPEC>;
#[doc = "device IN endpoint-0 transfer length register"]
pub mod diep0len;
#[doc = "DOEP0LEN register accessor: an alias for `Reg<DOEP0LEN_SPEC>`"]
pub type DOEP0LEN = crate::Reg<doep0len::DOEP0LEN_SPEC>;
#[doc = "device OUT endpoint-0 transfer length register"]
pub mod doep0len;
#[doc = "DIEP1LEN register accessor: an alias for `Reg<DIEP1LEN_SPEC>`"]
pub type DIEP1LEN = crate::Reg<diep1len::DIEP1LEN_SPEC>;
#[doc = "device IN endpoint-1 transfer length register"]
pub mod diep1len;
#[doc = "DIEP2LEN register accessor: an alias for `Reg<DIEP2LEN_SPEC>`"]
pub type DIEP2LEN = crate::Reg<diep2len::DIEP2LEN_SPEC>;
#[doc = "device IN endpoint-2 transfer length register"]
pub mod diep2len;
#[doc = "DIEP3LEN register accessor: an alias for `Reg<DIEP3LEN_SPEC>`"]
pub type DIEP3LEN = crate::Reg<diep3len::DIEP3LEN_SPEC>;
#[doc = "device IN endpoint-3 transfer length register"]
pub mod diep3len;
#[doc = "DOEP1LEN register accessor: an alias for `Reg<DOEP1LEN_SPEC>`"]
pub type DOEP1LEN = crate::Reg<doep1len::DOEP1LEN_SPEC>;
#[doc = "device OUT endpoint-1 transfer length register"]
pub mod doep1len;
#[doc = "DOEP2LEN register accessor: an alias for `Reg<DOEP2LEN_SPEC>`"]
pub type DOEP2LEN = crate::Reg<doep2len::DOEP2LEN_SPEC>;
#[doc = "device OUT endpoint-2 transfer length register"]
pub mod doep2len;
#[doc = "DOEP3LEN register accessor: an alias for `Reg<DOEP3LEN_SPEC>`"]
pub type DOEP3LEN = crate::Reg<doep3len::DOEP3LEN_SPEC>;
#[doc = "device OUT endpoint-3 transfer length register"]
pub mod doep3len;
#[doc = "DIEP0TFSTAT register accessor: an alias for `Reg<DIEP0TFSTAT_SPEC>`"]
pub type DIEP0TFSTAT = crate::Reg<diep0tfstat::DIEP0TFSTAT_SPEC>;
#[doc = "device IN endpoint 0 transmit FIFO status register"]
pub mod diep0tfstat;
#[doc = "DIEP1TFSTAT register accessor: an alias for `Reg<DIEP1TFSTAT_SPEC>`"]
pub type DIEP1TFSTAT = crate::Reg<diep1tfstat::DIEP1TFSTAT_SPEC>;
#[doc = "device IN endpoint 1 transmit FIFO status register"]
pub mod diep1tfstat;
#[doc = "DIEP2TFSTAT register accessor: an alias for `Reg<DIEP2TFSTAT_SPEC>`"]
pub type DIEP2TFSTAT = crate::Reg<diep2tfstat::DIEP2TFSTAT_SPEC>;
#[doc = "device IN endpoint 2 transmit FIFO status register"]
pub mod diep2tfstat;
#[doc = "DIEP3TFSTAT register accessor: an alias for `Reg<DIEP3TFSTAT_SPEC>`"]
pub type DIEP3TFSTAT = crate::Reg<diep3tfstat::DIEP3TFSTAT_SPEC>;
#[doc = "device IN endpoint 3 transmit FIFO status register"]
pub mod diep3tfstat;
