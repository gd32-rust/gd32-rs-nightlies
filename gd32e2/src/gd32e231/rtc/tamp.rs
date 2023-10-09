#[doc = "Register `TAMP` reader"]
pub type R = crate::R<TAMP_SPEC>;
#[doc = "Register `TAMP` writer"]
pub type W = crate::W<TAMP_SPEC>;
#[doc = "Field `TPIE` reader - Tamper detection interrupt enable"]
pub type TPIE_R = crate::BitReader;
#[doc = "Field `TPIE` writer - Tamper detection interrupt enable"]
pub type TPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TP1EN` reader - Tamper 1 detection enable"]
pub type TP1EN_R = crate::BitReader;
#[doc = "Field `TP1EN` writer - Tamper 1 detection enable"]
pub type TP1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TP1EG` reader - Tamper 1 event trigger edge"]
pub type TP1EG_R = crate::BitReader;
#[doc = "Field `TP1EG` writer - Tamper 1 event trigger edge"]
pub type TP1EG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FREQ` reader - Tamper sampling frequency"]
pub type FREQ_R = crate::FieldReader;
#[doc = "Field `FREQ` writer - Tamper sampling frequency"]
pub type FREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FLT` reader - RTC_TAMPx filter count"]
pub type FLT_R = crate::FieldReader;
#[doc = "Field `FLT` writer - RTC_TAMPx filter count"]
pub type FLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PRCH` reader - RTC_TAMPx precharge duration"]
pub type PRCH_R = crate::FieldReader;
#[doc = "Field `PRCH` writer - RTC_TAMPx precharge duration"]
pub type PRCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DISPU` reader - RTC_TAMPx pull-up disable"]
pub type DISPU_R = crate::BitReader;
#[doc = "Field `DISPU` writer - RTC_TAMPx pull-up disable"]
pub type DISPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC14VAL` reader - PC14 value"]
pub type PC14VAL_R = crate::BitReader;
#[doc = "Field `PC14VAL` writer - PC14 value"]
pub type PC14VAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC14MDE` reader - PC14 mode"]
pub type PC14MDE_R = crate::BitReader;
#[doc = "Field `PC14MDE` writer - PC14 mode"]
pub type PC14MDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC15VAL` reader - PC15 value"]
pub type PC15VAL_R = crate::BitReader;
#[doc = "Field `PC15VAL` writer - PC15 value"]
pub type PC15VAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC15MDE` reader - PC15 mode"]
pub type PC15MDE_R = crate::BitReader;
#[doc = "Field `PC15MDE` writer - PC15 mode"]
pub type PC15MDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tp1en(&self) -> TP1EN_R {
        TP1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge"]
    #[inline(always)]
    pub fn tp1eg(&self) -> TP1EG_R {
        TP1EG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn prch(&self) -> PRCH_R {
        PRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn dispu(&self) -> DISPU_R {
        DISPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14val(&self) -> PC14VAL_R {
        PC14VAL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mde(&self) -> PC14MDE_R {
        PC14MDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15val(&self) -> PC15VAL_R {
        PC15VAL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mde(&self) -> PC15MDE_R {
        PC15MDE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TPIE_W<TAMP_SPEC, 2> {
        TPIE_W::new(self)
    }
    #[doc = "Bit 3 - Tamper 1 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp1en(&mut self) -> TP1EN_W<TAMP_SPEC, 3> {
        TP1EN_W::new(self)
    }
    #[doc = "Bit 4 - Tamper 1 event trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn tp1eg(&mut self) -> TP1EG_W<TAMP_SPEC, 4> {
        TP1EG_W::new(self)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<TAMP_SPEC, 8> {
        FREQ_W::new(self)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    #[must_use]
    pub fn flt(&mut self) -> FLT_W<TAMP_SPEC, 11> {
        FLT_W::new(self)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    #[must_use]
    pub fn prch(&mut self) -> PRCH_W<TAMP_SPEC, 13> {
        PRCH_W::new(self)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    #[must_use]
    pub fn dispu(&mut self) -> DISPU_W<TAMP_SPEC, 15> {
        DISPU_W::new(self)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    #[must_use]
    pub fn pc14val(&mut self) -> PC14VAL_W<TAMP_SPEC, 20> {
        PC14VAL_W::new(self)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pc14mde(&mut self) -> PC14MDE_W<TAMP_SPEC, 21> {
        PC14MDE_W::new(self)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    #[must_use]
    pub fn pc15val(&mut self) -> PC15VAL_W<TAMP_SPEC, 22> {
        PC15VAL_W::new(self)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pc15mde(&mut self) -> PC15MDE_W<TAMP_SPEC, 23> {
        PC15MDE_W::new(self)
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
#[doc = "tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_SPEC;
impl crate::RegisterSpec for TAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp::R`](R) reader structure"]
impl crate::Readable for TAMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamp::W`](W) writer structure"]
impl crate::Writable for TAMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP to value 0"]
impl crate::Resettable for TAMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
