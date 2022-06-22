#[doc = "Register `STAT1` reader"]
pub struct R(crate::R<STAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT1` writer"]
pub struct W(crate::W<STAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT1_SPEC>;
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
impl From<crate::W<STAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSY` reader - Busy flag"]
pub type BSY_R = crate::BitReader<bool>;
#[doc = "Field `EBF` writer - End of block flag"]
pub type EBF_W<'a> = crate::BitWriter<'a, u32, STAT1_SPEC, bool, 12>;
#[doc = "Field `RTF` writer - Receiver timeout flag"]
pub type RTF_W<'a> = crate::BitWriter<'a, u32, STAT1_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 16 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - End of block flag"]
    #[inline(always)]
    pub fn ebf(&mut self) -> EBF_W {
        EBF_W::new(self)
    }
    #[doc = "Bit 11 - Receiver timeout flag"]
    #[inline(always)]
    pub fn rtf(&mut self) -> RTF_W {
        RTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](index.html) module"]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat1::R](R) reader structure"]
impl crate::Readable for STAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat1::W](W) writer structure"]
impl crate::Writable for STAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT1 to value 0xc0"]
impl crate::Resettable for STAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
