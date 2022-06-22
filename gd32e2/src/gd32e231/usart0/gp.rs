#[doc = "Register `GP` reader"]
pub struct R(crate::R<GP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP` writer"]
pub struct W(crate::W<GP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_SPEC>;
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
impl From<crate::W<GP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GUAT` reader - Guard time value"]
pub type GUAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GUAT` writer - Guard time value"]
pub type GUAT_W<'a> = crate::FieldWriterSafe<'a, u32, GP_SPEC, u8, u8, 8, 8>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PSC_W<'a> = crate::FieldWriterSafe<'a, u32, GP_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn guat(&self) -> GUAT_R {
        GUAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn guat(&mut self) -> GUAT_W {
        GUAT_W::new(self)
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Guard time and prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp](index.html) module"]
pub struct GP_SPEC;
impl crate::RegisterSpec for GP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp::R](R) reader structure"]
impl crate::Readable for GP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp::W](W) writer structure"]
impl crate::Writable for GP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP to value 0"]
impl crate::Resettable for GP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
