#[doc = "Register `DADDR` reader"]
pub struct R(crate::R<DADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADDR` writer"]
pub struct W(crate::W<DADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR_SPEC>;
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
impl From<crate::W<DADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBADDR` reader - USB device address"]
pub type USBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBADDR` writer - USB device address"]
pub type USBADDR_W<'a> = crate::FieldWriter<'a, u16, DADDR_SPEC, u8, u8, 7, 0>;
#[doc = "Field `USBEN` reader - USB device enable"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - USB device enable"]
pub type USBEN_W<'a> = crate::BitWriter<'a, u16, DADDR_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:6 - USB device address"]
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address"]
    #[inline(always)]
    pub fn usbaddr(&mut self) -> USBADDR_W {
        USBADDR_W::new(self)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr](index.html) module"]
pub struct DADDR_SPEC;
impl crate::RegisterSpec for DADDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [daddr::R](R) reader structure"]
impl crate::Readable for DADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daddr::W](W) writer structure"]
impl crate::Writable for DADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
