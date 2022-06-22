#[doc = "Register `TAMP` reader"]
pub struct R(crate::R<TAMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP` writer"]
pub struct W(crate::W<TAMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_SPEC>;
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
impl From<crate::W<TAMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC15MDE` reader - PC15 mode"]
pub type PC15MDE_R = crate::BitReader<bool>;
#[doc = "Field `PC15MDE` writer - PC15 mode"]
pub type PC15MDE_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 23>;
#[doc = "Field `PC15VAL` reader - PC15 value"]
pub type PC15VAL_R = crate::BitReader<bool>;
#[doc = "Field `PC15VAL` writer - PC15 value"]
pub type PC15VAL_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 22>;
#[doc = "Field `PC14MDE` reader - PC14 mode"]
pub type PC14MDE_R = crate::BitReader<bool>;
#[doc = "Field `PC14MDE` writer - PC14 mode"]
pub type PC14MDE_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 21>;
#[doc = "Field `PC14VAL` reader - PC14 value"]
pub type PC14VAL_R = crate::BitReader<bool>;
#[doc = "Field `PC14VAL` writer - PC14 value"]
pub type PC14VAL_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 20>;
#[doc = "Field `PC13MDE` reader - PC13 mode"]
pub type PC13MDE_R = crate::BitReader<bool>;
#[doc = "Field `PC13MDE` writer - PC13 mode"]
pub type PC13MDE_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 19>;
#[doc = "Field `PC13VAL` reader - Alarm output type control/PC13 output value"]
pub type PC13VAL_R = crate::BitReader<bool>;
#[doc = "Field `PC13VAL` writer - Alarm output type control/PC13 output value"]
pub type PC13VAL_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 18>;
#[doc = "Field `DISPU` reader - RTC_TAMPx pull up disable bit"]
pub type DISPU_R = crate::BitReader<bool>;
#[doc = "Field `DISPU` writer - RTC_TAMPx pull up disable bit"]
pub type DISPU_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 15>;
#[doc = "Field `PRCH` reader - Precharge duration time of RTC_TAMPx"]
pub type PRCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRCH` writer - Precharge duration time of RTC_TAMPx"]
pub type PRCH_W<'a> = crate::FieldWriter<'a, u32, TAMP_SPEC, u8, u8, 2, 13>;
#[doc = "Field `FLT` reader - RTC_TAMPx filter count setting"]
pub type FLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT` writer - RTC_TAMPx filter count setting"]
pub type FLT_W<'a> = crate::FieldWriter<'a, u32, TAMP_SPEC, u8, u8, 2, 11>;
#[doc = "Field `FREQ` reader - Sample frequency of tamper event detection"]
pub type FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FREQ` writer - Sample frequency of tamper event detection"]
pub type FREQ_W<'a> = crate::FieldWriter<'a, u32, TAMP_SPEC, u8, u8, 3, 8>;
#[doc = "Field `TPTS` reader - Make tamper function used for timestamp function"]
pub type TPTS_R = crate::BitReader<bool>;
#[doc = "Field `TPTS` writer - Make tamper function used for timestamp function"]
pub type TPTS_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 7>;
#[doc = "Field `TP1EG` reader - Tamper 1 event trigger edge for RTC_TAMP1 input"]
pub type TP1EG_R = crate::BitReader<bool>;
#[doc = "Field `TP1EG` writer - Tamper 1 event trigger edge for RTC_TAMP1 input"]
pub type TP1EG_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 4>;
#[doc = "Field `TP1EN` reader - Tamper 1 detection enable"]
pub type TP1EN_R = crate::BitReader<bool>;
#[doc = "Field `TP1EN` writer - Tamper 1 detection enable"]
pub type TP1EN_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 3>;
#[doc = "Field `TPIE` reader - Tamper detection interrupt enable"]
pub type TPIE_R = crate::BitReader<bool>;
#[doc = "Field `TPIE` writer - Tamper detection interrupt enable"]
pub type TPIE_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 2>;
#[doc = "Field `TP0EG` reader - Tamper 0 event trigger edge for RTC_TAMP0 input"]
pub type TP0EG_R = crate::BitReader<bool>;
#[doc = "Field `TP0EG` writer - Tamper 0 event trigger edge for RTC_TAMP0 input"]
pub type TP0EG_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 1>;
#[doc = "Field `TP0EN` reader - RTC_TAMP1 input detection enable"]
pub type TP0EN_R = crate::BitReader<bool>;
#[doc = "Field `TP0EN` writer - RTC_TAMP1 input detection enable"]
pub type TP0EN_W<'a> = crate::BitWriter<'a, u32, TAMP_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mde(&self) -> PC15MDE_R {
        PC15MDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15val(&self) -> PC15VAL_R {
        PC15VAL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mde(&self) -> PC14MDE_R {
        PC14MDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14val(&self) -> PC14VAL_R {
        PC14VAL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mde(&self) -> PC13MDE_R {
        PC13MDE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Alarm output type control/PC13 output value"]
    #[inline(always)]
    pub fn pc13val(&self) -> PC13VAL_R {
        PC13VAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull up disable bit"]
    #[inline(always)]
    pub fn dispu(&self) -> DISPU_R {
        DISPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Precharge duration time of RTC_TAMPx"]
    #[inline(always)]
    pub fn prch(&self) -> PRCH_R {
        PRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count setting"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Sample frequency of tamper event detection"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 7 - Make tamper function used for timestamp function"]
    #[inline(always)]
    pub fn tpts(&self) -> TPTS_R {
        TPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tp1eg(&self) -> TP1EG_R {
        TP1EG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tp1en(&self) -> TP1EN_R {
        TP1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 0 event trigger edge for RTC_TAMP0 input"]
    #[inline(always)]
    pub fn tp0eg(&self) -> TP0EG_R {
        TP0EG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tp0en(&self) -> TP0EN_R {
        TP0EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mde(&mut self) -> PC15MDE_W {
        PC15MDE_W::new(self)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15val(&mut self) -> PC15VAL_W {
        PC15VAL_W::new(self)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mde(&mut self) -> PC14MDE_W {
        PC14MDE_W::new(self)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14val(&mut self) -> PC14VAL_W {
        PC14VAL_W::new(self)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mde(&mut self) -> PC13MDE_W {
        PC13MDE_W::new(self)
    }
    #[doc = "Bit 18 - Alarm output type control/PC13 output value"]
    #[inline(always)]
    pub fn pc13val(&mut self) -> PC13VAL_W {
        PC13VAL_W::new(self)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull up disable bit"]
    #[inline(always)]
    pub fn dispu(&mut self) -> DISPU_W {
        DISPU_W::new(self)
    }
    #[doc = "Bits 13:14 - Precharge duration time of RTC_TAMPx"]
    #[inline(always)]
    pub fn prch(&mut self) -> PRCH_W {
        PRCH_W::new(self)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count setting"]
    #[inline(always)]
    pub fn flt(&mut self) -> FLT_W {
        FLT_W::new(self)
    }
    #[doc = "Bits 8:10 - Sample frequency of tamper event detection"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W::new(self)
    }
    #[doc = "Bit 7 - Make tamper function used for timestamp function"]
    #[inline(always)]
    pub fn tpts(&mut self) -> TPTS_W {
        TPTS_W::new(self)
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tp1eg(&mut self) -> TP1EG_W {
        TP1EG_W::new(self)
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tp1en(&mut self) -> TP1EN_W {
        TP1EN_W::new(self)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TPIE_W {
        TPIE_W::new(self)
    }
    #[doc = "Bit 1 - Tamper 0 event trigger edge for RTC_TAMP0 input"]
    #[inline(always)]
    pub fn tp0eg(&mut self) -> TP0EG_W {
        TP0EG_W::new(self)
    }
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tp0en(&mut self) -> TP0EN_W {
        TP0EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp](index.html) module"]
pub struct TAMP_SPEC;
impl crate::RegisterSpec for TAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp::R](R) reader structure"]
impl crate::Readable for TAMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp::W](W) writer structure"]
impl crate::Writable for TAMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP to value 0"]
impl crate::Resettable for TAMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
