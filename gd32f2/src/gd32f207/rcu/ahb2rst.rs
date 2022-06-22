#[doc = "Register `AHB2RST` reader"]
pub struct R(crate::R<AHB2RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2RST` writer"]
pub struct W(crate::W<AHB2RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RST_SPEC>;
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
impl From<crate::W<AHB2RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCIRST` reader - DCI reset"]
pub type DCIRST_R = crate::BitReader<bool>;
#[doc = "Field `DCIRST` writer - DCI reset"]
pub type DCIRST_W<'a> = crate::BitWriter<'a, u32, AHB2RST_SPEC, bool, 0>;
#[doc = "Field `CAURST` reader - CAU reset"]
pub type CAURST_R = crate::BitReader<bool>;
#[doc = "Field `CAURST` writer - CAU reset"]
pub type CAURST_W<'a> = crate::BitWriter<'a, u32, AHB2RST_SPEC, bool, 4>;
#[doc = "Field `HAURST` reader - HAU reset"]
pub type HAURST_R = crate::BitReader<bool>;
#[doc = "Field `HAURST` writer - HAU reset"]
pub type HAURST_W<'a> = crate::BitWriter<'a, u32, AHB2RST_SPEC, bool, 5>;
#[doc = "Field `TRNGRST` reader - TRNG reset"]
pub type TRNGRST_R = crate::BitReader<bool>;
#[doc = "Field `TRNGRST` writer - TRNG reset"]
pub type TRNGRST_W<'a> = crate::BitWriter<'a, u32, AHB2RST_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    pub fn dcirst(&self) -> DCIRST_R {
        DCIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CAU reset"]
    #[inline(always)]
    pub fn caurst(&self) -> CAURST_R {
        CAURST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HAU reset"]
    #[inline(always)]
    pub fn haurst(&self) -> HAURST_R {
        HAURST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    pub fn trngrst(&self) -> TRNGRST_R {
        TRNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    pub fn dcirst(&mut self) -> DCIRST_W {
        DCIRST_W::new(self)
    }
    #[doc = "Bit 4 - CAU reset"]
    #[inline(always)]
    pub fn caurst(&mut self) -> CAURST_W {
        CAURST_W::new(self)
    }
    #[doc = "Bit 5 - HAU reset"]
    #[inline(always)]
    pub fn haurst(&mut self) -> HAURST_W {
        HAURST_W::new(self)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    pub fn trngrst(&mut self) -> TRNGRST_W {
        TRNGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rst](index.html) module"]
pub struct AHB2RST_SPEC;
impl crate::RegisterSpec for AHB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2rst::R](R) reader structure"]
impl crate::Readable for AHB2RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2rst::W](W) writer structure"]
impl crate::Writable for AHB2RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2RST to value 0"]
impl crate::Resettable for AHB2RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
