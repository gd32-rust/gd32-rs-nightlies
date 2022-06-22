#[doc = "Register `MAC_VLT` reader"]
pub struct R(crate::R<MAC_VLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_VLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_VLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_VLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_VLT` writer"]
pub struct W(crate::W<MAC_VLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_VLT_SPEC>;
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
impl From<crate::W<MAC_VLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_VLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLTI` reader - VLAN tag identifier (for receive frames)"]
pub type VLTI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLTI` writer - VLAN tag identifier (for receive frames)"]
pub type VLTI_W<'a> = crate::FieldWriter<'a, u32, MAC_VLT_SPEC, u16, u16, 16, 0>;
#[doc = "Field `VLTC` reader - 12-bit VLAN tag comparison"]
pub type VLTC_R = crate::BitReader<bool>;
#[doc = "Field `VLTC` writer - 12-bit VLAN tag comparison"]
pub type VLTC_W<'a> = crate::BitWriter<'a, u32, MAC_VLT_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vltc(&self) -> VLTC_R {
        VLTC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlti(&mut self) -> VLTI_W {
        VLTI_W::new(self)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vltc(&mut self) -> VLTC_W {
        VLTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN tag register (MAC_VLT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_vlt](index.html) module"]
pub struct MAC_VLT_SPEC;
impl crate::RegisterSpec for MAC_VLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_vlt::R](R) reader structure"]
impl crate::Readable for MAC_VLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_vlt::W](W) writer structure"]
impl crate::Writable for MAC_VLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_VLT to value 0"]
impl crate::Resettable for MAC_VLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
