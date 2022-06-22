#[doc = "Register `DOEP2INTF` reader"]
pub struct R(crate::R<DOEP2INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP2INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP2INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP2INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP2INTF` writer"]
pub struct W(crate::W<DOEP2INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP2INTF_SPEC>;
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
impl From<crate::W<DOEP2INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP2INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTBSTP` reader - Back-to-back SETUP packets"]
pub type BTBSTP_R = crate::BitReader<bool>;
#[doc = "Field `BTBSTP` writer - Back-to-back SETUP packets"]
pub type BTBSTP_W<'a> = crate::BitWriter<'a, u32, DOEP2INTF_SPEC, bool, 6>;
#[doc = "Field `EPRXFOVR` reader - Endpoint Rx FIFO overrun"]
pub type EPRXFOVR_R = crate::BitReader<bool>;
#[doc = "Field `EPRXFOVR` writer - Endpoint Rx FIFO overrun"]
pub type EPRXFOVR_W<'a> = crate::BitWriter<'a, u32, DOEP2INTF_SPEC, bool, 4>;
#[doc = "Field `STPF` reader - Setup phase finished"]
pub type STPF_R = crate::BitReader<bool>;
#[doc = "Field `STPF` writer - Setup phase finished"]
pub type STPF_W<'a> = crate::BitWriter<'a, u32, DOEP2INTF_SPEC, bool, 3>;
#[doc = "Field `EPDIS` reader - Endpoint disabled"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` writer - Endpoint disabled"]
pub type EPDIS_W<'a> = crate::BitWriter<'a, u32, DOEP2INTF_SPEC, bool, 1>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader<bool>;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a> = crate::BitWriter<'a, u32, DOEP2INTF_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    pub fn btbstp(&self) -> BTBSTP_R {
        BTBSTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    pub fn eprxfovr(&self) -> EPRXFOVR_R {
        EPRXFOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    pub fn stpf(&self) -> STPF_R {
        STPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    pub fn btbstp(&mut self) -> BTBSTP_W {
        BTBSTP_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    pub fn eprxfovr(&mut self) -> EPRXFOVR_W {
        EPRXFOVR_W::new(self)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    pub fn stpf(&mut self) -> STPF_W {
        STPF_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&mut self) -> TF_W {
        TF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device out endpoint-2 interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2intf](index.html) module"]
pub struct DOEP2INTF_SPEC;
impl crate::RegisterSpec for DOEP2INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep2intf::R](R) reader structure"]
impl crate::Readable for DOEP2INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep2intf::W](W) writer structure"]
impl crate::Writable for DOEP2INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP2INTF to value 0"]
impl crate::Resettable for DOEP2INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
