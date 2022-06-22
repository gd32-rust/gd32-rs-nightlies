#[doc = "Register `BKP2` reader"]
pub struct R(crate::R<BKP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP2` writer"]
pub struct W(crate::W<BKP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP2_SPEC>;
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
impl From<crate::W<BKP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Backup domain registers"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Backup domain registers"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, BKP2_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Backup domain registers"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup domain registers"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp2](index.html) module"]
pub struct BKP2_SPEC;
impl crate::RegisterSpec for BKP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp2::R](R) reader structure"]
impl crate::Readable for BKP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp2::W](W) writer structure"]
impl crate::Writable for BKP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKP2 to value 0"]
impl crate::Resettable for BKP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
