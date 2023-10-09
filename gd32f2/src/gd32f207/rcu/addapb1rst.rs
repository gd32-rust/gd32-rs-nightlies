#[doc = "Register `ADDAPB1RST` reader"]
pub type R = crate::R<ADDAPB1RST_SPEC>;
#[doc = "Register `ADDAPB1RST` writer"]
pub type W = crate::W<ADDAPB1RST_SPEC>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2C2RST_R = crate::BitReader<I2C2RST_A>;
#[doc = "I2C2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<I2C2RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C2RST_A> {
        match self.bits {
            true => Some(I2C2RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C2RST_A::RESET
    }
}
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2C2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C2RST_A>;
impl<'a, REG, const O: u8> I2C2RST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2RST_A::RESET)
    }
}
#[doc = "Field `UART6RST` reader - UART6 reset"]
pub use I2C2RST_R as UART6RST_R;
#[doc = "Field `UART7RST` reader - UART7 reset"]
pub use I2C2RST_R as UART7RST_R;
#[doc = "Field `UART6RST` writer - UART6 reset"]
pub use I2C2RST_W as UART6RST_W;
#[doc = "Field `UART7RST` writer - UART7 reset"]
pub use I2C2RST_W as UART7RST_W;
impl R {
    #[doc = "Bit 23 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - UART6 reset"]
    #[inline(always)]
    pub fn uart6rst(&self) -> UART6RST_R {
        UART6RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<ADDAPB1RST_SPEC, 23> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 30 - UART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart6rst(&mut self) -> UART6RST_W<ADDAPB1RST_SPEC, 30> {
        UART6RST_W::new(self)
    }
    #[doc = "Bit 31 - UART7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart7rst(&mut self) -> UART7RST_W<ADDAPB1RST_SPEC, 31> {
        UART7RST_W::new(self)
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
#[doc = "APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDAPB1RST_SPEC;
impl crate::RegisterSpec for ADDAPB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1rst::R`](R) reader structure"]
impl crate::Readable for ADDAPB1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addapb1rst::W`](W) writer structure"]
impl crate::Writable for ADDAPB1RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDAPB1RST to value 0"]
impl crate::Resettable for ADDAPB1RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
