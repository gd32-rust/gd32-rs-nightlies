#[doc = "Register `GOTGINTF` reader"]
pub struct R(crate::R<GOTGINTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGINTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGINTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGINTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGINTF` writer"]
pub struct W(crate::W<GOTGINTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGINTF_SPEC>;
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
impl From<crate::W<GOTGINTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGINTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SESEND` reader - Session end"]
pub type SESEND_R = crate::BitReader<bool>;
#[doc = "Field `SESEND` writer - Session end"]
pub type SESEND_W<'a> = crate::BitWriter<'a, u32, GOTGINTF_SPEC, bool, 2>;
#[doc = "Field `SRPEND` reader - Session request success status change"]
pub type SRPEND_R = crate::BitReader<bool>;
#[doc = "Field `SRPEND` writer - Session request success status change"]
pub type SRPEND_W<'a> = crate::BitWriter<'a, u32, GOTGINTF_SPEC, bool, 8>;
#[doc = "Field `HNPEND` reader - HNP end"]
pub type HNPEND_R = crate::BitReader<bool>;
#[doc = "Field `HNPEND` writer - HNP end"]
pub type HNPEND_W<'a> = crate::BitWriter<'a, u32, GOTGINTF_SPEC, bool, 9>;
#[doc = "Field `HNPDET` reader - Host negotiation request detected"]
pub type HNPDET_R = crate::BitReader<bool>;
#[doc = "Field `HNPDET` writer - Host negotiation request detected"]
pub type HNPDET_W<'a> = crate::BitWriter<'a, u32, GOTGINTF_SPEC, bool, 17>;
#[doc = "Field `ADTO` reader - A-device timeout"]
pub type ADTO_R = crate::BitReader<bool>;
#[doc = "Field `ADTO` writer - A-device timeout"]
pub type ADTO_W<'a> = crate::BitWriter<'a, u32, GOTGINTF_SPEC, bool, 18>;
#[doc = "Field `DF` reader - Debounce finish"]
pub type DF_R = crate::BitReader<bool>;
#[doc = "Field `DF` writer - Debounce finish"]
pub type DF_W<'a> = crate::BitWriter<'a, u32, GOTGINTF_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    pub fn sesend(&self) -> SESEND_R {
        SESEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srpend(&self) -> SRPEND_R {
        SRPEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    pub fn hnpend(&self) -> HNPEND_R {
        HNPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host negotiation request detected"]
    #[inline(always)]
    pub fn hnpdet(&self) -> HNPDET_R {
        HNPDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    pub fn adto(&self) -> ADTO_R {
        ADTO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    pub fn df(&self) -> DF_R {
        DF_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    pub fn sesend(&mut self) -> SESEND_W {
        SESEND_W::new(self)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srpend(&mut self) -> SRPEND_W {
        SRPEND_W::new(self)
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    pub fn hnpend(&mut self) -> HNPEND_W {
        HNPEND_W::new(self)
    }
    #[doc = "Bit 17 - Host negotiation request detected"]
    #[inline(always)]
    pub fn hnpdet(&mut self) -> HNPDET_W {
        HNPDET_W::new(self)
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    pub fn adto(&mut self) -> ADTO_W {
        ADTO_W::new(self)
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    pub fn df(&mut self) -> DF_W {
        DF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgintf](index.html) module"]
pub struct GOTGINTF_SPEC;
impl crate::RegisterSpec for GOTGINTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgintf::R](R) reader structure"]
impl crate::Readable for GOTGINTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgintf::W](W) writer structure"]
impl crate::Writable for GOTGINTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GOTGINTF to value 0"]
impl crate::Resettable for GOTGINTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
