#[doc = "Register `L1CTL` reader"]
pub struct R(crate::R<L1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1CTL` writer"]
pub struct W(crate::W<L1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1CTL_SPEC>;
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
impl From<crate::W<L1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUTEN` reader - LUT enable"]
pub type LUTEN_R = crate::BitReader<bool>;
#[doc = "Field `LUTEN` writer - LUT enable"]
pub type LUTEN_W<'a> = crate::BitWriter<'a, u32, L1CTL_SPEC, bool, 4>;
#[doc = "Field `CKEYEN` reader - Color keying enable"]
pub type CKEYEN_R = crate::BitReader<bool>;
#[doc = "Field `CKEYEN` writer - Color keying enable"]
pub type CKEYEN_W<'a> = crate::BitWriter<'a, u32, L1CTL_SPEC, bool, 1>;
#[doc = "Field `LEN` reader - Layer enable"]
pub type LEN_R = crate::BitReader<bool>;
#[doc = "Field `LEN` writer - Layer enable"]
pub type LEN_W<'a> = crate::BitWriter<'a, u32, L1CTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 4 - LUT enable"]
    #[inline(always)]
    pub fn luten(&self) -> LUTEN_R {
        LUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - Color keying enable"]
    #[inline(always)]
    pub fn ckeyen(&self) -> CKEYEN_R {
        CKEYEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Layer enable"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - LUT enable"]
    #[inline(always)]
    pub fn luten(&mut self) -> LUTEN_W {
        LUTEN_W::new(self)
    }
    #[doc = "Bit 1 - Color keying enable"]
    #[inline(always)]
    pub fn ckeyen(&mut self) -> CKEYEN_W {
        CKEYEN_W::new(self)
    }
    #[doc = "Bit 0 - Layer enable"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1ctl](index.html) module"]
pub struct L1CTL_SPEC;
impl crate::RegisterSpec for L1CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1ctl::R](R) reader structure"]
impl crate::Readable for L1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1ctl::W](W) writer structure"]
impl crate::Writable for L1CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1CTL to value 0"]
impl crate::Resettable for L1CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
