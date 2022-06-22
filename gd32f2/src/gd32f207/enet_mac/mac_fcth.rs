#[doc = "Register `MAC_FCTH` reader"]
pub struct R(crate::R<MAC_FCTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_FCTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_FCTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_FCTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_FCTH` writer"]
pub struct W(crate::W<MAC_FCTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_FCTH_SPEC>;
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
impl From<crate::W<MAC_FCTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_FCTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFA` reader - Threshold of active flow control"]
pub type RFA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFA` writer - Threshold of active flow control"]
pub type RFA_W<'a> = crate::FieldWriter<'a, u32, MAC_FCTH_SPEC, u8, u8, 3, 0>;
#[doc = "Field `RFD` reader - Threshold of deactive flow control"]
pub type RFD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFD` writer - Threshold of deactive flow control"]
pub type RFD_W<'a> = crate::FieldWriter<'a, u32, MAC_FCTH_SPEC, u8, u8, 3, 4>;
impl R {
    #[doc = "Bits 0:2 - Threshold of active flow control"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Threshold of deactive flow control"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Threshold of active flow control"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RFA_W {
        RFA_W::new(self)
    }
    #[doc = "Bits 4:6 - Threshold of deactive flow control"]
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W {
        RFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC flow control threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_fcth](index.html) module"]
pub struct MAC_FCTH_SPEC;
impl crate::RegisterSpec for MAC_FCTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_fcth::R](R) reader structure"]
impl crate::Readable for MAC_FCTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_fcth::W](W) writer structure"]
impl crate::Writable for MAC_FCTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_FCTH to value 0x15"]
impl crate::Resettable for MAC_FCTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
