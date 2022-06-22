#[doc = "Register `MAC_ADDR0H` reader"]
pub struct R(crate::R<MAC_ADDR0H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR0H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR0H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR0H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR0H` writer"]
pub struct W(crate::W<MAC_ADDR0H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR0H_SPEC>;
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
impl From<crate::W<MAC_ADDR0H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR0H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0H` reader - MAC address0 high"]
pub type ADDR0H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR0H` writer - MAC address0 high"]
pub type ADDR0H_W<'a> = crate::FieldWriter<'a, u32, MAC_ADDR0H_SPEC, u16, u16, 16, 0>;
#[doc = "Field `MO` reader - Always 1"]
pub type MO_R = crate::BitReader<bool>;
#[doc = "Field `MO` writer - Always 1"]
pub type MO_W<'a> = crate::BitWriter<'a, u32, MAC_ADDR0H_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn addr0h(&self) -> ADDR0H_R {
        ADDR0H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Always 1"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn addr0h(&mut self) -> ADDR0H_W {
        ADDR0H_W::new(self)
    }
    #[doc = "Bit 31 - Always 1"]
    #[inline(always)]
    pub fn mo(&mut self) -> MO_W {
        MO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 0 high register (MAC_ADDR0H)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr0h](index.html) module"]
pub struct MAC_ADDR0H_SPEC;
impl crate::RegisterSpec for MAC_ADDR0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr0h::R](R) reader structure"]
impl crate::Readable for MAC_ADDR0H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr0h::W](W) writer structure"]
impl crate::Writable for MAC_ADDR0H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR0H to value 0x8000_ffff"]
impl crate::Resettable for MAC_ADDR0H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_ffff
    }
}
