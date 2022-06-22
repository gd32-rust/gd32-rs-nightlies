#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC0 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDUDR0_A {
    #[doc = "0: No DMA underrun error condition occurred"]
    NOUNDERRUN = 0,
    #[doc = "1: DMA underrun error condition occurred"]
    UNDERRUN = 1,
}
impl From<DDUDR0_A> for bool {
    #[inline(always)]
    fn from(variant: DDUDR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDUDR0` reader - DAC0 DMA underrun flag"]
pub type DDUDR0_R = crate::BitReader<DDUDR0_A>;
impl DDUDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDUDR0_A {
        match self.bits {
            false => DDUDR0_A::NOUNDERRUN,
            true => DDUDR0_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == DDUDR0_A::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == DDUDR0_A::UNDERRUN
    }
}
#[doc = "Field `DDUDR0` writer - DAC0 DMA underrun flag"]
pub type DDUDR0_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, DDUDR0_A, 13>;
impl<'a> DDUDR0_W<'a> {
    #[doc = "No DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DDUDR0_A::NOUNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DDUDR0_A::UNDERRUN)
    }
}
impl R {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr0(&self) -> DDUDR0_R {
        DDUDR0_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr0(&mut self) -> DDUDR0_W {
        DDUDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
