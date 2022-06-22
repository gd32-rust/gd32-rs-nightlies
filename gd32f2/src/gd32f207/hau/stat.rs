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
#[doc = "Field `BUSY` reader - Busy flag bit"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `DMAS` reader - DMA status flag"]
pub type DMAS_R = crate::BitReader<bool>;
#[doc = "Field `CINT` reader - Digest calculation completion interrupt flag"]
pub type CINT_R = crate::BitReader<bool>;
#[doc = "Field `CINT` writer - Digest calculation completion interrupt flag"]
pub type CINT_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 1>;
#[doc = "Field `DINT` reader - Data input interrupt status flag"]
pub type DINT_R = crate::BitReader<bool>;
#[doc = "Field `DINT` writer - Data input interrupt status flag"]
pub type DINT_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 3 - Busy flag bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA status flag"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt flag"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Data input interrupt status flag"]
    #[inline(always)]
    pub fn dint(&self) -> DINT_R {
        DINT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Digest calculation completion interrupt flag"]
    #[inline(always)]
    pub fn cint(&mut self) -> CINT_W {
        CINT_W::new(self)
    }
    #[doc = "Bit 0 - Data input interrupt status flag"]
    #[inline(always)]
    pub fn dint(&mut self) -> DINT_W {
        DINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HAU status and interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
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
#[doc = "`reset()` method sets STAT to value 0x01"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
