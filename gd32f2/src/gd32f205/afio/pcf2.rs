#[doc = "Register `PCF2` reader"]
pub struct R(crate::R<PCF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCF2` writer"]
pub struct W(crate::W<PCF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCF2_SPEC>;
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
impl From<crate::W<PCF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PH01_REMAP` reader - PH0/PH1 remapping"]
pub type PH01_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `PH01_REMAP` writer - PH0/PH1 remapping"]
pub type PH01_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF2_SPEC, bool, 31>;
#[doc = "Field `DCI_HSYNC_REMAP` reader - DCI_HSYNC remapping"]
pub type DCI_HSYNC_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `DCI_HSYNC_REMAP` writer - DCI_HSYNC remapping"]
pub type DCI_HSYNC_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF2_SPEC, bool, 29>;
#[doc = "Field `DCI_D13_REMAP` reader - DCI_D13 remapping"]
pub type DCI_D13_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D13_REMAP` writer - DCI_D13 remapping"]
pub type DCI_D13_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 27>;
#[doc = "Field `DCI_D12_REMAP` reader - DCI_D12 remapping"]
pub type DCI_D12_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `DCI_D12_REMAP` writer - DCI_D12 remapping"]
pub type DCI_D12_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF2_SPEC, bool, 26>;
#[doc = "Field `DCI_D11_REMAP` reader - DCI_D11 remapping"]
pub type DCI_D11_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D11_REMAP` writer - DCI_D11 remapping"]
pub type DCI_D11_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 24>;
#[doc = "Field `DCI_D10_REMAP` reader - DCI_D10 remapping"]
pub type DCI_D10_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D10_REMAP` writer - DCI_D10 remapping"]
pub type DCI_D10_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 22>;
#[doc = "Field `DCI_D9_REMAP` reader - DCI_D9 remapping"]
pub type DCI_D9_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D9_REMAP` writer - DCI_D9 remapping"]
pub type DCI_D9_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 20>;
#[doc = "Field `DCI_D8_REMAP` reader - DCI_D8 remapping"]
pub type DCI_D8_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D8_REMAP` writer - DCI_D8 remapping"]
pub type DCI_D8_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 18>;
#[doc = "Field `DCI_D7_REMAP` reader - DCI_D7 remapping"]
pub type DCI_D7_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D7_REMAP` writer - DCI_D7 remapping"]
pub type DCI_D7_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 16>;
#[doc = "Field `DCI_D6_REMAP` reader - DCI_D6 remapping"]
pub type DCI_D6_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D6_REMAP` writer - DCI_D6 remapping"]
pub type DCI_D6_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 14>;
#[doc = "Field `DCI_D5_REMAP` reader - DCI_D5 remapping"]
pub type DCI_D5_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D5_REMAP` writer - DCI_D5 remapping"]
pub type DCI_D5_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 12>;
#[doc = "Field `DCI_D4_REMAP` reader - DCI_D4 remapping"]
pub type DCI_D4_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D4_REMAP` writer - DCI_D4 remapping"]
pub type DCI_D4_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 10>;
#[doc = "Field `DCI_D3_REMAP` reader - DCI_D3 remapping"]
pub type DCI_D3_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D3_REMAP` writer - DCI_D3 remapping"]
pub type DCI_D3_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 8>;
#[doc = "Field `DCI_D2_REMAP` reader - DCI_D2 remapping"]
pub type DCI_D2_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D2_REMAP` writer - DCI_D2 remapping"]
pub type DCI_D2_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 6>;
#[doc = "Field `DCI_D1_REMAP` reader - DCI_D1 remapping"]
pub type DCI_D1_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D1_REMAP` writer - DCI_D1 remapping"]
pub type DCI_D1_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 4>;
#[doc = "Field `DCI_D0_REMAP` reader - DCI_D0 remapping"]
pub type DCI_D0_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_D0_REMAP` writer - DCI_D0 remapping"]
pub type DCI_D0_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 2>;
#[doc = "Field `DCI_VSYNC_REMAP` reader - DCI_VSYNC remapping"]
pub type DCI_VSYNC_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCI_VSYNC_REMAP` writer - DCI_VSYNC remapping"]
pub type DCI_VSYNC_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCF2_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bit 31 - PH0/PH1 remapping"]
    #[inline(always)]
    pub fn ph01_remap(&self) -> PH01_REMAP_R {
        PH01_REMAP_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 29 - DCI_HSYNC remapping"]
    #[inline(always)]
    pub fn dci_hsync_remap(&self) -> DCI_HSYNC_REMAP_R {
        DCI_HSYNC_REMAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 27:28 - DCI_D13 remapping"]
    #[inline(always)]
    pub fn dci_d13_remap(&self) -> DCI_D13_REMAP_R {
        DCI_D13_REMAP_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 26 - DCI_D12 remapping"]
    #[inline(always)]
    pub fn dci_d12_remap(&self) -> DCI_D12_REMAP_R {
        DCI_D12_REMAP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 24:25 - DCI_D11 remapping"]
    #[inline(always)]
    pub fn dci_d11_remap(&self) -> DCI_D11_REMAP_R {
        DCI_D11_REMAP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - DCI_D10 remapping"]
    #[inline(always)]
    pub fn dci_d10_remap(&self) -> DCI_D10_REMAP_R {
        DCI_D10_REMAP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DCI_D9 remapping"]
    #[inline(always)]
    pub fn dci_d9_remap(&self) -> DCI_D9_REMAP_R {
        DCI_D9_REMAP_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - DCI_D8 remapping"]
    #[inline(always)]
    pub fn dci_d8_remap(&self) -> DCI_D8_REMAP_R {
        DCI_D8_REMAP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DCI_D7 remapping"]
    #[inline(always)]
    pub fn dci_d7_remap(&self) -> DCI_D7_REMAP_R {
        DCI_D7_REMAP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DCI_D6 remapping"]
    #[inline(always)]
    pub fn dci_d6_remap(&self) -> DCI_D6_REMAP_R {
        DCI_D6_REMAP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DCI_D5 remapping"]
    #[inline(always)]
    pub fn dci_d5_remap(&self) -> DCI_D5_REMAP_R {
        DCI_D5_REMAP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DCI_D4 remapping"]
    #[inline(always)]
    pub fn dci_d4_remap(&self) -> DCI_D4_REMAP_R {
        DCI_D4_REMAP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DCI_D3 remapping"]
    #[inline(always)]
    pub fn dci_d3_remap(&self) -> DCI_D3_REMAP_R {
        DCI_D3_REMAP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DCI_D2 remapping"]
    #[inline(always)]
    pub fn dci_d2_remap(&self) -> DCI_D2_REMAP_R {
        DCI_D2_REMAP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DCI_D1 remapping"]
    #[inline(always)]
    pub fn dci_d1_remap(&self) -> DCI_D1_REMAP_R {
        DCI_D1_REMAP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - DCI_D0 remapping"]
    #[inline(always)]
    pub fn dci_d0_remap(&self) -> DCI_D0_REMAP_R {
        DCI_D0_REMAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - DCI_VSYNC remapping"]
    #[inline(always)]
    pub fn dci_vsync_remap(&self) -> DCI_VSYNC_REMAP_R {
        DCI_VSYNC_REMAP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - PH0/PH1 remapping"]
    #[inline(always)]
    pub fn ph01_remap(&mut self) -> PH01_REMAP_W {
        PH01_REMAP_W::new(self)
    }
    #[doc = "Bit 29 - DCI_HSYNC remapping"]
    #[inline(always)]
    pub fn dci_hsync_remap(&mut self) -> DCI_HSYNC_REMAP_W {
        DCI_HSYNC_REMAP_W::new(self)
    }
    #[doc = "Bits 27:28 - DCI_D13 remapping"]
    #[inline(always)]
    pub fn dci_d13_remap(&mut self) -> DCI_D13_REMAP_W {
        DCI_D13_REMAP_W::new(self)
    }
    #[doc = "Bit 26 - DCI_D12 remapping"]
    #[inline(always)]
    pub fn dci_d12_remap(&mut self) -> DCI_D12_REMAP_W {
        DCI_D12_REMAP_W::new(self)
    }
    #[doc = "Bits 24:25 - DCI_D11 remapping"]
    #[inline(always)]
    pub fn dci_d11_remap(&mut self) -> DCI_D11_REMAP_W {
        DCI_D11_REMAP_W::new(self)
    }
    #[doc = "Bits 22:23 - DCI_D10 remapping"]
    #[inline(always)]
    pub fn dci_d10_remap(&mut self) -> DCI_D10_REMAP_W {
        DCI_D10_REMAP_W::new(self)
    }
    #[doc = "Bits 20:21 - DCI_D9 remapping"]
    #[inline(always)]
    pub fn dci_d9_remap(&mut self) -> DCI_D9_REMAP_W {
        DCI_D9_REMAP_W::new(self)
    }
    #[doc = "Bits 18:19 - DCI_D8 remapping"]
    #[inline(always)]
    pub fn dci_d8_remap(&mut self) -> DCI_D8_REMAP_W {
        DCI_D8_REMAP_W::new(self)
    }
    #[doc = "Bits 16:17 - DCI_D7 remapping"]
    #[inline(always)]
    pub fn dci_d7_remap(&mut self) -> DCI_D7_REMAP_W {
        DCI_D7_REMAP_W::new(self)
    }
    #[doc = "Bits 14:15 - DCI_D6 remapping"]
    #[inline(always)]
    pub fn dci_d6_remap(&mut self) -> DCI_D6_REMAP_W {
        DCI_D6_REMAP_W::new(self)
    }
    #[doc = "Bits 12:13 - DCI_D5 remapping"]
    #[inline(always)]
    pub fn dci_d5_remap(&mut self) -> DCI_D5_REMAP_W {
        DCI_D5_REMAP_W::new(self)
    }
    #[doc = "Bits 10:11 - DCI_D4 remapping"]
    #[inline(always)]
    pub fn dci_d4_remap(&mut self) -> DCI_D4_REMAP_W {
        DCI_D4_REMAP_W::new(self)
    }
    #[doc = "Bits 8:9 - DCI_D3 remapping"]
    #[inline(always)]
    pub fn dci_d3_remap(&mut self) -> DCI_D3_REMAP_W {
        DCI_D3_REMAP_W::new(self)
    }
    #[doc = "Bits 6:7 - DCI_D2 remapping"]
    #[inline(always)]
    pub fn dci_d2_remap(&mut self) -> DCI_D2_REMAP_W {
        DCI_D2_REMAP_W::new(self)
    }
    #[doc = "Bits 4:5 - DCI_D1 remapping"]
    #[inline(always)]
    pub fn dci_d1_remap(&mut self) -> DCI_D1_REMAP_W {
        DCI_D1_REMAP_W::new(self)
    }
    #[doc = "Bits 2:3 - DCI_D0 remapping"]
    #[inline(always)]
    pub fn dci_d0_remap(&mut self) -> DCI_D0_REMAP_W {
        DCI_D0_REMAP_W::new(self)
    }
    #[doc = "Bits 0:1 - DCI_VSYNC remapping"]
    #[inline(always)]
    pub fn dci_vsync_remap(&mut self) -> DCI_VSYNC_REMAP_W {
        DCI_VSYNC_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO port configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf2](index.html) module"]
pub struct PCF2_SPEC;
impl crate::RegisterSpec for PCF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcf2::R](R) reader structure"]
impl crate::Readable for PCF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcf2::W](W) writer structure"]
impl crate::Writable for PCF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCF2 to value 0"]
impl crate::Resettable for PCF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
