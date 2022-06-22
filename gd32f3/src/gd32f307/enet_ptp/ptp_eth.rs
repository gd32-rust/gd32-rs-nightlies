#[doc = "Register `PTP_ETH` reader"]
pub struct R(crate::R<PTP_ETH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTP_ETH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTP_ETH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTP_ETH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTP_ETH` writer"]
pub struct W(crate::W<PTP_ETH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTP_ETH_SPEC>;
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
impl From<crate::W<PTP_ETH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTP_ETH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETSH` reader - Expected time stamp high"]
pub type ETSH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ETSH` writer - Expected time stamp high"]
pub type ETSH_W<'a> = crate::FieldWriter<'a, u32, PTP_ETH_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Expected time stamp high"]
    #[inline(always)]
    pub fn etsh(&self) -> ETSH_R {
        ETSH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Expected time stamp high"]
    #[inline(always)]
    pub fn etsh(&mut self) -> ETSH_W {
        ETSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP expected time high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptp_eth](index.html) module"]
pub struct PTP_ETH_SPEC;
impl crate::RegisterSpec for PTP_ETH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptp_eth::R](R) reader structure"]
impl crate::Readable for PTP_ETH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptp_eth::W](W) writer structure"]
impl crate::Writable for PTP_ETH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTP_ETH to value 0"]
impl crate::Resettable for PTP_ETH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
