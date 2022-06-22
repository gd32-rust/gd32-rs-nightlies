#[doc = "Register `CTL3` reader"]
pub struct R(crate::R<CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL3` writer"]
pub struct W(crate::W<CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL3_SPEC>;
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
impl From<crate::W<CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSBF` reader - Most significant bit first"]
pub type MSBF_R = crate::BitReader<bool>;
#[doc = "Field `MSBF` writer - Most significant bit first"]
pub type MSBF_W<'a> = crate::BitWriter<'a, u32, CTL3_SPEC, bool, 11>;
#[doc = "Field `DINV` reader - Data bit level inversion"]
pub type DINV_R = crate::BitReader<bool>;
#[doc = "Field `DINV` writer - Data bit level inversion"]
pub type DINV_W<'a> = crate::BitWriter<'a, u32, CTL3_SPEC, bool, 10>;
#[doc = "Field `TINV` reader - TX pin level inversion"]
pub type TINV_R = crate::BitReader<bool>;
#[doc = "Field `TINV` writer - TX pin level inversion"]
pub type TINV_W<'a> = crate::BitWriter<'a, u32, CTL3_SPEC, bool, 9>;
#[doc = "Field `RINV` reader - RX pin level inversion"]
pub type RINV_R = crate::BitReader<bool>;
#[doc = "Field `RINV` writer - RX pin level inversion"]
pub type RINV_W<'a> = crate::BitWriter<'a, u32, CTL3_SPEC, bool, 8>;
#[doc = "Field `EBIE` reader - Interrupt enable bit of end of block event"]
pub type EBIE_R = crate::BitReader<bool>;
#[doc = "Field `EBIE` writer - Interrupt enable bit of end of block event"]
pub type EBIE_W<'a> = crate::BitWriter<'a, u32, CTL3_SPEC, bool, 5>;
#[doc = "Field `RTIE` reader - Interrupt enable bit of receive timeout event"]
pub type RTIE_R = crate::BitReader<bool>;
#[doc = "Field `RTIE` writer - Interrupt enable bit of receive timeout event"]
pub type RTIE_W<'a> = crate::BitWriter<'a, u32, CTL3_SPEC, bool, 4>;
#[doc = "Field `SCRTNUM` reader - Smartcard auto-retry number"]
pub type SCRTNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCRTNUM` writer - Smartcard auto-retry number"]
pub type SCRTNUM_W<'a> = crate::FieldWriter<'a, u32, CTL3_SPEC, u8, u8, 3, 1>;
#[doc = "Field `RTEN` reader - Receiver timeout enable"]
pub type RTEN_R = crate::BitReader<bool>;
#[doc = "Field `RTEN` writer - Receiver timeout enable"]
pub type RTEN_W<'a> = crate::BitWriter<'a, u32, CTL3_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 11 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&self) -> DINV_R {
        DINV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable bit of end of block event"]
    #[inline(always)]
    pub fn ebie(&self) -> EBIE_R {
        EBIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable bit of receive timeout event"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 1:3 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&self) -> SCRTNUM_R {
        SCRTNUM_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 0 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&self) -> RTEN_R {
        RTEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W::new(self)
    }
    #[doc = "Bit 10 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&mut self) -> DINV_W {
        DINV_W::new(self)
    }
    #[doc = "Bit 9 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&mut self) -> TINV_W {
        TINV_W::new(self)
    }
    #[doc = "Bit 8 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&mut self) -> RINV_W {
        RINV_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable bit of end of block event"]
    #[inline(always)]
    pub fn ebie(&mut self) -> EBIE_W {
        EBIE_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable bit of receive timeout event"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W::new(self)
    }
    #[doc = "Bits 1:3 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&mut self) -> SCRTNUM_W {
        SCRTNUM_W::new(self)
    }
    #[doc = "Bit 0 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&mut self) -> RTEN_W {
        RTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl3](index.html) module"]
pub struct CTL3_SPEC;
impl crate::RegisterSpec for CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl3::R](R) reader structure"]
impl crate::Readable for CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl3::W](W) writer structure"]
impl crate::Writable for CTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL3 to value 0"]
impl crate::Resettable for CTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
