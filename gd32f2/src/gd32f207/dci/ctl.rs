#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCIEN` reader - DCI Enable"]
pub type DCIEN_R = crate::BitReader<bool>;
#[doc = "Field `DCIEN` writer - DCI Enable"]
pub type DCIEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 14>;
#[doc = "Field `DCIF` reader - Digital camera interface format"]
pub type DCIF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCIF` writer - Digital camera interface format"]
pub type DCIF_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 10>;
#[doc = "Field `FR` reader - Frame rate"]
pub type FR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FR` writer - Frame rate"]
pub type FR_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `VPS` reader - Vertical Polarity Selection"]
pub type VPS_R = crate::BitReader<bool>;
#[doc = "Field `VPS` writer - Vertical Polarity Selection"]
pub type VPS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 7>;
#[doc = "Field `HPS` reader - Horizontal Polarity Selection"]
pub type HPS_R = crate::BitReader<bool>;
#[doc = "Field `HPS` writer - Horizontal Polarity Selection"]
pub type HPS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 6>;
#[doc = "Field `CKS` reader - Clock Polarity Selection"]
pub type CKS_R = crate::BitReader<bool>;
#[doc = "Field `CKS` writer - Clock Polarity Selection"]
pub type CKS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 5>;
#[doc = "Field `ESM` reader - Embedded synchronous mode"]
pub type ESM_R = crate::BitReader<bool>;
#[doc = "Field `ESM` writer - Embedded synchronous mode"]
pub type ESM_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 4>;
#[doc = "Field `JM` reader - JPEG mode"]
pub type JM_R = crate::BitReader<bool>;
#[doc = "Field `JM` writer - JPEG mode"]
pub type JM_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
#[doc = "Field `WDEN` reader - Window Enable"]
pub type WDEN_R = crate::BitReader<bool>;
#[doc = "Field `WDEN` writer - Window Enable"]
pub type WDEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
#[doc = "Field `SNAP` reader - Snapshot mode"]
pub type SNAP_R = crate::BitReader<bool>;
#[doc = "Field `SNAP` writer - Snapshot mode"]
pub type SNAP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `CAP` reader - Capture Enable"]
pub type CAP_R = crate::BitReader<bool>;
#[doc = "Field `CAP` writer - Capture Enable"]
pub type CAP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 14 - DCI Enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DCIEN_R {
        DCIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Digital camera interface format"]
    #[inline(always)]
    pub fn dcif(&self) -> DCIF_R {
        DCIF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Frame rate"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Vertical Polarity Selection"]
    #[inline(always)]
    pub fn vps(&self) -> VPS_R {
        VPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal Polarity Selection"]
    #[inline(always)]
    pub fn hps(&self) -> HPS_R {
        HPS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Embedded synchronous mode"]
    #[inline(always)]
    pub fn esm(&self) -> ESM_R {
        ESM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG mode"]
    #[inline(always)]
    pub fn jm(&self) -> JM_R {
        JM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Enable"]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Snapshot mode"]
    #[inline(always)]
    pub fn snap(&self) -> SNAP_R {
        SNAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Capture Enable"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - DCI Enable"]
    #[inline(always)]
    pub fn dcien(&mut self) -> DCIEN_W {
        DCIEN_W::new(self)
    }
    #[doc = "Bits 10:11 - Digital camera interface format"]
    #[inline(always)]
    pub fn dcif(&mut self) -> DCIF_W {
        DCIF_W::new(self)
    }
    #[doc = "Bits 8:9 - Frame rate"]
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W {
        FR_W::new(self)
    }
    #[doc = "Bit 7 - Vertical Polarity Selection"]
    #[inline(always)]
    pub fn vps(&mut self) -> VPS_W {
        VPS_W::new(self)
    }
    #[doc = "Bit 6 - Horizontal Polarity Selection"]
    #[inline(always)]
    pub fn hps(&mut self) -> HPS_W {
        HPS_W::new(self)
    }
    #[doc = "Bit 5 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W {
        CKS_W::new(self)
    }
    #[doc = "Bit 4 - Embedded synchronous mode"]
    #[inline(always)]
    pub fn esm(&mut self) -> ESM_W {
        ESM_W::new(self)
    }
    #[doc = "Bit 3 - JPEG mode"]
    #[inline(always)]
    pub fn jm(&mut self) -> JM_W {
        JM_W::new(self)
    }
    #[doc = "Bit 2 - Window Enable"]
    #[inline(always)]
    pub fn wden(&mut self) -> WDEN_W {
        WDEN_W::new(self)
    }
    #[doc = "Bit 1 - Snapshot mode"]
    #[inline(always)]
    pub fn snap(&mut self) -> SNAP_W {
        SNAP_W::new(self)
    }
    #[doc = "Bit 0 - Capture Enable"]
    #[inline(always)]
    pub fn cap(&mut self) -> CAP_W {
        CAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCI Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
