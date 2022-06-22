#[doc = "Register `MAC_ADDR3H` reader"]
pub struct R(crate::R<MAC_ADDR3H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR3H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR3H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR3H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR3H` writer"]
pub struct W(crate::W<MAC_ADDR3H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR3H_SPEC>;
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
impl From<crate::W<MAC_ADDR3H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR3H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR3H` reader - MAC address3 high"]
pub type ADDR3H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR3H` writer - MAC address3 high"]
pub type ADDR3H_W<'a> = crate::FieldWriter<'a, u32, MAC_ADDR3H_SPEC, u16, u16, 16, 0>;
#[doc = "Field `MB` reader - Mask byte"]
pub type MB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MB` writer - Mask byte"]
pub type MB_W<'a> = crate::FieldWriter<'a, u32, MAC_ADDR3H_SPEC, u8, u8, 6, 24>;
#[doc = "Field `SAF` reader - Source address filter"]
pub type SAF_R = crate::BitReader<bool>;
#[doc = "Field `SAF` writer - Source address filter"]
pub type SAF_W<'a> = crate::BitWriter<'a, u32, MAC_ADDR3H_SPEC, bool, 30>;
#[doc = "Field `AFE` reader - Address filter enable"]
pub type AFE_R = crate::BitReader<bool>;
#[doc = "Field `AFE` writer - Address filter enable"]
pub type AFE_W<'a> = crate::BitWriter<'a, u32, MAC_ADDR3H_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    pub fn addr3h(&self) -> ADDR3H_R {
        ADDR3H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address filter enable"]
    #[inline(always)]
    pub fn afe(&self) -> AFE_R {
        AFE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    pub fn addr3h(&mut self) -> ADDR3H_W {
        ADDR3H_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask byte"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W::new(self)
    }
    #[doc = "Bit 30 - Source address filter"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W {
        SAF_W::new(self)
    }
    #[doc = "Bit 31 - Address filter enable"]
    #[inline(always)]
    pub fn afe(&mut self) -> AFE_W {
        AFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 3 high register (MAC_ADDR3H)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr3h](index.html) module"]
pub struct MAC_ADDR3H_SPEC;
impl crate::RegisterSpec for MAC_ADDR3H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr3h::R](R) reader structure"]
impl crate::Readable for MAC_ADDR3H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr3h::W](W) writer structure"]
impl crate::Writable for MAC_ADDR3H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR3H to value 0xffff"]
impl crate::Resettable for MAC_ADDR3H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
