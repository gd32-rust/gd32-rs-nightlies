#[doc = "Register `L1DC` reader"]
pub struct R(crate::R<L1DC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1DC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1DC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1DC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1DC` writer"]
pub struct W(crate::W<L1DC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1DC_SPEC>;
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
impl From<crate::W<L1DC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1DC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCA` reader - The default color ALPHA"]
pub type DCA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCA` writer - The default color ALPHA"]
pub type DCA_W<'a> = crate::FieldWriter<'a, u32, L1DC_SPEC, u8, u8, 8, 24>;
#[doc = "Field `DCR` reader - The default color red"]
pub type DCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCR` writer - The default color red"]
pub type DCR_W<'a> = crate::FieldWriter<'a, u32, L1DC_SPEC, u8, u8, 8, 16>;
#[doc = "Field `DCG` reader - The default color green"]
pub type DCG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCG` writer - The default color green"]
pub type DCG_W<'a> = crate::FieldWriter<'a, u32, L1DC_SPEC, u8, u8, 8, 8>;
#[doc = "Field `DCB` reader - The default color blue"]
pub type DCB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCB` writer - The default color blue"]
pub type DCB_W<'a> = crate::FieldWriter<'a, u32, L1DC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - The default color ALPHA"]
    #[inline(always)]
    pub fn dca(&self) -> DCA_R {
        DCA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The default color red"]
    #[inline(always)]
    pub fn dcr(&self) -> DCR_R {
        DCR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The default color green"]
    #[inline(always)]
    pub fn dcg(&self) -> DCG_R {
        DCG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - The default color blue"]
    #[inline(always)]
    pub fn dcb(&self) -> DCB_R {
        DCB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - The default color ALPHA"]
    #[inline(always)]
    pub fn dca(&mut self) -> DCA_W {
        DCA_W::new(self)
    }
    #[doc = "Bits 16:23 - The default color red"]
    #[inline(always)]
    pub fn dcr(&mut self) -> DCR_W {
        DCR_W::new(self)
    }
    #[doc = "Bits 8:15 - The default color green"]
    #[inline(always)]
    pub fn dcg(&mut self) -> DCG_W {
        DCG_W::new(self)
    }
    #[doc = "Bits 0:7 - The default color blue"]
    #[inline(always)]
    pub fn dcb(&mut self) -> DCB_W {
        DCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 1 default color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1dc](index.html) module"]
pub struct L1DC_SPEC;
impl crate::RegisterSpec for L1DC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1dc::R](R) reader structure"]
impl crate::Readable for L1DC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1dc::W](W) writer structure"]
impl crate::Writable for L1DC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1DC to value 0"]
impl crate::Resettable for L1DC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
