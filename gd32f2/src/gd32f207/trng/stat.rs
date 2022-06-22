#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEIF` reader - Seed error interrupt flag"]
pub type SEIF_R = crate::BitReader<bool>;
#[doc = "Field `SEIF` writer - Seed error interrupt flag"]
pub type SEIF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 6>;
#[doc = "Field `CEIF` reader - Clock error interrupt flag"]
pub type CEIF_R = crate::BitReader<bool>;
#[doc = "Field `CEIF` writer - Clock error interrupt flag"]
pub type CEIF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 5>;
#[doc = "Field `SECS` reader - Seed error current status"]
pub type SECS_R = crate::BitReader<bool>;
#[doc = "Field `CECS` reader - Clock error current status"]
pub type CECS_R = crate::BitReader<bool>;
#[doc = "Field `DRDY` reader - Random data ready status bit"]
pub type DRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    pub fn ceif(&self) -> CEIF_R {
        CEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status"]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Random data ready status bit"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Seed error interrupt flag"]
    #[inline(always)]
    pub fn seif(&mut self) -> SEIF_W {
        SEIF_W::new(self)
    }
    #[doc = "Bit 5 - Clock error interrupt flag"]
    #[inline(always)]
    pub fn ceif(&mut self) -> CEIF_W {
        CEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
