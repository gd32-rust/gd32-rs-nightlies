#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `CFGRST` reader - System configuration reset"]
pub type CFGRST_R = crate::BitReader<CFGRST_A>;
#[doc = "System configuration reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFGRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<CFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: CFGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFGRST_A> {
        match self.bits {
            true => Some(CFGRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CFGRST_A::RESET
    }
}
#[doc = "Field `CFGRST` writer - System configuration reset"]
pub type CFGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CFGRST_A>;
impl<'a, REG, const O: u8> CFGRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CFGRST_A::RESET)
    }
}
#[doc = "Field `ADCRST` reader - ADC reset"]
pub use CFGRST_R as ADCRST_R;
#[doc = "Field `TIMER0RST` reader - TIMER0 reset"]
pub use CFGRST_R as TIMER0RST_R;
#[doc = "Field `SPI0RST` reader - SPI0 Reset"]
pub use CFGRST_R as SPI0RST_R;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub use CFGRST_R as USART0RST_R;
#[doc = "Field `TIMER14RST` reader - TIMER14 reset"]
pub use CFGRST_R as TIMER14RST_R;
#[doc = "Field `TIMER15RST` reader - TIMER15 reset"]
pub use CFGRST_R as TIMER15RST_R;
#[doc = "Field `TIMER16RST` reader - TIMER16 reset"]
pub use CFGRST_R as TIMER16RST_R;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub use CFGRST_W as ADCRST_W;
#[doc = "Field `TIMER0RST` writer - TIMER0 reset"]
pub use CFGRST_W as TIMER0RST_W;
#[doc = "Field `SPI0RST` writer - SPI0 Reset"]
pub use CFGRST_W as SPI0RST_W;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub use CFGRST_W as USART0RST_W;
#[doc = "Field `TIMER14RST` writer - TIMER14 reset"]
pub use CFGRST_W as TIMER14RST_W;
#[doc = "Field `TIMER15RST` writer - TIMER15 reset"]
pub use CFGRST_W as TIMER15RST_W;
#[doc = "Field `TIMER16RST` writer - TIMER16 reset"]
pub use CFGRST_W as TIMER16RST_W;
impl R {
    #[doc = "Bit 0 - System configuration reset"]
    #[inline(always)]
    pub fn cfgrst(&self) -> CFGRST_R {
        CFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> TIMER0RST_R {
        TIMER0RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> USART0RST_R {
        USART0RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    pub fn timer14rst(&self) -> TIMER14RST_R {
        TIMER14RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    pub fn timer15rst(&self) -> TIMER15RST_R {
        TIMER15RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    pub fn timer16rst(&self) -> TIMER16RST_R {
        TIMER16RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration reset"]
    #[inline(always)]
    #[must_use]
    pub fn cfgrst(&mut self) -> CFGRST_W<APB2RST_SPEC, 0> {
        CFGRST_W::new(self)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<APB2RST_SPEC, 9> {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 11 - TIMER0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer0rst(&mut self) -> TIMER0RST_W<APB2RST_SPEC, 11> {
        TIMER0RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> SPI0RST_W<APB2RST_SPEC, 12> {
        SPI0RST_W::new(self)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart0rst(&mut self) -> USART0RST_W<APB2RST_SPEC, 14> {
        USART0RST_W::new(self)
    }
    #[doc = "Bit 16 - TIMER14 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer14rst(&mut self) -> TIMER14RST_W<APB2RST_SPEC, 16> {
        TIMER14RST_W::new(self)
    }
    #[doc = "Bit 17 - TIMER15 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer15rst(&mut self) -> TIMER15RST_W<APB2RST_SPEC, 17> {
        TIMER15RST_W::new(self)
    }
    #[doc = "Bit 18 - TIMER16 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer16rst(&mut self) -> TIMER16RST_W<APB2RST_SPEC, 18> {
        TIMER16RST_W::new(self)
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
#[doc = "APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
