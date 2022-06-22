#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKDIV` reader - Clock division"]
pub type CKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDIV` writer - Clock division"]
pub type CKDIV_W<'a> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, 8>;
#[doc = "Field `ARSE` reader - Auto-reload preload enable"]
pub type ARSE_R = crate::BitReader<bool>;
#[doc = "Field `ARSE` writer - Auto-reload preload enable"]
pub type ARSE_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 7>;
#[doc = "Field `CAM` reader - Center-aligned mode selection"]
pub type CAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAM` writer - Center-aligned mode selection"]
pub type CAM_W<'a> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, 5>;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 4>;
#[doc = "Field `SPM` reader - One-pulse mode"]
pub type SPM_R = crate::BitReader<bool>;
#[doc = "Field `SPM` writer - One-pulse mode"]
pub type SPM_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 3>;
#[doc = "Field `UPS` reader - Update request source"]
pub type UPS_R = crate::BitReader<bool>;
#[doc = "Field `UPS` writer - Update request source"]
pub type UPS_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 2>;
#[doc = "Field `UPDIS` reader - Update disable"]
pub type UPDIS_R = crate::BitReader<bool>;
#[doc = "Field `UPDIS` writer - Update disable"]
pub type UPDIS_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 1>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arse(&self) -> ARSE_R {
        ARSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cam(&self) -> CAM_R {
        CAM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SPM_R {
        SPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn ups(&self) -> UPS_R {
        UPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UPDIS_R {
        UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arse(&mut self) -> ARSE_W {
        ARSE_W::new(self)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cam(&mut self) -> CAM_W {
        CAM_W::new(self)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn spm(&mut self) -> SPM_W {
        SPM_W::new(self)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn ups(&mut self) -> UPS_W {
        UPS_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&mut self) -> UPDIS_W {
        UPDIS_W::new(self)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
