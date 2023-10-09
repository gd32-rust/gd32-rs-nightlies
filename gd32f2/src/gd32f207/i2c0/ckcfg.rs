#[doc = "Register `CKCFG` reader"]
pub type R = crate::R<CKCFG_SPEC>;
#[doc = "Register `CKCFG` writer"]
pub type W = crate::W<CKCFG_SPEC>;
#[doc = "Field `CLKC` reader - I2C Clock control in master mode"]
pub type CLKC_R = crate::FieldReader<u16>;
#[doc = "Field `CLKC` writer - I2C Clock control in master mode"]
pub type CLKC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 12, O, u16>;
#[doc = "Field `DTCY` reader - Duty cycle in fast mode"]
pub type DTCY_R = crate::BitReader<DTCY_A>;
#[doc = "Duty cycle in fast mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCY_A {
    #[doc = "0: Duty cycle t_low/t_high = 2"]
    DUTY2 = 0,
    #[doc = "1: Duty cycle t_low/t_high = 16/9"]
    DUTY16_9 = 1,
}
impl From<DTCY_A> for bool {
    #[inline(always)]
    fn from(variant: DTCY_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCY_A {
        match self.bits {
            false => DTCY_A::DUTY2,
            true => DTCY_A::DUTY16_9,
        }
    }
    #[doc = "Duty cycle t_low/t_high = 2"]
    #[inline(always)]
    pub fn is_duty2(&self) -> bool {
        *self == DTCY_A::DUTY2
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        *self == DTCY_A::DUTY16_9
    }
}
#[doc = "Field `DTCY` writer - Duty cycle in fast mode"]
pub type DTCY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTCY_A>;
impl<'a, REG, const O: u8> DTCY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Duty cycle t_low/t_high = 2"]
    #[inline(always)]
    pub fn duty2(self) -> &'a mut crate::W<REG> {
        self.variant(DTCY_A::DUTY2)
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut crate::W<REG> {
        self.variant(DTCY_A::DUTY16_9)
    }
}
#[doc = "Field `FAST` reader - I2C speed selection in master mode"]
pub type FAST_R = crate::BitReader<FAST_A>;
#[doc = "I2C speed selection in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAST_A {
    #[doc = "0: Standard mode I2C"]
    STANDARD = 0,
    #[doc = "1: Fast mode I2C"]
    FAST = 1,
}
impl From<FAST_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_A) -> Self {
        variant as u8 != 0
    }
}
impl FAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_A {
        match self.bits {
            false => FAST_A::STANDARD,
            true => FAST_A::FAST,
        }
    }
    #[doc = "Standard mode I2C"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FAST_A::STANDARD
    }
    #[doc = "Fast mode I2C"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == FAST_A::FAST
    }
}
#[doc = "Field `FAST` writer - I2C speed selection in master mode"]
pub type FAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FAST_A>;
impl<'a, REG, const O: u8> FAST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard mode I2C"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(FAST_A::STANDARD)
    }
    #[doc = "Fast mode I2C"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(FAST_A::FAST)
    }
}
impl R {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    pub fn clkc(&self) -> CLKC_R {
        CLKC_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    pub fn dtcy(&self) -> DTCY_R {
        DTCY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn clkc(&mut self) -> CLKC_W<CKCFG_SPEC, 0> {
        CLKC_W::new(self)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtcy(&mut self) -> DTCY_W<CKCFG_SPEC, 14> {
        DTCY_W::new(self)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<CKCFG_SPEC, 15> {
        FAST_W::new(self)
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
#[doc = "Clock configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKCFG_SPEC;
impl crate::RegisterSpec for CKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckcfg::R`](R) reader structure"]
impl crate::Readable for CKCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckcfg::W`](W) writer structure"]
impl crate::Writable for CKCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCFG to value 0"]
impl crate::Resettable for CKCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
