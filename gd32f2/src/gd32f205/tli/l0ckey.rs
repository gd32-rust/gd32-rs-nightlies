#[doc = "Register `L0CKEY` reader"]
pub struct R(crate::R<L0CKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L0CKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L0CKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L0CKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L0CKEY` writer"]
pub struct W(crate::W<L0CKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L0CKEY_SPEC>;
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
impl From<crate::W<L0CKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L0CKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKEYR` reader - Color Key Red"]
pub type CKEYR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKEYR` writer - Color Key Red"]
pub type CKEYR_W<'a> = crate::FieldWriter<'a, u32, L0CKEY_SPEC, u8, u8, 8, 16>;
#[doc = "Field `CKEYG` reader - Color Key Green"]
pub type CKEYG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKEYG` writer - Color Key Green"]
pub type CKEYG_W<'a> = crate::FieldWriter<'a, u32, L0CKEY_SPEC, u8, u8, 8, 8>;
#[doc = "Field `CKEYB` reader - Color Key Blue"]
pub type CKEYB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKEYB` writer - Color Key Blue"]
pub type CKEYB_W<'a> = crate::FieldWriter<'a, u32, L0CKEY_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    pub fn ckeyr(&self) -> CKEYR_R {
        CKEYR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    pub fn ckeyg(&self) -> CKEYG_R {
        CKEYG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    pub fn ckeyb(&self) -> CKEYB_R {
        CKEYB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Color Key Red"]
    #[inline(always)]
    pub fn ckeyr(&mut self) -> CKEYR_W {
        CKEYR_W::new(self)
    }
    #[doc = "Bits 8:15 - Color Key Green"]
    #[inline(always)]
    pub fn ckeyg(&mut self) -> CKEYG_W {
        CKEYG_W::new(self)
    }
    #[doc = "Bits 0:7 - Color Key Blue"]
    #[inline(always)]
    pub fn ckeyb(&mut self) -> CKEYB_W {
        CKEYB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 0 color key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l0ckey](index.html) module"]
pub struct L0CKEY_SPEC;
impl crate::RegisterSpec for L0CKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l0ckey::R](R) reader structure"]
impl crate::Readable for L0CKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l0ckey::W](W) writer structure"]
impl crate::Writable for L0CKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L0CKEY to value 0"]
impl crate::Resettable for L0CKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
