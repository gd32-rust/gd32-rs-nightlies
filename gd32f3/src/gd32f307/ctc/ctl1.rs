#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFPOL` reader - Reference signal source polarity"]
pub type REFPOL_R = crate::BitReader<bool>;
#[doc = "Field `REFPOL` writer - Reference signal source polarity"]
pub type REFPOL_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 31>;
#[doc = "Field `REFSEL` reader - Reference signal source selection"]
pub type REFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFSEL` writer - Reference signal source selection"]
pub type REFSEL_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 2, 28>;
#[doc = "Field `REFPSC` reader - Reference signal source prescaler"]
pub type REFPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFPSC` writer - Reference signal source prescaler"]
pub type REFPSC_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 3, 24>;
#[doc = "Field `CKLIM` reader - Clock trim base limit value"]
pub type CKLIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKLIM` writer - Clock trim base limit value"]
pub type CKLIM_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 8, 16>;
#[doc = "Field `RLVALUE` reader - CTC counter reload value"]
pub type RLVALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RLVALUE` writer - CTC counter reload value"]
pub type RLVALUE_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&self) -> REFPOL_R {
        REFPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    pub fn refpsc(&self) -> REFPSC_R {
        REFPSC_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    pub fn cklim(&self) -> CKLIM_R {
        CKLIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&self) -> RLVALUE_R {
        RLVALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&mut self) -> REFPOL_W {
        REFPOL_W::new(self)
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    pub fn refpsc(&mut self) -> REFPSC_W {
        REFPSC_W::new(self)
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    pub fn cklim(&mut self) -> CKLIM_W {
        CKLIM_W::new(self)
    }
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&mut self) -> RLVALUE_W {
        RLVALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0x2022_bb7f"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2022_bb7f
    }
}
