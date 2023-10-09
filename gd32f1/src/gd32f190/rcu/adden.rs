#[doc = "Register `ADDEN` reader"]
pub type R = crate::R<ADDEN_SPEC>;
#[doc = "Register `ADDEN` writer"]
pub type W = crate::W<ADDEN_SPEC>;
#[doc = "Field `I2C2EN` reader - I2C2 unit clock enable"]
pub type I2C2EN_R = crate::BitReader<I2C2EN_A>;
#[doc = "I2C2 unit clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<I2C2EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2EN_A {
        match self.bits {
            false => I2C2EN_A::DISABLED,
            true => I2C2EN_A::ENABLED,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C2EN_A::DISABLED
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C2EN_A::ENABLED
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 unit clock enable"]
pub type I2C2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C2EN_A>;
impl<'a, REG, const O: u8> I2C2EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - I2C2 unit clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C2 unit clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<ADDEN_SPEC, 0> {
        I2C2EN_W::new(self)
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
#[doc = "Additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adden::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adden::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDEN_SPEC;
impl crate::RegisterSpec for ADDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adden::R`](R) reader structure"]
impl crate::Readable for ADDEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adden::W`](W) writer structure"]
impl crate::Writable for ADDEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDEN to value 0"]
impl crate::Resettable for ADDEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
