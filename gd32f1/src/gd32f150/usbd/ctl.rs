#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETRST` reader - USB Reset"]
pub type SETRST_R = crate::BitReader<bool>;
#[doc = "Field `SETRST` writer - USB Reset"]
pub type SETRST_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 0>;
#[doc = "Field `CLOSE` reader - USB close"]
pub type CLOSE_R = crate::BitReader<bool>;
#[doc = "Field `CLOSE` writer - USB close"]
pub type CLOSE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 1>;
#[doc = "Field `LOWM` reader - Low-power mode"]
pub type LOWM_R = crate::BitReader<bool>;
#[doc = "Field `LOWM` writer - Low-power mode"]
pub type LOWM_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 2>;
#[doc = "Field `SETSPS` reader - Set suspend state"]
pub type SETSPS_R = crate::BitReader<bool>;
#[doc = "Field `SETSPS` writer - Set suspend state"]
pub type SETSPS_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 3>;
#[doc = "Field `RSREQ` reader - Send resume request"]
pub type RSREQ_R = crate::BitReader<bool>;
#[doc = "Field `RSREQ` writer - Send resume request"]
pub type RSREQ_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 4>;
#[doc = "Field `ESOFIE` reader - Expected start of frame interrupt enable"]
pub type ESOFIE_R = crate::BitReader<bool>;
#[doc = "Field `ESOFIE` writer - Expected start of frame interrupt enable"]
pub type ESOFIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 8>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SOFIE_R = crate::BitReader<bool>;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SOFIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 9>;
#[doc = "Field `RSTIE` reader - USB reset interrupt enable"]
pub type RSTIE_R = crate::BitReader<bool>;
#[doc = "Field `RSTIE` writer - USB reset interrupt enable"]
pub type RSTIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 10>;
#[doc = "Field `SPSIE` reader - Suspend state interrupt enable"]
pub type SPSIE_R = crate::BitReader<bool>;
#[doc = "Field `SPSIE` writer - Suspend state interrupt enable"]
pub type SPSIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 11>;
#[doc = "Field `WKUPIE` reader - Wakeup interrupt mask"]
pub type WKUPIE_R = crate::BitReader<bool>;
#[doc = "Field `WKUPIE` writer - Wakeup interrupt mask"]
pub type WKUPIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 12>;
#[doc = "Field `ERRIE` reader - Error interrupt mask"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt mask"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 13>;
#[doc = "Field `PMOUIE` reader - Packet memory overrun / underrun interrupt enable"]
pub type PMOUIE_R = crate::BitReader<bool>;
#[doc = "Field `PMOUIE` writer - Packet memory overrun / underrun interrupt enable"]
pub type PMOUIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 14>;
#[doc = "Field `STIE` reader - Successful transfer interrupt enable"]
pub type STIE_R = crate::BitReader<bool>;
#[doc = "Field `STIE` writer - Successful transfer interrupt enable"]
pub type STIE_W<'a> = crate::BitWriter<'a, u16, CTL_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - USB Reset"]
    #[inline(always)]
    pub fn setrst(&self) -> SETRST_R {
        SETRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB close"]
    #[inline(always)]
    pub fn close(&self) -> CLOSE_R {
        CLOSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lowm(&self) -> LOWM_R {
        LOWM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set suspend state"]
    #[inline(always)]
    pub fn setsps(&self) -> SETSPS_R {
        SETSPS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Send resume request"]
    #[inline(always)]
    pub fn rsreq(&self) -> RSREQ_R {
        RSREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt enable"]
    #[inline(always)]
    pub fn esofie(&self) -> ESOFIE_R {
        ESOFIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend state interrupt enable"]
    #[inline(always)]
    pub fn spsie(&self) -> SPSIE_R {
        SPSIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory overrun / underrun interrupt enable"]
    #[inline(always)]
    pub fn pmouie(&self) -> PMOUIE_R {
        PMOUIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Successful transfer interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reset"]
    #[inline(always)]
    pub fn setrst(&mut self) -> SETRST_W {
        SETRST_W::new(self)
    }
    #[doc = "Bit 1 - USB close"]
    #[inline(always)]
    pub fn close(&mut self) -> CLOSE_W {
        CLOSE_W::new(self)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lowm(&mut self) -> LOWM_W {
        LOWM_W::new(self)
    }
    #[doc = "Bit 3 - Set suspend state"]
    #[inline(always)]
    pub fn setsps(&mut self) -> SETSPS_W {
        SETSPS_W::new(self)
    }
    #[doc = "Bit 4 - Send resume request"]
    #[inline(always)]
    pub fn rsreq(&mut self) -> RSREQ_W {
        RSREQ_W::new(self)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt enable"]
    #[inline(always)]
    pub fn esofie(&mut self) -> ESOFIE_W {
        ESOFIE_W::new(self)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 10 - USB reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W {
        RSTIE_W::new(self)
    }
    #[doc = "Bit 11 - Suspend state interrupt enable"]
    #[inline(always)]
    pub fn spsie(&mut self) -> SPSIE_W {
        SPSIE_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupie(&mut self) -> WKUPIE_W {
        WKUPIE_W::new(self)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory overrun / underrun interrupt enable"]
    #[inline(always)]
    pub fn pmouie(&mut self) -> PMOUIE_W {
        PMOUIE_W::new(self)
    }
    #[doc = "Bit 15 - Successful transfer interrupt enable"]
    #[inline(always)]
    pub fn stie(&mut self) -> STIE_W {
        STIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0x03"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
