#[doc = "Register `I2SPSC` reader"]
pub type R = crate::R<I2SPSC_SPEC>;
#[doc = "Register `I2SPSC` writer"]
pub type W = crate::W<I2SPSC_SPEC>;
#[doc = "Field `DIV` reader - Dividing factor for the prescaler"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - Dividing factor for the prescaler"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `OF` reader - Odd factor for the prescaler"]
pub type OF_R = crate::BitReader<OF_A>;
#[doc = "Odd factor for the prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF_A {
    #[doc = "0: Real divider value is DIV * 2"]
    EVEN = 0,
    #[doc = "1: Real divider value is (DIV * 2) + 1"]
    ODD = 1,
}
impl From<OF_A> for bool {
    #[inline(always)]
    fn from(variant: OF_A) -> Self {
        variant as u8 != 0
    }
}
impl OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OF_A {
        match self.bits {
            false => OF_A::EVEN,
            true => OF_A::ODD,
        }
    }
    #[doc = "Real divider value is DIV * 2"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == OF_A::EVEN
    }
    #[doc = "Real divider value is (DIV * 2) + 1"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == OF_A::ODD
    }
}
#[doc = "Field `OF` writer - Odd factor for the prescaler"]
pub type OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OF_A>;
impl<'a, REG, const O: u8> OF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real divider value is DIV * 2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(OF_A::EVEN)
    }
    #[doc = "Real divider value is (DIV * 2) + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(OF_A::ODD)
    }
}
#[doc = "Field `MCKOEN` reader - I2S_MCK output enable"]
pub type MCKOEN_R = crate::BitReader<MCKOEN_A>;
#[doc = "I2S_MCK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOEN_A {
    #[doc = "0: Master clock output is disabled"]
    DISABLED = 0,
    #[doc = "1: Master clock output is enabled"]
    ENABLED = 1,
}
impl From<MCKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCKOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKOEN_A {
        match self.bits {
            false => MCKOEN_A::DISABLED,
            true => MCKOEN_A::ENABLED,
        }
    }
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOEN_A::DISABLED
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOEN_A::ENABLED
    }
}
#[doc = "Field `MCKOEN` writer - I2S_MCK output enable"]
pub type MCKOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MCKOEN_A>;
impl<'a, REG, const O: u8> MCKOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOEN_A::DISABLED)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    pub fn mckoen(&self) -> MCKOEN_R {
        MCKOEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<I2SPSC_SPEC, 0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<I2SPSC_SPEC, 8> {
        OF_W::new(self)
    }
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoen(&mut self) -> MCKOEN_W<I2SPSC_SPEC, 9> {
        MCKOEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2spsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2spsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SPSC_SPEC;
impl crate::RegisterSpec for I2SPSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`i2spsc::R`](R) reader structure"]
impl crate::Readable for I2SPSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2spsc::W`](W) writer structure"]
impl crate::Writable for I2SPSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SPSC to value 0x02"]
impl crate::Resettable for I2SPSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
