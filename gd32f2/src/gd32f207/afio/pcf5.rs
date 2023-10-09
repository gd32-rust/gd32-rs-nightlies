#[doc = "Register `PCF5` reader"]
pub type R = crate::R<PCF5_SPEC>;
#[doc = "Register `PCF5` writer"]
pub type W = crate::W<PCF5_SPEC>;
#[doc = "Field `I2C2_REMAP0` reader - I2C2 remapping 0"]
pub type I2C2_REMAP0_R = crate::BitReader;
#[doc = "Field `I2C2_REMAP0` writer - I2C2 remapping 0"]
pub type I2C2_REMAP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2_REMAP1` reader - I2C2 remapping 1"]
pub type I2C2_REMAP1_R = crate::BitReader;
#[doc = "Field `I2C2_REMAP1` writer - I2C2 remapping 1"]
pub type I2C2_REMAP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1_CH0_REMAP` reader - TIMER1_CH0 remapping"]
pub type TIMER1_CH0_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER1_CH0_REMAP` writer - TIMER1_CH0 remapping"]
pub type TIMER1_CH0_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER4_REMAP` reader - TIMER4 remapping"]
pub type TIMER4_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER4_REMAP` writer - TIMER4 remapping"]
pub type TIMER4_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER7_CHON_REMAP` reader - TIMER7_CHON remapping"]
pub type TIMER7_CHON_REMAP_R = crate::FieldReader;
#[doc = "Field `TIMER7_CHON_REMAP` writer - TIMER7_CHON remapping"]
pub type TIMER7_CHON_REMAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIMER7_CH_REMAP` reader - TIMER7_CH remapping"]
pub type TIMER7_CH_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER7_CH_REMAP` writer - TIMER7_CH remapping"]
pub type TIMER7_CH_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1_REMAP` reader - I2C1 remapping"]
pub type I2C1_REMAP_R = crate::FieldReader;
#[doc = "Field `I2C1_REMAP` writer - I2C1 remapping"]
pub type I2C1_REMAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SPI1_NSCK_REMAP` reader - SPI1_NSCK remapping"]
pub type SPI1_NSCK_REMAP_R = crate::FieldReader;
#[doc = "Field `SPI1_NSCK_REMAP` writer - SPI1_NSCK remapping"]
pub type SPI1_NSCK_REMAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SPI1_IO_REMAP` reader - SPI1_IO remapping"]
pub type SPI1_IO_REMAP_R = crate::FieldReader;
#[doc = "Field `SPI1_IO_REMAP` writer - SPI1_IO remapping"]
pub type SPI1_IO_REMAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `UART3_REMAP` reader - UART3 remapping"]
pub type UART3_REMAP_R = crate::BitReader;
#[doc = "Field `UART3_REMAP` writer - UART3 remapping"]
pub type UART3_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER11_REMAP` reader - TIMER11 remapping"]
pub type TIMER11_REMAP_R = crate::BitReader;
#[doc = "Field `TIMER11_REMAP` writer - TIMER11 remapping"]
pub type TIMER11_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN0_ADD_REMAP` reader - CAN0_ADD remapping"]
pub type CAN0_ADD_REMAP_R = crate::BitReader;
#[doc = "Field `CAN0_ADD_REMAP` writer - CAN0_ADD remapping"]
pub type CAN0_ADD_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENET_TXD3_REMAP` reader - ENET _TXD3 remapping"]
pub type ENET_TXD3_REMAP_R = crate::BitReader;
#[doc = "Field `ENET_TXD3_REMAP` writer - ENET _TXD3 remapping"]
pub type ENET_TXD3_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PPS_HI_REMAP` reader - PPS_HI remapping"]
pub type PPS_HI_REMAP_R = crate::BitReader;
#[doc = "Field `PPS_HI_REMAP` writer - PPS_HI remapping"]
pub type PPS_HI_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENET_TXD01_REMAP` reader - ENET_TXD01 remapping"]
pub type ENET_TXD01_REMAP_R = crate::BitReader;
#[doc = "Field `ENET_TXD01_REMAP` writer - ENET_TXD01 remapping"]
pub type ENET_TXD01_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENET_CRSCOL_REMAP` reader - ENET_CRSCOL remapping"]
pub type ENET_CRSCOL_REMAP_R = crate::BitReader;
#[doc = "Field `ENET_CRSCOL_REMAP` writer - ENET_CRSCOL remapping"]
pub type ENET_CRSCOL_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENET_RX_HI_REMAP` reader - ENET _RX_HI remapping"]
pub type ENET_RX_HI_REMAP_R = crate::BitReader;
#[doc = "Field `ENET_RX_HI_REMAP` writer - ENET _RX_HI remapping"]
pub type ENET_RX_HI_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART6_REMAP` reader - UART6 remapping"]
pub type UART6_REMAP_R = crate::BitReader;
#[doc = "Field `UART6_REMAP` writer - UART6 remapping"]
pub type UART6_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5_CK_REMAP` reader - USART5_CK remapping"]
pub type USART5_CK_REMAP_R = crate::BitReader;
#[doc = "Field `USART5_CK_REMAP` writer - USART5_CK remapping"]
pub type USART5_CK_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5_RTS_REMAP` reader - USART5_RTS remapping"]
pub type USART5_RTS_REMAP_R = crate::BitReader;
#[doc = "Field `USART5_RTS_REMAP` writer - USART5_RTS remapping"]
pub type USART5_RTS_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5_CTS_REMAP` reader - USART5_CTS remapping"]
pub type USART5_CTS_REMAP_R = crate::BitReader;
#[doc = "Field `USART5_CTS_REMAP` writer - USART5_CTS remapping"]
pub type USART5_CTS_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5_TX_REMAP` reader - USART5_TX remapping"]
pub type USART5_TX_REMAP_R = crate::BitReader;
#[doc = "Field `USART5_TX_REMAP` writer - USART5_TX remapping"]
pub type USART5_TX_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5_RX_REMAP` reader - USART5_RX remapping"]
pub type USART5_RX_REMAP_R = crate::BitReader;
#[doc = "Field `USART5_RX_REMAP` writer - USART5_RX remapping"]
pub type USART5_RX_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXMC_SDNWE_REMAP` reader - EXMC_SDNWE remapping"]
pub type EXMC_SDNWE_REMAP_R = crate::BitReader;
#[doc = "Field `EXMC_SDNWE_REMAP` writer - EXMC_SDNWE remapping"]
pub type EXMC_SDNWE_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXMC_SDCKE0_REMAP` reader - EXMC_SDCKE0 remapping"]
pub type EXMC_SDCKE0_REMAP_R = crate::BitReader;
#[doc = "Field `EXMC_SDCKE0_REMAP` writer - EXMC_SDCKE0 remapping"]
pub type EXMC_SDCKE0_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXMC_SDCKE1_REMAP` reader - EXMC_SDCKE1 remapping"]
pub type EXMC_SDCKE1_REMAP_R = crate::BitReader;
#[doc = "Field `EXMC_SDCKE1_REMAP` writer - EXMC_SDCKE1 remapping"]
pub type EXMC_SDCKE1_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXMC_SDNE0_REMAP` reader - EXMC_SDNE0 remapping"]
pub type EXMC_SDNE0_REMAP_R = crate::BitReader;
#[doc = "Field `EXMC_SDNE0_REMAP` writer - EXMC_SDNE0 remapping"]
pub type EXMC_SDNE0_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXMC_SDNE1_REMAP` reader - EXMC_SDNE1 remapping"]
pub type EXMC_SDNE1_REMAP_R = crate::BitReader;
#[doc = "Field `EXMC_SDNE1_REMAP` writer - EXMC_SDNE1 remapping"]
pub type EXMC_SDNE1_REMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C2 remapping 0"]
    #[inline(always)]
    pub fn i2c2_remap0(&self) -> I2C2_REMAP0_R {
        I2C2_REMAP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2 remapping 1"]
    #[inline(always)]
    pub fn i2c2_remap1(&self) -> I2C2_REMAP1_R {
        I2C2_REMAP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMER1_CH0 remapping"]
    #[inline(always)]
    pub fn timer1_ch0_remap(&self) -> TIMER1_CH0_REMAP_R {
        TIMER1_CH0_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 remapping"]
    #[inline(always)]
    pub fn timer4_remap(&self) -> TIMER4_REMAP_R {
        TIMER4_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - TIMER7_CHON remapping"]
    #[inline(always)]
    pub fn timer7_chon_remap(&self) -> TIMER7_CHON_REMAP_R {
        TIMER7_CHON_REMAP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - TIMER7_CH remapping"]
    #[inline(always)]
    pub fn timer7_ch_remap(&self) -> TIMER7_CH_REMAP_R {
        TIMER7_CH_REMAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1_remap(&self) -> I2C1_REMAP_R {
        I2C1_REMAP_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - SPI1_NSCK remapping"]
    #[inline(always)]
    pub fn spi1_nsck_remap(&self) -> SPI1_NSCK_REMAP_R {
        SPI1_NSCK_REMAP_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - SPI1_IO remapping"]
    #[inline(always)]
    pub fn spi1_io_remap(&self) -> SPI1_IO_REMAP_R {
        SPI1_IO_REMAP_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - UART3 remapping"]
    #[inline(always)]
    pub fn uart3_remap(&self) -> UART3_REMAP_R {
        UART3_REMAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIMER11 remapping"]
    #[inline(always)]
    pub fn timer11_remap(&self) -> TIMER11_REMAP_R {
        TIMER11_REMAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CAN0_ADD remapping"]
    #[inline(always)]
    pub fn can0_add_remap(&self) -> CAN0_ADD_REMAP_R {
        CAN0_ADD_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ENET _TXD3 remapping"]
    #[inline(always)]
    pub fn enet_txd3_remap(&self) -> ENET_TXD3_REMAP_R {
        ENET_TXD3_REMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PPS_HI remapping"]
    #[inline(always)]
    pub fn pps_hi_remap(&self) -> PPS_HI_REMAP_R {
        PPS_HI_REMAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ENET_TXD01 remapping"]
    #[inline(always)]
    pub fn enet_txd01_remap(&self) -> ENET_TXD01_REMAP_R {
        ENET_TXD01_REMAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ENET_CRSCOL remapping"]
    #[inline(always)]
    pub fn enet_crscol_remap(&self) -> ENET_CRSCOL_REMAP_R {
        ENET_CRSCOL_REMAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ENET _RX_HI remapping"]
    #[inline(always)]
    pub fn enet_rx_hi_remap(&self) -> ENET_RX_HI_REMAP_R {
        ENET_RX_HI_REMAP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - UART6 remapping"]
    #[inline(always)]
    pub fn uart6_remap(&self) -> UART6_REMAP_R {
        UART6_REMAP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - USART5_CK remapping"]
    #[inline(always)]
    pub fn usart5_ck_remap(&self) -> USART5_CK_REMAP_R {
        USART5_CK_REMAP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USART5_RTS remapping"]
    #[inline(always)]
    pub fn usart5_rts_remap(&self) -> USART5_RTS_REMAP_R {
        USART5_RTS_REMAP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - USART5_CTS remapping"]
    #[inline(always)]
    pub fn usart5_cts_remap(&self) -> USART5_CTS_REMAP_R {
        USART5_CTS_REMAP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USART5_TX remapping"]
    #[inline(always)]
    pub fn usart5_tx_remap(&self) -> USART5_TX_REMAP_R {
        USART5_TX_REMAP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USART5_RX remapping"]
    #[inline(always)]
    pub fn usart5_rx_remap(&self) -> USART5_RX_REMAP_R {
        USART5_RX_REMAP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EXMC_SDNWE remapping"]
    #[inline(always)]
    pub fn exmc_sdnwe_remap(&self) -> EXMC_SDNWE_REMAP_R {
        EXMC_SDNWE_REMAP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXMC_SDCKE0 remapping"]
    #[inline(always)]
    pub fn exmc_sdcke0_remap(&self) -> EXMC_SDCKE0_REMAP_R {
        EXMC_SDCKE0_REMAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EXMC_SDCKE1 remapping"]
    #[inline(always)]
    pub fn exmc_sdcke1_remap(&self) -> EXMC_SDCKE1_REMAP_R {
        EXMC_SDCKE1_REMAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EXMC_SDNE0 remapping"]
    #[inline(always)]
    pub fn exmc_sdne0_remap(&self) -> EXMC_SDNE0_REMAP_R {
        EXMC_SDNE0_REMAP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EXMC_SDNE1 remapping"]
    #[inline(always)]
    pub fn exmc_sdne1_remap(&self) -> EXMC_SDNE1_REMAP_R {
        EXMC_SDNE1_REMAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C2 remapping 0"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_remap0(&mut self) -> I2C2_REMAP0_W<PCF5_SPEC, 0> {
        I2C2_REMAP0_W::new(self)
    }
    #[doc = "Bit 1 - I2C2 remapping 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_remap1(&mut self) -> I2C2_REMAP1_W<PCF5_SPEC, 1> {
        I2C2_REMAP1_W::new(self)
    }
    #[doc = "Bit 2 - TIMER1_CH0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ch0_remap(&mut self) -> TIMER1_CH0_REMAP_W<PCF5_SPEC, 2> {
        TIMER1_CH0_REMAP_W::new(self)
    }
    #[doc = "Bit 3 - TIMER4 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer4_remap(&mut self) -> TIMER4_REMAP_W<PCF5_SPEC, 3> {
        TIMER4_REMAP_W::new(self)
    }
    #[doc = "Bits 4:5 - TIMER7_CHON remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer7_chon_remap(&mut self) -> TIMER7_CHON_REMAP_W<PCF5_SPEC, 4> {
        TIMER7_CHON_REMAP_W::new(self)
    }
    #[doc = "Bit 6 - TIMER7_CH remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer7_ch_remap(&mut self) -> TIMER7_CH_REMAP_W<PCF5_SPEC, 6> {
        TIMER7_CH_REMAP_W::new(self)
    }
    #[doc = "Bits 7:8 - I2C1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_remap(&mut self) -> I2C1_REMAP_W<PCF5_SPEC, 7> {
        I2C1_REMAP_W::new(self)
    }
    #[doc = "Bits 9:10 - SPI1_NSCK remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_nsck_remap(&mut self) -> SPI1_NSCK_REMAP_W<PCF5_SPEC, 9> {
        SPI1_NSCK_REMAP_W::new(self)
    }
    #[doc = "Bits 11:12 - SPI1_IO remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_io_remap(&mut self) -> SPI1_IO_REMAP_W<PCF5_SPEC, 11> {
        SPI1_IO_REMAP_W::new(self)
    }
    #[doc = "Bit 13 - UART3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uart3_remap(&mut self) -> UART3_REMAP_W<PCF5_SPEC, 13> {
        UART3_REMAP_W::new(self)
    }
    #[doc = "Bit 14 - TIMER11 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer11_remap(&mut self) -> TIMER11_REMAP_W<PCF5_SPEC, 14> {
        TIMER11_REMAP_W::new(self)
    }
    #[doc = "Bit 15 - CAN0_ADD remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can0_add_remap(&mut self) -> CAN0_ADD_REMAP_W<PCF5_SPEC, 15> {
        CAN0_ADD_REMAP_W::new(self)
    }
    #[doc = "Bit 16 - ENET _TXD3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_txd3_remap(&mut self) -> ENET_TXD3_REMAP_W<PCF5_SPEC, 16> {
        ENET_TXD3_REMAP_W::new(self)
    }
    #[doc = "Bit 17 - PPS_HI remapping"]
    #[inline(always)]
    #[must_use]
    pub fn pps_hi_remap(&mut self) -> PPS_HI_REMAP_W<PCF5_SPEC, 17> {
        PPS_HI_REMAP_W::new(self)
    }
    #[doc = "Bit 18 - ENET_TXD01 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_txd01_remap(&mut self) -> ENET_TXD01_REMAP_W<PCF5_SPEC, 18> {
        ENET_TXD01_REMAP_W::new(self)
    }
    #[doc = "Bit 19 - ENET_CRSCOL remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_crscol_remap(&mut self) -> ENET_CRSCOL_REMAP_W<PCF5_SPEC, 19> {
        ENET_CRSCOL_REMAP_W::new(self)
    }
    #[doc = "Bit 20 - ENET _RX_HI remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enet_rx_hi_remap(&mut self) -> ENET_RX_HI_REMAP_W<PCF5_SPEC, 20> {
        ENET_RX_HI_REMAP_W::new(self)
    }
    #[doc = "Bit 21 - UART6 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uart6_remap(&mut self) -> UART6_REMAP_W<PCF5_SPEC, 21> {
        UART6_REMAP_W::new(self)
    }
    #[doc = "Bit 22 - USART5_CK remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_ck_remap(&mut self) -> USART5_CK_REMAP_W<PCF5_SPEC, 22> {
        USART5_CK_REMAP_W::new(self)
    }
    #[doc = "Bit 23 - USART5_RTS remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_rts_remap(&mut self) -> USART5_RTS_REMAP_W<PCF5_SPEC, 23> {
        USART5_RTS_REMAP_W::new(self)
    }
    #[doc = "Bit 24 - USART5_CTS remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_cts_remap(&mut self) -> USART5_CTS_REMAP_W<PCF5_SPEC, 24> {
        USART5_CTS_REMAP_W::new(self)
    }
    #[doc = "Bit 25 - USART5_TX remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_tx_remap(&mut self) -> USART5_TX_REMAP_W<PCF5_SPEC, 25> {
        USART5_TX_REMAP_W::new(self)
    }
    #[doc = "Bit 26 - USART5_RX remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_rx_remap(&mut self) -> USART5_RX_REMAP_W<PCF5_SPEC, 26> {
        USART5_RX_REMAP_W::new(self)
    }
    #[doc = "Bit 27 - EXMC_SDNWE remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdnwe_remap(&mut self) -> EXMC_SDNWE_REMAP_W<PCF5_SPEC, 27> {
        EXMC_SDNWE_REMAP_W::new(self)
    }
    #[doc = "Bit 28 - EXMC_SDCKE0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdcke0_remap(&mut self) -> EXMC_SDCKE0_REMAP_W<PCF5_SPEC, 28> {
        EXMC_SDCKE0_REMAP_W::new(self)
    }
    #[doc = "Bit 29 - EXMC_SDCKE1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdcke1_remap(&mut self) -> EXMC_SDCKE1_REMAP_W<PCF5_SPEC, 29> {
        EXMC_SDCKE1_REMAP_W::new(self)
    }
    #[doc = "Bit 30 - EXMC_SDNE0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdne0_remap(&mut self) -> EXMC_SDNE0_REMAP_W<PCF5_SPEC, 30> {
        EXMC_SDNE0_REMAP_W::new(self)
    }
    #[doc = "Bit 31 - EXMC_SDNE1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_sdne1_remap(&mut self) -> EXMC_SDNE1_REMAP_W<PCF5_SPEC, 31> {
        EXMC_SDNE1_REMAP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AFIO port configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCF5_SPEC;
impl crate::RegisterSpec for PCF5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf5::R`](R) reader structure"]
impl crate::Readable for PCF5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcf5::W`](W) writer structure"]
impl crate::Writable for PCF5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCF5 to value 0"]
impl crate::Resettable for PCF5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
