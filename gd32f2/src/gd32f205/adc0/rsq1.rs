#[doc = "Register `RSQ1` reader"]
pub struct R(crate::R<RSQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSQ1` writer"]
pub struct W(crate::W<RSQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSQ1_SPEC>;
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
impl From<crate::W<RSQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSQ11` reader - 12th conversion in regular sequence"]
pub type RSQ11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ11` writer - 12th conversion in regular sequence"]
pub type RSQ11_W<'a> = crate::FieldWriter<'a, u32, RSQ1_SPEC, u8, u8, 5, 25>;
#[doc = "Field `RSQ10` reader - 11th conversion in regular sequence"]
pub type RSQ10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ10` writer - 11th conversion in regular sequence"]
pub type RSQ10_W<'a> = crate::FieldWriter<'a, u32, RSQ1_SPEC, u8, u8, 5, 20>;
#[doc = "Field `RSQ9` reader - 10th conversion in regular sequence"]
pub type RSQ9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ9` writer - 10th conversion in regular sequence"]
pub type RSQ9_W<'a> = crate::FieldWriter<'a, u32, RSQ1_SPEC, u8, u8, 5, 15>;
#[doc = "Field `RSQ8` reader - 9th conversion in regular sequence"]
pub type RSQ8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ8` writer - 9th conversion in regular sequence"]
pub type RSQ8_W<'a> = crate::FieldWriter<'a, u32, RSQ1_SPEC, u8, u8, 5, 10>;
#[doc = "Field `RSQ7` reader - 8th conversion in regular sequence"]
pub type RSQ7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ7` writer - 8th conversion in regular sequence"]
pub type RSQ7_W<'a> = crate::FieldWriter<'a, u32, RSQ1_SPEC, u8, u8, 5, 5>;
#[doc = "Field `RSQ6` reader - 7th conversion in regular sequence"]
pub type RSQ6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ6` writer - 7th conversion in regular sequence"]
pub type RSQ6_W<'a> = crate::FieldWriter<'a, u32, RSQ1_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq11(&self) -> RSQ11_R {
        RSQ11_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq10(&self) -> RSQ10_R {
        RSQ10_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq9(&self) -> RSQ9_R {
        RSQ9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq8(&self) -> RSQ8_R {
        RSQ8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq7(&self) -> RSQ7_R {
        RSQ7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq6(&self) -> RSQ6_R {
        RSQ6_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq11(&mut self) -> RSQ11_W {
        RSQ11_W::new(self)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq10(&mut self) -> RSQ10_W {
        RSQ10_W::new(self)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq9(&mut self) -> RSQ9_W {
        RSQ9_W::new(self)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq8(&mut self) -> RSQ8_W {
        RSQ8_W::new(self)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq7(&mut self) -> RSQ7_W {
        RSQ7_W::new(self)
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq6(&mut self) -> RSQ6_W {
        RSQ6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsq1](index.html) module"]
pub struct RSQ1_SPEC;
impl crate::RegisterSpec for RSQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsq1::R](R) reader structure"]
impl crate::Readable for RSQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsq1::W](W) writer structure"]
impl crate::Writable for RSQ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSQ1 to value 0"]
impl crate::Resettable for RSQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
