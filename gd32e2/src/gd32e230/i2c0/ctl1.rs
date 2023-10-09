#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `I2CCLK` reader - Peripheral clock frequency"]
pub type I2CCLK_R = crate::FieldReader;
#[doc = "Field `I2CCLK` writer - Peripheral clock frequency"]
pub type I2CCLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIE_A>;
impl<'a, REG, const O: u8> ERRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Field `EVIE` reader - Event interrupt enable"]
pub type EVIE_R = crate::BitReader<EVIE_A>;
#[doc = "Event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVIE_A {
    #[doc = "0: Event interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Event interrupt enabled"]
    ENABLED = 1,
}
impl From<EVIE_A> for bool {
    #[inline(always)]
    fn from(variant: EVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVIE_A {
        match self.bits {
            false => EVIE_A::DISABLED,
            true => EVIE_A::ENABLED,
        }
    }
    #[doc = "Event interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EVIE_A::DISABLED
    }
    #[doc = "Event interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EVIE_A::ENABLED
    }
}
#[doc = "Field `EVIE` writer - Event interrupt enable"]
pub type EVIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EVIE_A>;
impl<'a, REG, const O: u8> EVIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EVIE_A::DISABLED)
    }
    #[doc = "Event interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EVIE_A::ENABLED)
    }
}
#[doc = "Field `BUFIE` reader - Buffer interrupt enable"]
pub type BUFIE_R = crate::BitReader<BUFIE_A>;
#[doc = "Buffer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFIE_A {
    #[doc = "0: TBE=1 or RBNE=1 does not generate any interrupt"]
    DISABLED = 0,
    #[doc = "1: TBE=1 or RBNE=1 generates Event interrupt"]
    ENABLED = 1,
}
impl From<BUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: BUFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFIE_A {
        match self.bits {
            false => BUFIE_A::DISABLED,
            true => BUFIE_A::ENABLED,
        }
    }
    #[doc = "TBE=1 or RBNE=1 does not generate any interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BUFIE_A::DISABLED
    }
    #[doc = "TBE=1 or RBNE=1 generates Event interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BUFIE_A::ENABLED
    }
}
#[doc = "Field `BUFIE` writer - Buffer interrupt enable"]
pub type BUFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BUFIE_A>;
impl<'a, REG, const O: u8> BUFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TBE=1 or RBNE=1 does not generate any interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BUFIE_A::DISABLED)
    }
    #[doc = "TBE=1 or RBNE=1 generates Event interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BUFIE_A::ENABLED)
    }
}
#[doc = "Field `DMAON` reader - DMA mode switch"]
pub type DMAON_R = crate::BitReader<DMAON_A>;
#[doc = "DMA mode switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAON_A {
    #[doc = "0: DMA requests disabled"]
    DISABLED = 0,
    #[doc = "1: DMA requests enabled"]
    ENABLED = 1,
}
impl From<DMAON_A> for bool {
    #[inline(always)]
    fn from(variant: DMAON_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAON_A {
        match self.bits {
            false => DMAON_A::DISABLED,
            true => DMAON_A::ENABLED,
        }
    }
    #[doc = "DMA requests disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAON_A::DISABLED
    }
    #[doc = "DMA requests enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAON_A::ENABLED
    }
}
#[doc = "Field `DMAON` writer - DMA mode switch"]
pub type DMAON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAON_A>;
impl<'a, REG, const O: u8> DMAON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA requests disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAON_A::DISABLED)
    }
    #[doc = "DMA requests enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAON_A::ENABLED)
    }
}
#[doc = "Field `DMALST` reader - Flag indicating DMA last transfer"]
pub type DMALST_R = crate::BitReader<DMALST_A>;
#[doc = "Flag indicating DMA last transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMALST_A {
    #[doc = "0: Next DMA EOT is not the last transfer"]
    NOT_LAST = 0,
    #[doc = "1: Next DMA EOT is the last transfer"]
    LAST = 1,
}
impl From<DMALST_A> for bool {
    #[inline(always)]
    fn from(variant: DMALST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMALST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMALST_A {
        match self.bits {
            false => DMALST_A::NOT_LAST,
            true => DMALST_A::LAST,
        }
    }
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == DMALST_A::NOT_LAST
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == DMALST_A::LAST
    }
}
#[doc = "Field `DMALST` writer - Flag indicating DMA last transfer"]
pub type DMALST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMALST_A>;
impl<'a, REG, const O: u8> DMALST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn not_last(self) -> &'a mut crate::W<REG> {
        self.variant(DMALST_A::NOT_LAST)
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(DMALST_A::LAST)
    }
}
impl R {
    #[doc = "Bits 0:6 - Peripheral clock frequency"]
    #[inline(always)]
    pub fn i2cclk(&self) -> I2CCLK_R {
        I2CCLK_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufie(&self) -> BUFIE_R {
        BUFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    pub fn dmaon(&self) -> DMAON_R {
        DMAON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    pub fn dmalst(&self) -> DMALST_R {
        DMALST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Peripheral clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn i2cclk(&mut self) -> I2CCLK_W<CTL1_SPEC, 0> {
        I2CCLK_W::new(self)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL1_SPEC, 8> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EVIE_W<CTL1_SPEC, 9> {
        EVIE_W::new(self)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufie(&mut self) -> BUFIE_W<CTL1_SPEC, 10> {
        BUFIE_W::new(self)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn dmaon(&mut self) -> DMAON_W<CTL1_SPEC, 11> {
        DMAON_W::new(self)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dmalst(&mut self) -> DMALST_W<CTL1_SPEC, 12> {
        DMALST_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
