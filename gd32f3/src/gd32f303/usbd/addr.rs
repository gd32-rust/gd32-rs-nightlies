#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBDAR` reader - Device address"]
pub type USBDAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBDAR` writer - Device address"]
pub type USBDAR_W<'a> = crate::FieldWriter<'a, u32, ADDR_SPEC, u8, u8, 7, 0>;
#[doc = "Field `USBEN` reader - USB device enable"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - USB device enable"]
pub type USBEN_W<'a> = crate::BitWriter<'a, u32, ADDR_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn usbdar(&self) -> USBDAR_R {
        USBDAR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn usbdar(&mut self) -> USBDAR_W {
        USBDAR_W::new(self)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
