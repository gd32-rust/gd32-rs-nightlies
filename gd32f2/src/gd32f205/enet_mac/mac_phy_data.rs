#[doc = "Register `MAC_PHY_DATA` reader"]
pub struct R(crate::R<MAC_PHY_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_PHY_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_PHY_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_PHY_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_PHY_DATA` writer"]
pub struct W(crate::W<MAC_PHY_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_PHY_DATA_SPEC>;
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
impl From<crate::W<MAC_PHY_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_PHY_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - PHY data"]
pub type PD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PD` writer - PHY data"]
pub type PD_W<'a> = crate::FieldWriter<'a, u32, MAC_PHY_DATA_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - PHY data"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY data"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MII data register (MAC_PHY_DATA)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_phy_data](index.html) module"]
pub struct MAC_PHY_DATA_SPEC;
impl crate::RegisterSpec for MAC_PHY_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_phy_data::R](R) reader structure"]
impl crate::Readable for MAC_PHY_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_phy_data::W](W) writer structure"]
impl crate::Writable for MAC_PHY_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_PHY_DATA to value 0"]
impl crate::Resettable for MAC_PHY_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
