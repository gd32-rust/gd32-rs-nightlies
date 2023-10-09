#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `CAP` reader - Capture Enable"]
pub type CAP_R = crate::BitReader;
#[doc = "Field `CAP` writer - Capture Enable"]
pub type CAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNAP` reader - Snapshot mode"]
pub type SNAP_R = crate::BitReader;
#[doc = "Field `SNAP` writer - Snapshot mode"]
pub type SNAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDEN` reader - Window Enable"]
pub type WDEN_R = crate::BitReader;
#[doc = "Field `WDEN` writer - Window Enable"]
pub type WDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JM` reader - JPEG mode"]
pub type JM_R = crate::BitReader;
#[doc = "Field `JM` writer - JPEG mode"]
pub type JM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESM` reader - Embedded synchronous mode"]
pub type ESM_R = crate::BitReader;
#[doc = "Field `ESM` writer - Embedded synchronous mode"]
pub type ESM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKS` reader - Clock Polarity Selection"]
pub type CKS_R = crate::BitReader;
#[doc = "Field `CKS` writer - Clock Polarity Selection"]
pub type CKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPS` reader - Horizontal Polarity Selection"]
pub type HPS_R = crate::BitReader;
#[doc = "Field `HPS` writer - Horizontal Polarity Selection"]
pub type HPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VPS` reader - Vertical Polarity Selection"]
pub type VPS_R = crate::BitReader;
#[doc = "Field `VPS` writer - Vertical Polarity Selection"]
pub type VPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FR` reader - Frame rate"]
pub type FR_R = crate::FieldReader;
#[doc = "Field `FR` writer - Frame rate"]
pub type FR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DCIF` reader - Digital camera interface format"]
pub type DCIF_R = crate::FieldReader;
#[doc = "Field `DCIF` writer - Digital camera interface format"]
pub type DCIF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DCIEN` reader - DCI Enable"]
pub type DCIEN_R = crate::BitReader;
#[doc = "Field `DCIEN` writer - DCI Enable"]
pub type DCIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Capture Enable"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Snapshot mode"]
    #[inline(always)]
    pub fn snap(&self) -> SNAP_R {
        SNAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Enable"]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG mode"]
    #[inline(always)]
    pub fn jm(&self) -> JM_R {
        JM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Embedded synchronous mode"]
    #[inline(always)]
    pub fn esm(&self) -> ESM_R {
        ESM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal Polarity Selection"]
    #[inline(always)]
    pub fn hps(&self) -> HPS_R {
        HPS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Vertical Polarity Selection"]
    #[inline(always)]
    pub fn vps(&self) -> VPS_R {
        VPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Frame rate"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Digital camera interface format"]
    #[inline(always)]
    pub fn dcif(&self) -> DCIF_R {
        DCIF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - DCI Enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DCIEN_R {
        DCIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CAP_W<CTL_SPEC, 0> {
        CAP_W::new(self)
    }
    #[doc = "Bit 1 - Snapshot mode"]
    #[inline(always)]
    #[must_use]
    pub fn snap(&mut self) -> SNAP_W<CTL_SPEC, 1> {
        SNAP_W::new(self)
    }
    #[doc = "Bit 2 - Window Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wden(&mut self) -> WDEN_W<CTL_SPEC, 2> {
        WDEN_W::new(self)
    }
    #[doc = "Bit 3 - JPEG mode"]
    #[inline(always)]
    #[must_use]
    pub fn jm(&mut self) -> JM_W<CTL_SPEC, 3> {
        JM_W::new(self)
    }
    #[doc = "Bit 4 - Embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn esm(&mut self) -> ESM_W<CTL_SPEC, 4> {
        ESM_W::new(self)
    }
    #[doc = "Bit 5 - Clock Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<CTL_SPEC, 5> {
        CKS_W::new(self)
    }
    #[doc = "Bit 6 - Horizontal Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn hps(&mut self) -> HPS_W<CTL_SPEC, 6> {
        HPS_W::new(self)
    }
    #[doc = "Bit 7 - Vertical Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vps(&mut self) -> VPS_W<CTL_SPEC, 7> {
        VPS_W::new(self)
    }
    #[doc = "Bits 8:9 - Frame rate"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FR_W<CTL_SPEC, 8> {
        FR_W::new(self)
    }
    #[doc = "Bits 10:11 - Digital camera interface format"]
    #[inline(always)]
    #[must_use]
    pub fn dcif(&mut self) -> DCIF_W<CTL_SPEC, 10> {
        DCIF_W::new(self)
    }
    #[doc = "Bit 14 - DCI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcien(&mut self) -> DCIEN_W<CTL_SPEC, 14> {
        DCIEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DCI Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
