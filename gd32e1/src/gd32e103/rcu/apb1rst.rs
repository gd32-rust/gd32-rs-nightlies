#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<APB1RST_SPEC>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<APB1RST_SPEC>;
#[doc = "Field `TIMER1RST` reader - TIMER1 timer reset"]
pub type TIMER1RST_R = crate::BitReader<TIMER1RST_A>;
#[doc = "TIMER1 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER1RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<TIMER1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMER1RST_A> {
        match self.bits {
            true => Some(TIMER1RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIMER1RST_A::RESET
    }
}
#[doc = "Field `TIMER1RST` writer - TIMER1 timer reset"]
pub type TIMER1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER1RST_A>;
impl<'a, REG, const O: u8> TIMER1RST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER1RST_A::RESET)
    }
}
#[doc = "Field `TIMER2RST` reader - TIMER2 timer reset"]
pub use TIMER1RST_R as TIMER2RST_R;
#[doc = "Field `TIMER3RST` reader - TIMER3 timer reset"]
pub use TIMER1RST_R as TIMER3RST_R;
#[doc = "Field `TIMER4RST` reader - TIMER4 timer reset"]
pub use TIMER1RST_R as TIMER4RST_R;
#[doc = "Field `TIMER5RST` reader - TIMER5 timer reset"]
pub use TIMER1RST_R as TIMER5RST_R;
#[doc = "Field `TIMER6RST` reader - TIMER6 timer reset"]
pub use TIMER1RST_R as TIMER6RST_R;
#[doc = "Field `TIMER11RST` reader - TIMER11 timer reset"]
pub use TIMER1RST_R as TIMER11RST_R;
#[doc = "Field `TIMER12RST` reader - TIMER12 timer reset"]
pub use TIMER1RST_R as TIMER12RST_R;
#[doc = "Field `TIMER13RST` reader - TIMER13 timer reset"]
pub use TIMER1RST_R as TIMER13RST_R;
#[doc = "Field `WWDGTRST` reader - Window watchdog timer reset"]
pub use TIMER1RST_R as WWDGTRST_R;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub use TIMER1RST_R as SPI1RST_R;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub use TIMER1RST_R as SPI2RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use TIMER1RST_R as USART1RST_R;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub use TIMER1RST_R as USART2RST_R;
#[doc = "Field `UART3RST` reader - UART3 reset"]
pub use TIMER1RST_R as UART3RST_R;
#[doc = "Field `UART4RST` reader - UART4 reset"]
pub use TIMER1RST_R as UART4RST_R;
#[doc = "Field `I2C0RST` reader - I2C0 reset"]
pub use TIMER1RST_R as I2C0RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use TIMER1RST_R as I2C1RST_R;
#[doc = "Field `CAN0RST` reader - CAN0 reset"]
pub use TIMER1RST_R as CAN0RST_R;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub use TIMER1RST_R as CAN1RST_R;
#[doc = "Field `BKPIRST` reader - Backup interface reset"]
pub use TIMER1RST_R as BKPIRST_R;
#[doc = "Field `PMURST` reader - Power control reset"]
pub use TIMER1RST_R as PMURST_R;
#[doc = "Field `DACRST` reader - DAC reset"]
pub use TIMER1RST_R as DACRST_R;
#[doc = "Field `TIMER2RST` writer - TIMER2 timer reset"]
pub use TIMER1RST_W as TIMER2RST_W;
#[doc = "Field `TIMER3RST` writer - TIMER3 timer reset"]
pub use TIMER1RST_W as TIMER3RST_W;
#[doc = "Field `TIMER4RST` writer - TIMER4 timer reset"]
pub use TIMER1RST_W as TIMER4RST_W;
#[doc = "Field `TIMER5RST` writer - TIMER5 timer reset"]
pub use TIMER1RST_W as TIMER5RST_W;
#[doc = "Field `TIMER6RST` writer - TIMER6 timer reset"]
pub use TIMER1RST_W as TIMER6RST_W;
#[doc = "Field `TIMER11RST` writer - TIMER11 timer reset"]
pub use TIMER1RST_W as TIMER11RST_W;
#[doc = "Field `TIMER12RST` writer - TIMER12 timer reset"]
pub use TIMER1RST_W as TIMER12RST_W;
#[doc = "Field `TIMER13RST` writer - TIMER13 timer reset"]
pub use TIMER1RST_W as TIMER13RST_W;
#[doc = "Field `WWDGTRST` writer - Window watchdog timer reset"]
pub use TIMER1RST_W as WWDGTRST_W;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub use TIMER1RST_W as SPI1RST_W;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub use TIMER1RST_W as SPI2RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use TIMER1RST_W as USART1RST_W;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub use TIMER1RST_W as USART2RST_W;
#[doc = "Field `UART3RST` writer - UART3 reset"]
pub use TIMER1RST_W as UART3RST_W;
#[doc = "Field `UART4RST` writer - UART4 reset"]
pub use TIMER1RST_W as UART4RST_W;
#[doc = "Field `I2C0RST` writer - I2C0 reset"]
pub use TIMER1RST_W as I2C0RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use TIMER1RST_W as I2C1RST_W;
#[doc = "Field `CAN0RST` writer - CAN0 reset"]
pub use TIMER1RST_W as CAN0RST_W;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub use TIMER1RST_W as CAN1RST_W;
#[doc = "Field `BKPIRST` writer - Backup interface reset"]
pub use TIMER1RST_W as BKPIRST_W;
#[doc = "Field `PMURST` writer - Power control reset"]
pub use TIMER1RST_W as PMURST_W;
#[doc = "Field `DACRST` writer - DAC reset"]
pub use TIMER1RST_W as DACRST_W;
impl R {
    #[doc = "Bit 0 - TIMER1 timer reset"]
    #[inline(always)]
    pub fn timer1rst(&self) -> TIMER1RST_R {
        TIMER1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER2 timer reset"]
    #[inline(always)]
    pub fn timer2rst(&self) -> TIMER2RST_R {
        TIMER2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMER3 timer reset"]
    #[inline(always)]
    pub fn timer3rst(&self) -> TIMER3RST_R {
        TIMER3RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 timer reset"]
    #[inline(always)]
    pub fn timer4rst(&self) -> TIMER4RST_R {
        TIMER4RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    pub fn timer5rst(&self) -> TIMER5RST_R {
        TIMER5RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER6 timer reset"]
    #[inline(always)]
    pub fn timer6rst(&self) -> TIMER6RST_R {
        TIMER6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMER11 timer reset"]
    #[inline(always)]
    pub fn timer11rst(&self) -> TIMER11RST_R {
        TIMER11RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMER12 timer reset"]
    #[inline(always)]
    pub fn timer12rst(&self) -> TIMER12RST_R {
        TIMER12RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    pub fn timer13rst(&self) -> TIMER13RST_R {
        TIMER13RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    pub fn wwdgtrst(&self) -> WWDGTRST_R {
        WWDGTRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART3 reset"]
    #[inline(always)]
    pub fn uart3rst(&self) -> UART3RST_R {
        UART3RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    pub fn can0rst(&self) -> CAN0RST_R {
        CAN0RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    pub fn bkpirst(&self) -> BKPIRST_R {
        BKPIRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    pub fn pmurst(&self) -> PMURST_R {
        PMURST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer1rst(&mut self) -> TIMER1RST_W<APB1RST_SPEC, 0> {
        TIMER1RST_W::new(self)
    }
    #[doc = "Bit 1 - TIMER2 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer2rst(&mut self) -> TIMER2RST_W<APB1RST_SPEC, 1> {
        TIMER2RST_W::new(self)
    }
    #[doc = "Bit 2 - TIMER3 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer3rst(&mut self) -> TIMER3RST_W<APB1RST_SPEC, 2> {
        TIMER3RST_W::new(self)
    }
    #[doc = "Bit 3 - TIMER4 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer4rst(&mut self) -> TIMER4RST_W<APB1RST_SPEC, 3> {
        TIMER4RST_W::new(self)
    }
    #[doc = "Bit 4 - TIMER5 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer5rst(&mut self) -> TIMER5RST_W<APB1RST_SPEC, 4> {
        TIMER5RST_W::new(self)
    }
    #[doc = "Bit 5 - TIMER6 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer6rst(&mut self) -> TIMER6RST_W<APB1RST_SPEC, 5> {
        TIMER6RST_W::new(self)
    }
    #[doc = "Bit 6 - TIMER11 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer11rst(&mut self) -> TIMER11RST_W<APB1RST_SPEC, 6> {
        TIMER11RST_W::new(self)
    }
    #[doc = "Bit 7 - TIMER12 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer12rst(&mut self) -> TIMER12RST_W<APB1RST_SPEC, 7> {
        TIMER12RST_W::new(self)
    }
    #[doc = "Bit 8 - TIMER13 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer13rst(&mut self) -> TIMER13RST_W<APB1RST_SPEC, 8> {
        TIMER13RST_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgtrst(&mut self) -> WWDGTRST_W<APB1RST_SPEC, 11> {
        WWDGTRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB1RST_SPEC, 14> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1RST_SPEC, 15> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 17 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB1RST_SPEC, 17> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 18 - USART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1RST_SPEC, 18> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 19 - UART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart3rst(&mut self) -> UART3RST_W<APB1RST_SPEC, 19> {
        UART3RST_W::new(self)
    }
    #[doc = "Bit 20 - UART4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<APB1RST_SPEC, 20> {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0rst(&mut self) -> I2C0RST_W<APB1RST_SPEC, 21> {
        I2C0RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RST_SPEC, 22> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 25 - CAN0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can0rst(&mut self) -> CAN0RST_W<APB1RST_SPEC, 25> {
        CAN0RST_W::new(self)
    }
    #[doc = "Bit 26 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1rst(&mut self) -> CAN1RST_W<APB1RST_SPEC, 26> {
        CAN1RST_W::new(self)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkpirst(&mut self) -> BKPIRST_W<APB1RST_SPEC, 27> {
        BKPIRST_W::new(self)
    }
    #[doc = "Bit 28 - Power control reset"]
    #[inline(always)]
    #[must_use]
    pub fn pmurst(&mut self) -> PMURST_W<APB1RST_SPEC, 28> {
        PMURST_W::new(self)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn dacrst(&mut self) -> DACRST_W<APB1RST_SPEC, 29> {
        DACRST_W::new(self)
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
#[doc = "APB1 reset register (RCU_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RST_SPEC;
impl crate::RegisterSpec for APB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rst::R`](R) reader structure"]
impl crate::Readable for APB1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1rst::W`](W) writer structure"]
impl crate::Writable for APB1RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1RST to value 0"]
impl crate::Resettable for APB1RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
