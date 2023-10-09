#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - device configuration register (DCFG)"]
    pub dcfg: DCFG,
    #[doc = "0x04 - device control register (DCTL)"]
    pub dctl: DCTL,
    #[doc = "0x08 - device status register (DSTAT)"]
    pub dstat: DSTAT,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - device IN endpoint common interrupt mask register (DIEPINTEN)"]
    pub diepinten: DIEPINTEN,
    #[doc = "0x14 - device OUT endpoint common interrupt enable register (DOEPINTEN)"]
    pub doepinten: DOEPINTEN,
    #[doc = "0x18 - device all endpoints interrupt register (DAEPINT)"]
    pub daepint: DAEPINT,
    #[doc = "0x1c - Device all endpoints interrupt enable register (DAEPINTEN)"]
    pub daepinten: DAEPINTEN,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - device VBUS discharge time register"]
    pub dvbusdt: DVBUSDT,
    #[doc = "0x2c - device VBUS pulsing time register"]
    pub dvbuspt: DVBUSPT,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - device IN endpoint FIFO empty interrupt enable register"]
    pub diepfeinten: DIEPFEINTEN,
    #[doc = "0x38 - Device endpoint 1 interrupt register"]
    pub dep1int: DEP1INT,
    #[doc = "0x3c - Device endpoint 1 interrupt register"]
    pub dep1inten: DEP1INTEN,
    _reserved12: [u8; 0x04],
    #[doc = "0x44 - Device IN endpoint 1 interrupt enable register"]
    pub diep1inten: DIEP1INTEN,
    _reserved13: [u8; 0x3c],
    #[doc = "0x84 - Device OUT endpoint 1 interrupt enable register"]
    pub doep1inten: DOEP1INTEN,
    _reserved14: [u8; 0x78],
    #[doc = "0x100 - device IN endpoint 0 control register (DIEP0CTL)"]
    pub diep0ctl: DIEP0CTL,
    _reserved15: [u8; 0x04],
    #[doc = "0x108 - device endpoint-0 interrupt register"]
    pub diep0intf: DIEP0INTF,
    _reserved16: [u8; 0x04],
    #[doc = "0x110 - device IN endpoint-0 transfer length register"]
    pub diep0len: DIEP0LEN,
    #[doc = "0x114 - Device IN endpoint 0 DMA address register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0x118 - device IN endpoint 0 transmit FIFO status register"]
    pub diep0tfstat: DIEP0TFSTAT,
    _reserved19: [u8; 0x04],
    #[doc = "0x120 - device in endpoint-1 control register"]
    pub diep1ctl: DIEP1CTL,
    _reserved20: [u8; 0x04],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diep1intf: DIEP1INTF,
    _reserved21: [u8; 0x04],
    #[doc = "0x130 - device IN endpoint-1 transfer length register"]
    pub diep1len: DIEP1LEN,
    #[doc = "0x134 - Device IN endpoint 1 DMA address register"]
    pub diep1dmaaddr: DIEP1DMAADDR,
    #[doc = "0x138 - device IN endpoint 1 transmit FIFO status register"]
    pub diep1tfstat: DIEP1TFSTAT,
    _reserved24: [u8; 0x04],
    #[doc = "0x140 - device endpoint-2 control register"]
    pub diep2ctl: DIEP2CTL,
    _reserved25: [u8; 0x04],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub diep2intf: DIEP2INTF,
    _reserved26: [u8; 0x04],
    #[doc = "0x150 - device IN endpoint-2 transfer length register"]
    pub diep2len: DIEP2LEN,
    #[doc = "0x154 - Device IN endpoint 2 DMA address register"]
    pub diep2dmaaddr: DIEP2DMAADDR,
    #[doc = "0x158 - device IN endpoint 2 transmit FIFO status register"]
    pub diep2tfstat: DIEP2TFSTAT,
    _reserved29: [u8; 0x04],
    #[doc = "0x160 - device endpoint-3 control register"]
    pub diep3ctl: DIEP3CTL,
    _reserved30: [u8; 0x04],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub diep3intf: DIEP3INTF,
    _reserved31: [u8; 0x04],
    #[doc = "0x170 - device IN endpoint-3 transfer length register"]
    pub diep3len: DIEP3LEN,
    #[doc = "0x174 - Device IN endpoint 3 DMA address register"]
    pub diep3dmaaddr: DIEP3DMAADDR,
    #[doc = "0x178 - device IN endpoint 3 transmit FIFO status register"]
    pub diep3tfstat: DIEP3TFSTAT,
    _reserved34: [u8; 0x04],
    #[doc = "0x180 - device endpoint-4 control register"]
    pub diep4ctl: DIEP4CTL,
    _reserved35: [u8; 0x04],
    #[doc = "0x188 - device endpoint-4 interrupt register"]
    pub diep4intf: DIEP4INTF,
    _reserved36: [u8; 0x04],
    #[doc = "0x190 - device IN endpoint-4 transfer length register"]
    pub diep4len: DIEP4LEN,
    #[doc = "0x194 - Device IN endpoint 4 DMA address register"]
    pub diep4dmaaddr: DIEP4DMAADDR,
    #[doc = "0x198 - device IN endpoint 4 transmit FIFO status register"]
    pub diep4tfstat: DIEP4TFSTAT,
    _reserved39: [u8; 0x04],
    #[doc = "0x1a0 - device endpoint-5 control register"]
    pub diep5ctl: DIEP5CTL,
    _reserved40: [u8; 0x04],
    #[doc = "0x1a8 - device endpoint-5 interrupt register"]
    pub diep5intf: DIEP5INTF,
    _reserved41: [u8; 0x04],
    #[doc = "0x1b0 - device IN endpoint-5 transfer length register"]
    pub diep5len: DIEP5LEN,
    #[doc = "0x1b4 - Device IN endpoint 5 DMA address register"]
    pub diep5dmaaddr: DIEP5DMAADDR,
    #[doc = "0x1b8 - device IN endpoint 5 transmit FIFO status register"]
    pub diep5tfstat: DIEP5TFSTAT,
    _reserved44: [u8; 0x0144],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doep0ctl: DOEP0CTL,
    _reserved45: [u8; 0x04],
    #[doc = "0x308 - device out endpoint-0 interrupt flag register"]
    pub doep0intf: DOEP0INTF,
    _reserved46: [u8; 0x04],
    #[doc = "0x310 - device OUT endpoint-0 transfer length register"]
    pub doep0len: DOEP0LEN,
    #[doc = "0x314 - Device OUT endpoint 0 DMA address register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved48: [u8; 0x08],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doep1ctl: DOEP1CTL,
    _reserved49: [u8; 0x04],
    #[doc = "0x328 - device out endpoint-1 interrupt flag register"]
    pub doep1intf: DOEP1INTF,
    _reserved50: [u8; 0x04],
    #[doc = "0x330 - device OUT endpoint-1 transfer length register"]
    pub doep1len: DOEP1LEN,
    #[doc = "0x334 - Device OUT endpoint 1 DMA address register"]
    pub doep1dmaaddr: DOEP1DMAADDR,
    _reserved52: [u8; 0x08],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub doep2ctl: DOEP2CTL,
    _reserved53: [u8; 0x04],
    #[doc = "0x348 - device out endpoint-2 interrupt flag register"]
    pub doep2intf: DOEP2INTF,
    _reserved54: [u8; 0x04],
    #[doc = "0x350 - device OUT endpoint-2 transfer length register"]
    pub doep2len: DOEP2LEN,
    #[doc = "0x354 - Device OUT endpoint 2 DMA address register"]
    pub doep2dmaaddr: DOEP2DMAADDR,
    _reserved56: [u8; 0x08],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub doep3ctl: DOEP3CTL,
    _reserved57: [u8; 0x04],
    #[doc = "0x368 - device out endpoint-3 interrupt flag register"]
    pub doep3intf: DOEP3INTF,
    _reserved58: [u8; 0x04],
    #[doc = "0x370 - device OUT endpoint-3 transfer length register"]
    pub doep3len: DOEP3LEN,
    #[doc = "0x374 - Device OUT endpoint 3 DMA address register"]
    pub doep3dmaaddr: DOEP3DMAADDR,
    _reserved60: [u8; 0x08],
    #[doc = "0x380 - device endpoint-4 control register"]
    pub doep4ctl: DOEP4CTL,
    _reserved61: [u8; 0x04],
    #[doc = "0x388 - device out endpoint-4 interrupt flag register"]
    pub doep4intf: DOEP4INTF,
    _reserved62: [u8; 0x04],
    #[doc = "0x390 - device OUT endpoint-4 transfer length register"]
    pub doep4len: DOEP4LEN,
    #[doc = "0x394 - Device OUT endpoint 4 DMA address register"]
    pub doep4dmaaddr: DOEP4DMAADDR,
    _reserved64: [u8; 0x08],
    #[doc = "0x3a0 - device endpoint-5 control register"]
    pub doep5ctl: DOEP5CTL,
    _reserved65: [u8; 0x04],
    #[doc = "0x3a8 - device out endpoint-5 interrupt flag register"]
    pub doep5intf: DOEP5INTF,
    _reserved66: [u8; 0x04],
    #[doc = "0x3b0 - device OUT endpoint-5 transfer length register"]
    pub doep5len: DOEP5LEN,
    #[doc = "0x3b4 - Device OUT endpoint 5 DMA address register"]
    pub doep5dmaaddr: DOEP5DMAADDR,
}
#[doc = "DCFG (rw) register accessor: device configuration register (DCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "device configuration register (DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: device control register (DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "device control register (DCTL)"]
pub mod dctl;
#[doc = "DSTAT (r) register accessor: device status register (DSTAT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dstat`]
module"]
pub type DSTAT = crate::Reg<dstat::DSTAT_SPEC>;
#[doc = "device status register (DSTAT)"]
pub mod dstat;
#[doc = "DIEPINTEN (rw) register accessor: device IN endpoint common interrupt mask register (DIEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepinten`]
module"]
pub type DIEPINTEN = crate::Reg<diepinten::DIEPINTEN_SPEC>;
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)"]
pub mod diepinten;
#[doc = "DOEPINTEN (rw) register accessor: device OUT endpoint common interrupt enable register (DOEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepinten`]
module"]
pub type DOEPINTEN = crate::Reg<doepinten::DOEPINTEN_SPEC>;
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)"]
pub mod doepinten;
#[doc = "DAEPINT (r) register accessor: device all endpoints interrupt register (DAEPINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daepint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daepint`]
module"]
pub type DAEPINT = crate::Reg<daepint::DAEPINT_SPEC>;
#[doc = "device all endpoints interrupt register (DAEPINT)"]
pub mod daepint;
#[doc = "DAEPINTEN (rw) register accessor: Device all endpoints interrupt enable register (DAEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daepinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daepinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daepinten`]
module"]
pub type DAEPINTEN = crate::Reg<daepinten::DAEPINTEN_SPEC>;
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)"]
pub mod daepinten;
#[doc = "DVBUSDT (rw) register accessor: device VBUS discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dvbusdt`]
module"]
pub type DVBUSDT = crate::Reg<dvbusdt::DVBUSDT_SPEC>;
#[doc = "device VBUS discharge time register"]
pub mod dvbusdt;
#[doc = "DVBUSPT (rw) register accessor: device VBUS pulsing time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dvbuspt`]
module"]
pub type DVBUSPT = crate::Reg<dvbuspt::DVBUSPT_SPEC>;
#[doc = "device VBUS pulsing time register"]
pub mod dvbuspt;
#[doc = "DIEPFEINTEN (rw) register accessor: device IN endpoint FIFO empty interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepfeinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepfeinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepfeinten`]
module"]
pub type DIEPFEINTEN = crate::Reg<diepfeinten::DIEPFEINTEN_SPEC>;
#[doc = "device IN endpoint FIFO empty interrupt enable register"]
pub mod diepfeinten;
#[doc = "DEP1INT (r) register accessor: Device endpoint 1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dep1int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dep1int`]
module"]
pub type DEP1INT = crate::Reg<dep1int::DEP1INT_SPEC>;
#[doc = "Device endpoint 1 interrupt register"]
pub mod dep1int;
#[doc = "DEP1INTEN (r) register accessor: Device endpoint 1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dep1inten::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dep1inten`]
module"]
pub type DEP1INTEN = crate::Reg<dep1inten::DEP1INTEN_SPEC>;
#[doc = "Device endpoint 1 interrupt register"]
pub mod dep1inten;
#[doc = "DIEP1INTEN (rw) register accessor: Device IN endpoint 1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1inten`]
module"]
pub type DIEP1INTEN = crate::Reg<diep1inten::DIEP1INTEN_SPEC>;
#[doc = "Device IN endpoint 1 interrupt enable register"]
pub mod diep1inten;
#[doc = "DOEP1INTEN (rw) register accessor: Device OUT endpoint 1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1inten`]
module"]
pub type DOEP1INTEN = crate::Reg<doep1inten::DOEP1INTEN_SPEC>;
#[doc = "Device OUT endpoint 1 interrupt enable register"]
pub mod doep1inten;
#[doc = "DIEP0CTL (rw) register accessor: device IN endpoint 0 control register (DIEP0CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0ctl`]
module"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "device IN endpoint 0 control register (DIEP0CTL)"]
pub mod diep0ctl;
#[doc = "DIEP1CTL (rw) register accessor: device in endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1ctl`]
module"]
pub type DIEP1CTL = crate::Reg<diep1ctl::DIEP1CTL_SPEC>;
#[doc = "device in endpoint-1 control register"]
pub mod diep1ctl;
#[doc = "DIEP2CTL (rw) register accessor: device endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2ctl`]
module"]
pub type DIEP2CTL = crate::Reg<diep2ctl::DIEP2CTL_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod diep2ctl;
#[doc = "DIEP3CTL (rw) register accessor: device endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3ctl`]
module"]
pub type DIEP3CTL = crate::Reg<diep3ctl::DIEP3CTL_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod diep3ctl;
#[doc = "DIEP4CTL (rw) register accessor: device endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4ctl`]
module"]
pub type DIEP4CTL = crate::Reg<diep4ctl::DIEP4CTL_SPEC>;
#[doc = "device endpoint-4 control register"]
pub mod diep4ctl;
#[doc = "DIEP5CTL (rw) register accessor: device endpoint-5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5ctl`]
module"]
pub type DIEP5CTL = crate::Reg<diep5ctl::DIEP5CTL_SPEC>;
#[doc = "device endpoint-5 control register"]
pub mod diep5ctl;
#[doc = "DOEP0CTL (rw) register accessor: device endpoint-0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0ctl`]
module"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "device endpoint-0 control register"]
pub mod doep0ctl;
#[doc = "DOEP1CTL (rw) register accessor: device endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1ctl`]
module"]
pub type DOEP1CTL = crate::Reg<doep1ctl::DOEP1CTL_SPEC>;
#[doc = "device endpoint-1 control register"]
pub mod doep1ctl;
#[doc = "DOEP2CTL (rw) register accessor: device endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2ctl`]
module"]
pub type DOEP2CTL = crate::Reg<doep2ctl::DOEP2CTL_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod doep2ctl;
#[doc = "DOEP3CTL (rw) register accessor: device endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3ctl`]
module"]
pub type DOEP3CTL = crate::Reg<doep3ctl::DOEP3CTL_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod doep3ctl;
#[doc = "DOEP4CTL (rw) register accessor: device endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4ctl`]
module"]
pub type DOEP4CTL = crate::Reg<doep4ctl::DOEP4CTL_SPEC>;
#[doc = "device endpoint-4 control register"]
pub mod doep4ctl;
#[doc = "DOEP5CTL (rw) register accessor: device endpoint-5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5ctl`]
module"]
pub type DOEP5CTL = crate::Reg<doep5ctl::DOEP5CTL_SPEC>;
#[doc = "device endpoint-5 control register"]
pub mod doep5ctl;
#[doc = "DIEP0INTF (rw) register accessor: device endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0intf`]
module"]
pub type DIEP0INTF = crate::Reg<diep0intf::DIEP0INTF_SPEC>;
#[doc = "device endpoint-0 interrupt register"]
pub mod diep0intf;
#[doc = "DIEP1INTF (rw) register accessor: device endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1intf`]
module"]
pub type DIEP1INTF = crate::Reg<diep1intf::DIEP1INTF_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod diep1intf;
#[doc = "DIEP2INTF (rw) register accessor: device endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2intf`]
module"]
pub type DIEP2INTF = crate::Reg<diep2intf::DIEP2INTF_SPEC>;
#[doc = "device endpoint-2 interrupt register"]
pub mod diep2intf;
#[doc = "DIEP3INTF (rw) register accessor: device endpoint-3 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3intf`]
module"]
pub type DIEP3INTF = crate::Reg<diep3intf::DIEP3INTF_SPEC>;
#[doc = "device endpoint-3 interrupt register"]
pub mod diep3intf;
#[doc = "DIEP4INTF (rw) register accessor: device endpoint-4 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4intf`]
module"]
pub type DIEP4INTF = crate::Reg<diep4intf::DIEP4INTF_SPEC>;
#[doc = "device endpoint-4 interrupt register"]
pub mod diep4intf;
#[doc = "DIEP5INTF (rw) register accessor: device endpoint-5 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5intf`]
module"]
pub type DIEP5INTF = crate::Reg<diep5intf::DIEP5INTF_SPEC>;
#[doc = "device endpoint-5 interrupt register"]
pub mod diep5intf;
#[doc = "DOEP0INTF (rw) register accessor: device out endpoint-0 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0intf`]
module"]
pub type DOEP0INTF = crate::Reg<doep0intf::DOEP0INTF_SPEC>;
#[doc = "device out endpoint-0 interrupt flag register"]
pub mod doep0intf;
#[doc = "DOEP1INTF (rw) register accessor: device out endpoint-1 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1intf`]
module"]
pub type DOEP1INTF = crate::Reg<doep1intf::DOEP1INTF_SPEC>;
#[doc = "device out endpoint-1 interrupt flag register"]
pub mod doep1intf;
#[doc = "DOEP2INTF (rw) register accessor: device out endpoint-2 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2intf`]
module"]
pub type DOEP2INTF = crate::Reg<doep2intf::DOEP2INTF_SPEC>;
#[doc = "device out endpoint-2 interrupt flag register"]
pub mod doep2intf;
#[doc = "DOEP3INTF (rw) register accessor: device out endpoint-3 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3intf`]
module"]
pub type DOEP3INTF = crate::Reg<doep3intf::DOEP3INTF_SPEC>;
#[doc = "device out endpoint-3 interrupt flag register"]
pub mod doep3intf;
#[doc = "DOEP4INTF (rw) register accessor: device out endpoint-4 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4intf`]
module"]
pub type DOEP4INTF = crate::Reg<doep4intf::DOEP4INTF_SPEC>;
#[doc = "device out endpoint-4 interrupt flag register"]
pub mod doep4intf;
#[doc = "DOEP5INTF (rw) register accessor: device out endpoint-5 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5intf`]
module"]
pub type DOEP5INTF = crate::Reg<doep5intf::DOEP5INTF_SPEC>;
#[doc = "device out endpoint-5 interrupt flag register"]
pub mod doep5intf;
#[doc = "DIEP0LEN (rw) register accessor: device IN endpoint-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0len`]
module"]
pub type DIEP0LEN = crate::Reg<diep0len::DIEP0LEN_SPEC>;
#[doc = "device IN endpoint-0 transfer length register"]
pub mod diep0len;
#[doc = "DOEP0LEN (rw) register accessor: device OUT endpoint-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0len`]
module"]
pub type DOEP0LEN = crate::Reg<doep0len::DOEP0LEN_SPEC>;
#[doc = "device OUT endpoint-0 transfer length register"]
pub mod doep0len;
#[doc = "DIEP1LEN (rw) register accessor: device IN endpoint-1 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1len`]
module"]
pub type DIEP1LEN = crate::Reg<diep1len::DIEP1LEN_SPEC>;
#[doc = "device IN endpoint-1 transfer length register"]
pub mod diep1len;
#[doc = "DIEP2LEN (rw) register accessor: device IN endpoint-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2len`]
module"]
pub type DIEP2LEN = crate::Reg<diep2len::DIEP2LEN_SPEC>;
#[doc = "device IN endpoint-2 transfer length register"]
pub mod diep2len;
#[doc = "DIEP3LEN (rw) register accessor: device IN endpoint-3 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3len`]
module"]
pub type DIEP3LEN = crate::Reg<diep3len::DIEP3LEN_SPEC>;
#[doc = "device IN endpoint-3 transfer length register"]
pub mod diep3len;
#[doc = "DIEP4LEN (rw) register accessor: device IN endpoint-4 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4len`]
module"]
pub type DIEP4LEN = crate::Reg<diep4len::DIEP4LEN_SPEC>;
#[doc = "device IN endpoint-4 transfer length register"]
pub mod diep4len;
#[doc = "DIEP5LEN (rw) register accessor: device IN endpoint-5 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5len`]
module"]
pub type DIEP5LEN = crate::Reg<diep5len::DIEP5LEN_SPEC>;
#[doc = "device IN endpoint-5 transfer length register"]
pub mod diep5len;
#[doc = "DOEP1LEN (rw) register accessor: device OUT endpoint-1 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1len`]
module"]
pub type DOEP1LEN = crate::Reg<doep1len::DOEP1LEN_SPEC>;
#[doc = "device OUT endpoint-1 transfer length register"]
pub mod doep1len;
#[doc = "DOEP2LEN (rw) register accessor: device OUT endpoint-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2len`]
module"]
pub type DOEP2LEN = crate::Reg<doep2len::DOEP2LEN_SPEC>;
#[doc = "device OUT endpoint-2 transfer length register"]
pub mod doep2len;
#[doc = "DOEP3LEN (rw) register accessor: device OUT endpoint-3 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3len`]
module"]
pub type DOEP3LEN = crate::Reg<doep3len::DOEP3LEN_SPEC>;
#[doc = "device OUT endpoint-3 transfer length register"]
pub mod doep3len;
#[doc = "DOEP4LEN (rw) register accessor: device OUT endpoint-4 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4len`]
module"]
pub type DOEP4LEN = crate::Reg<doep4len::DOEP4LEN_SPEC>;
#[doc = "device OUT endpoint-4 transfer length register"]
pub mod doep4len;
#[doc = "DOEP5LEN (rw) register accessor: device OUT endpoint-5 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5len`]
module"]
pub type DOEP5LEN = crate::Reg<doep5len::DOEP5LEN_SPEC>;
#[doc = "device OUT endpoint-5 transfer length register"]
pub mod doep5len;
#[doc = "DIEP0DMAADDR (rw) register accessor: Device IN endpoint 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0dmaaddr`]
module"]
pub type DIEP0DMAADDR = crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>;
#[doc = "Device IN endpoint 0 DMA address register"]
pub mod diep0dmaaddr;
#[doc = "DIEP1DMAADDR (rw) register accessor: Device IN endpoint 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1dmaaddr`]
module"]
pub type DIEP1DMAADDR = crate::Reg<diep1dmaaddr::DIEP1DMAADDR_SPEC>;
#[doc = "Device IN endpoint 1 DMA address register"]
pub mod diep1dmaaddr;
#[doc = "DIEP2DMAADDR (rw) register accessor: Device IN endpoint 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2dmaaddr`]
module"]
pub type DIEP2DMAADDR = crate::Reg<diep2dmaaddr::DIEP2DMAADDR_SPEC>;
#[doc = "Device IN endpoint 2 DMA address register"]
pub mod diep2dmaaddr;
#[doc = "DIEP3DMAADDR (rw) register accessor: Device IN endpoint 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3dmaaddr`]
module"]
pub type DIEP3DMAADDR = crate::Reg<diep3dmaaddr::DIEP3DMAADDR_SPEC>;
#[doc = "Device IN endpoint 3 DMA address register"]
pub mod diep3dmaaddr;
#[doc = "DIEP4DMAADDR (rw) register accessor: Device IN endpoint 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4dmaaddr`]
module"]
pub type DIEP4DMAADDR = crate::Reg<diep4dmaaddr::DIEP4DMAADDR_SPEC>;
#[doc = "Device IN endpoint 4 DMA address register"]
pub mod diep4dmaaddr;
#[doc = "DIEP5DMAADDR (rw) register accessor: Device IN endpoint 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5dmaaddr`]
module"]
pub type DIEP5DMAADDR = crate::Reg<diep5dmaaddr::DIEP5DMAADDR_SPEC>;
#[doc = "Device IN endpoint 5 DMA address register"]
pub mod diep5dmaaddr;
#[doc = "DOEP0DMAADDR (rw) register accessor: Device OUT endpoint 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep0dmaaddr`]
module"]
pub type DOEP0DMAADDR = crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>;
#[doc = "Device OUT endpoint 0 DMA address register"]
pub mod doep0dmaaddr;
#[doc = "DOEP1DMAADDR (rw) register accessor: Device OUT endpoint 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep1dmaaddr`]
module"]
pub type DOEP1DMAADDR = crate::Reg<doep1dmaaddr::DOEP1DMAADDR_SPEC>;
#[doc = "Device OUT endpoint 1 DMA address register"]
pub mod doep1dmaaddr;
#[doc = "DOEP2DMAADDR (rw) register accessor: Device OUT endpoint 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep2dmaaddr`]
module"]
pub type DOEP2DMAADDR = crate::Reg<doep2dmaaddr::DOEP2DMAADDR_SPEC>;
#[doc = "Device OUT endpoint 2 DMA address register"]
pub mod doep2dmaaddr;
#[doc = "DOEP3DMAADDR (rw) register accessor: Device OUT endpoint 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep3dmaaddr`]
module"]
pub type DOEP3DMAADDR = crate::Reg<doep3dmaaddr::DOEP3DMAADDR_SPEC>;
#[doc = "Device OUT endpoint 3 DMA address register"]
pub mod doep3dmaaddr;
#[doc = "DOEP4DMAADDR (rw) register accessor: Device OUT endpoint 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep4dmaaddr`]
module"]
pub type DOEP4DMAADDR = crate::Reg<doep4dmaaddr::DOEP4DMAADDR_SPEC>;
#[doc = "Device OUT endpoint 4 DMA address register"]
pub mod doep4dmaaddr;
#[doc = "DOEP5DMAADDR (rw) register accessor: Device OUT endpoint 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doep5dmaaddr`]
module"]
pub type DOEP5DMAADDR = crate::Reg<doep5dmaaddr::DOEP5DMAADDR_SPEC>;
#[doc = "Device OUT endpoint 5 DMA address register"]
pub mod doep5dmaaddr;
#[doc = "DIEP0TFSTAT (r) register accessor: device IN endpoint 0 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep0tfstat`]
module"]
pub type DIEP0TFSTAT = crate::Reg<diep0tfstat::DIEP0TFSTAT_SPEC>;
#[doc = "device IN endpoint 0 transmit FIFO status register"]
pub mod diep0tfstat;
#[doc = "DIEP1TFSTAT (r) register accessor: device IN endpoint 1 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep1tfstat`]
module"]
pub type DIEP1TFSTAT = crate::Reg<diep1tfstat::DIEP1TFSTAT_SPEC>;
#[doc = "device IN endpoint 1 transmit FIFO status register"]
pub mod diep1tfstat;
#[doc = "DIEP2TFSTAT (r) register accessor: device IN endpoint 2 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep2tfstat`]
module"]
pub type DIEP2TFSTAT = crate::Reg<diep2tfstat::DIEP2TFSTAT_SPEC>;
#[doc = "device IN endpoint 2 transmit FIFO status register"]
pub mod diep2tfstat;
#[doc = "DIEP3TFSTAT (r) register accessor: device IN endpoint 3 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep3tfstat`]
module"]
pub type DIEP3TFSTAT = crate::Reg<diep3tfstat::DIEP3TFSTAT_SPEC>;
#[doc = "device IN endpoint 3 transmit FIFO status register"]
pub mod diep3tfstat;
#[doc = "DIEP4TFSTAT (r) register accessor: device IN endpoint 4 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep4tfstat`]
module"]
pub type DIEP4TFSTAT = crate::Reg<diep4tfstat::DIEP4TFSTAT_SPEC>;
#[doc = "device IN endpoint 4 transmit FIFO status register"]
pub mod diep4tfstat;
#[doc = "DIEP5TFSTAT (r) register accessor: device IN endpoint 5 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diep5tfstat`]
module"]
pub type DIEP5TFSTAT = crate::Reg<diep5tfstat::DIEP5TFSTAT_SPEC>;
#[doc = "device IN endpoint 5 transmit FIFO status register"]
pub mod diep5tfstat;
