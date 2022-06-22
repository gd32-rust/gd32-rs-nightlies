#[doc = "Register `SNWTCFG2` reader"]
pub struct R(crate::R<SNWTCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNWTCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNWTCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNWTCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNWTCFG2` writer"]
pub struct W(crate::W<SNWTCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNWTCFG2_SPEC>;
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
impl From<crate::W<SNWTCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNWTCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WASYNCMOD` reader - Asynchronous access mode"]
pub type WASYNCMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WASYNCMOD` writer - Asynchronous access mode"]
pub type WASYNCMOD_W<'a> = crate::FieldWriter<'a, u32, SNWTCFG2_SPEC, u8, u8, 2, 28>;
#[doc = "Field `DLAT` reader - Data latency for NOR Flash"]
pub type DLAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLAT` writer - Data latency for NOR Flash"]
pub type DLAT_W<'a> = crate::FieldWriter<'a, u32, SNWTCFG2_SPEC, u8, u8, 4, 24>;
#[doc = "Field `CKDIV` reader - Synchronous clock divide ratio"]
pub type CKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDIV` writer - Synchronous clock divide ratio"]
pub type CKDIV_W<'a> = crate::FieldWriter<'a, u32, SNWTCFG2_SPEC, u8, u8, 4, 20>;
#[doc = "Field `WDSET` reader - Data setup time"]
pub type WDSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDSET` writer - Data setup time"]
pub type WDSET_W<'a> = crate::FieldWriter<'a, u32, SNWTCFG2_SPEC, u8, u8, 8, 8>;
#[doc = "Field `WAHLD` reader - Address hold time"]
pub type WAHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAHLD` writer - Address hold time"]
pub type WAHLD_W<'a> = crate::FieldWriter<'a, u32, SNWTCFG2_SPEC, u8, u8, 4, 4>;
#[doc = "Field `WASET` reader - Address setup time"]
pub type WASET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WASET` writer - Address setup time"]
pub type WASET_W<'a> = crate::FieldWriter<'a, u32, SNWTCFG2_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&self) -> WASYNCMOD_R {
        WASYNCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&self) -> DLAT_R {
        DLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn wdset(&self) -> WDSET_R {
        WDSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn wahld(&self) -> WAHLD_R {
        WAHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn waset(&self) -> WASET_R {
        WASET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&mut self) -> WASYNCMOD_W {
        WASYNCMOD_W::new(self)
    }
    #[doc = "Bits 24:27 - Data latency for NOR Flash"]
    #[inline(always)]
    pub fn dlat(&mut self) -> DLAT_W {
        DLAT_W::new(self)
    }
    #[doc = "Bits 20:23 - Synchronous clock divide ratio"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn wdset(&mut self) -> WDSET_W {
        WDSET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn wahld(&mut self) -> WAHLD_W {
        WAHLD_W::new(self)
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn waset(&mut self) -> WASET_W {
        WASET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR flash write timing configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snwtcfg2](index.html) module"]
pub struct SNWTCFG2_SPEC;
impl crate::RegisterSpec for SNWTCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snwtcfg2::R](R) reader structure"]
impl crate::Readable for SNWTCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snwtcfg2::W](W) writer structure"]
impl crate::Writable for SNWTCFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNWTCFG2 to value 0x0fff_ffff"]
impl crate::Resettable for SNWTCFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
