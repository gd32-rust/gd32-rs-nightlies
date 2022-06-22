#[doc = "Register `SNTCFG1` reader"]
pub struct R(crate::R<SNTCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNTCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNTCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNTCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNTCFG1` writer"]
pub struct W(crate::W<SNTCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNTCFG1_SPEC>;
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
impl From<crate::W<SNTCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNTCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCMOD` reader - Asynchronous access mode"]
pub type ASYNCMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASYNCMOD` writer - Asynchronous access mode"]
pub type ASYNCMOD_W<'a> = crate::FieldWriter<'a, u32, SNTCFG1_SPEC, u8, u8, 2, 28>;
#[doc = "Field `DLAT` reader - Data latency for NOR Flash"]
pub type DLAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLAT` writer - Data latency for NOR Flash"]
pub type DLAT_W<'a> = crate::FieldWriter<'a, u32, SNTCFG1_SPEC, u8, u8, 4, 24>;
#[doc = "Field `CKDIV` reader - Synchronous clock divide ratio"]
pub type CKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDIV` writer - Synchronous clock divide ratio"]
pub type CKDIV_W<'a> = crate::FieldWriter<'a, u32, SNTCFG1_SPEC, u8, u8, 4, 20>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BUSLAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BUSLAT_W<'a> = crate::FieldWriter<'a, u32, SNTCFG1_SPEC, u8, u8, 4, 16>;
#[doc = "Field `DSET` reader - Data setup time"]
pub type DSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSET` writer - Data setup time"]
pub type DSET_W<'a> = crate::FieldWriter<'a, u32, SNTCFG1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `AHLD` reader - Address hold time"]
pub type AHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHLD` writer - Address hold time"]
pub type AHLD_W<'a> = crate::FieldWriter<'a, u32, SNTCFG1_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ASET` reader - Address setup time"]
pub type ASET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASET` writer - Address setup time"]
pub type ASET_W<'a> = crate::FieldWriter<'a, u32, SNTCFG1_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn asyncmod(&self) -> ASYNCMOD_R {
        ASYNCMOD_R::new(((self.bits >> 28) & 3) as u8)
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
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BUSLAT_R {
        BUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&self) -> DSET_R {
        DSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&self) -> AHLD_R {
        AHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&self) -> ASET_R {
        ASET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn asyncmod(&mut self) -> ASYNCMOD_W {
        ASYNCMOD_W::new(self)
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
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&mut self) -> BUSLAT_W {
        BUSLAT_W::new(self)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&mut self) -> DSET_W {
        DSET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&mut self) -> AHLD_W {
        AHLD_W::new(self)
    }
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&mut self) -> ASET_W {
        ASET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR flash timing configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sntcfg1](index.html) module"]
pub struct SNTCFG1_SPEC;
impl crate::RegisterSpec for SNTCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sntcfg1::R](R) reader structure"]
impl crate::Readable for SNTCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sntcfg1::W](W) writer structure"]
impl crate::Writable for SNTCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNTCFG1 to value 0x0fff_ffff"]
impl crate::Resettable for SNTCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
