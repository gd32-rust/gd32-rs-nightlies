#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTF` writer"]
pub struct W(crate::W<INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTF_SPEC>;
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
impl From<crate::W<INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPNUM` reader - Endpoint Identifier"]
pub type EPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNUM` writer - Endpoint Identifier"]
pub type EPNUM_W<'a> = crate::FieldWriter<'a, u32, INTF_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DIR` reader - Direction of transaction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Direction of transaction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 4>;
#[doc = "Field `L1REQ` reader - LPM L1 transaction is successful"]
pub type L1REQ_R = crate::BitReader<bool>;
#[doc = "Field `L1REQ` writer - LPM L1 transaction is successful"]
pub type L1REQ_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 7>;
#[doc = "Field `ESOFIF` reader - Expected start of frame interrupt flag"]
pub type ESOFIF_R = crate::BitReader<bool>;
#[doc = "Field `ESOFIF` writer - Expected start of frame interrupt flag"]
pub type ESOFIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 8>;
#[doc = "Field `SOFIF` reader - start of frame interrupt flag"]
pub type SOFIF_R = crate::BitReader<bool>;
#[doc = "Field `SOFIF` writer - start of frame interrupt flag"]
pub type SOFIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 9>;
#[doc = "Field `RSTIF` reader - reset interrupt flag"]
pub type RSTIF_R = crate::BitReader<bool>;
#[doc = "Field `RSTIF` writer - reset interrupt flag"]
pub type RSTIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 10>;
#[doc = "Field `SPSIF` reader - Suspend mode interrupt flag"]
pub type SPSIF_R = crate::BitReader<bool>;
#[doc = "Field `SPSIF` writer - Suspend mode interrupt flag"]
pub type SPSIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 11>;
#[doc = "Field `WKUPIF` reader - Wakeup interrupt flag"]
pub type WKUPIF_R = crate::BitReader<bool>;
#[doc = "Field `WKUPIF` writer - Wakeup interrupt flag"]
pub type WKUPIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 12>;
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ERRIF_R = crate::BitReader<bool>;
#[doc = "Field `ERRIF` writer - Error interrupt flag"]
pub type ERRIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 13>;
#[doc = "Field `PMOUIF` reader - Packet memory area over / underrun interrupt flag"]
pub type PMOUIF_R = crate::BitReader<bool>;
#[doc = "Field `PMOUIF` writer - Packet memory area over / underrun interrupt flag"]
pub type PMOUIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 14>;
#[doc = "Field `STIF` reader - Successful transfer interrupt flag"]
pub type STIF_R = crate::BitReader<bool>;
#[doc = "Field `STIF` writer - Successful transfer interrupt flag"]
pub type STIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 transaction is successful"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt flag"]
    #[inline(always)]
    pub fn esofif(&self) -> ESOFIF_R {
        ESOFIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame interrupt flag"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reset interrupt flag"]
    #[inline(always)]
    pub fn rstif(&self) -> RSTIF_R {
        RSTIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt flag"]
    #[inline(always)]
    pub fn spsif(&self) -> SPSIF_R {
        SPSIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&self) -> WKUPIF_R {
        WKUPIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt flag"]
    #[inline(always)]
    pub fn pmouif(&self) -> PMOUIF_R {
        PMOUIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Successful transfer interrupt flag"]
    #[inline(always)]
    pub fn stif(&self) -> STIF_R {
        STIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W {
        EPNUM_W::new(self)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 7 - LPM L1 transaction is successful"]
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W {
        L1REQ_W::new(self)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt flag"]
    #[inline(always)]
    pub fn esofif(&mut self) -> ESOFIF_W {
        ESOFIF_W::new(self)
    }
    #[doc = "Bit 9 - start of frame interrupt flag"]
    #[inline(always)]
    pub fn sofif(&mut self) -> SOFIF_W {
        SOFIF_W::new(self)
    }
    #[doc = "Bit 10 - reset interrupt flag"]
    #[inline(always)]
    pub fn rstif(&mut self) -> RSTIF_W {
        RSTIF_W::new(self)
    }
    #[doc = "Bit 11 - Suspend mode interrupt flag"]
    #[inline(always)]
    pub fn spsif(&mut self) -> SPSIF_W {
        SPSIF_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&mut self) -> WKUPIF_W {
        WKUPIF_W::new(self)
    }
    #[doc = "Bit 13 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&mut self) -> ERRIF_W {
        ERRIF_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt flag"]
    #[inline(always)]
    pub fn pmouif(&mut self) -> PMOUIF_W {
        PMOUIF_W::new(self)
    }
    #[doc = "Bit 15 - Successful transfer interrupt flag"]
    #[inline(always)]
    pub fn stif(&mut self) -> STIF_W {
        STIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intf::W](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
