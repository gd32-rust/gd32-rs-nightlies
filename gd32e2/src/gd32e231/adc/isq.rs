#[doc = "Register `ISQ` reader"]
pub struct R(crate::R<ISQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISQ` writer"]
pub struct W(crate::W<ISQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISQ_SPEC>;
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
impl From<crate::W<ISQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IL` reader - Injected sequence length"]
pub type IL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IL` writer - Injected sequence length"]
pub type IL_W<'a> = crate::FieldWriterSafe<'a, u32, ISQ_SPEC, u8, u8, 2, 20>;
#[doc = "Field `ISQ3` reader - 3rd conversion in injected sequence"]
pub type ISQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISQ3` writer - 3rd conversion in injected sequence"]
pub type ISQ3_W<'a> = crate::FieldWriter<'a, u32, ISQ_SPEC, u8, u8, 5, 15>;
#[doc = "Field `ISQ2` reader - 2nd conversion in injected sequence"]
pub type ISQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISQ2` writer - 2nd conversion in injected sequence"]
pub type ISQ2_W<'a> = crate::FieldWriter<'a, u32, ISQ_SPEC, u8, u8, 5, 10>;
#[doc = "Field `ISQ1` reader - 1st conversion in injected sequence"]
pub type ISQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISQ1` writer - 1st conversion in injected sequence"]
pub type ISQ1_W<'a> = crate::FieldWriter<'a, u32, ISQ_SPEC, u8, u8, 5, 5>;
#[doc = "Field `ISQ0` reader - conversion in injected sequence"]
pub type ISQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISQ0` writer - conversion in injected sequence"]
pub type ISQ0_W<'a> = crate::FieldWriter<'a, u32, ISQ_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 20:21 - Injected sequence length"]
    #[inline(always)]
    pub fn il(&self) -> IL_R {
        IL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 15:19 - 3rd conversion in injected sequence"]
    #[inline(always)]
    pub fn isq3(&self) -> ISQ3_R {
        ISQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 2nd conversion in injected sequence"]
    #[inline(always)]
    pub fn isq2(&self) -> ISQ2_R {
        ISQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 1st conversion in injected sequence"]
    #[inline(always)]
    pub fn isq1(&self) -> ISQ1_R {
        ISQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - conversion in injected sequence"]
    #[inline(always)]
    pub fn isq0(&self) -> ISQ0_R {
        ISQ0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Injected sequence length"]
    #[inline(always)]
    pub fn il(&mut self) -> IL_W {
        IL_W::new(self)
    }
    #[doc = "Bits 15:19 - 3rd conversion in injected sequence"]
    #[inline(always)]
    pub fn isq3(&mut self) -> ISQ3_W {
        ISQ3_W::new(self)
    }
    #[doc = "Bits 10:14 - 2nd conversion in injected sequence"]
    #[inline(always)]
    pub fn isq2(&mut self) -> ISQ2_W {
        ISQ2_W::new(self)
    }
    #[doc = "Bits 5:9 - 1st conversion in injected sequence"]
    #[inline(always)]
    pub fn isq1(&mut self) -> ISQ1_W {
        ISQ1_W::new(self)
    }
    #[doc = "Bits 0:4 - conversion in injected sequence"]
    #[inline(always)]
    pub fn isq0(&mut self) -> ISQ0_W {
        ISQ0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isq](index.html) module"]
pub struct ISQ_SPEC;
impl crate::RegisterSpec for ISQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isq::R](R) reader structure"]
impl crate::Readable for ISQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isq::W](W) writer structure"]
impl crate::Writable for ISQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISQ to value 0"]
impl crate::Resettable for ISQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
