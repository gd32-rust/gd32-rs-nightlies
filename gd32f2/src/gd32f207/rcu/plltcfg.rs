#[doc = "Register `PLLTCFG` reader"]
pub struct R(crate::R<PLLTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLTCFG` writer"]
pub struct W(crate::W<PLLTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLTCFG_SPEC>;
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
impl From<crate::W<PLLTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLTPSC` reader - PLLT prescaler selection"]
pub type PLLTPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLTPSC` writer - PLLT prescaler selection"]
pub type PLLTPSC_W<'a> = crate::FieldWriter<'a, u32, PLLTCFG_SPEC, u8, u8, 6, 0>;
#[doc = "Field `PLLTMF` reader - PLLT multiply factor for VCO"]
pub type PLLTMF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLLTMF` writer - PLLT multiply factor for VCO"]
pub type PLLTMF_W<'a> = crate::FieldWriter<'a, u32, PLLTCFG_SPEC, u16, u16, 9, 6>;
#[doc = "Field `TLIPSC` reader - TLI prescaler selection"]
pub type TLIPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLIPSC` writer - TLI prescaler selection"]
pub type TLIPSC_W<'a> = crate::FieldWriter<'a, u32, PLLTCFG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `PLLTRPSC` reader - PLLTR prescaler selection"]
pub type PLLTRPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLTRPSC` writer - PLLTR prescaler selection"]
pub type PLLTRPSC_W<'a> = crate::FieldWriter<'a, u32, PLLTCFG_SPEC, u8, u8, 3, 28>;
#[doc = "Field `PLLTSEL` reader - PLLT clock source select"]
pub type PLLTSEL_R = crate::BitReader<bool>;
#[doc = "Field `PLLTSEL` writer - PLLT clock source select"]
pub type PLLTSEL_W<'a> = crate::BitWriter<'a, u32, PLLTCFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:5 - PLLT prescaler selection"]
    #[inline(always)]
    pub fn plltpsc(&self) -> PLLTPSC_R {
        PLLTPSC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLLT multiply factor for VCO"]
    #[inline(always)]
    pub fn plltmf(&self) -> PLLTMF_R {
        PLLTMF_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - TLI prescaler selection"]
    #[inline(always)]
    pub fn tlipsc(&self) -> TLIPSC_R {
        TLIPSC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:30 - PLLTR prescaler selection"]
    #[inline(always)]
    pub fn plltrpsc(&self) -> PLLTRPSC_R {
        PLLTRPSC_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - PLLT clock source select"]
    #[inline(always)]
    pub fn plltsel(&self) -> PLLTSEL_R {
        PLLTSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLLT prescaler selection"]
    #[inline(always)]
    pub fn plltpsc(&mut self) -> PLLTPSC_W {
        PLLTPSC_W::new(self)
    }
    #[doc = "Bits 6:14 - PLLT multiply factor for VCO"]
    #[inline(always)]
    pub fn plltmf(&mut self) -> PLLTMF_W {
        PLLTMF_W::new(self)
    }
    #[doc = "Bits 16:17 - TLI prescaler selection"]
    #[inline(always)]
    pub fn tlipsc(&mut self) -> TLIPSC_W {
        TLIPSC_W::new(self)
    }
    #[doc = "Bits 28:30 - PLLTR prescaler selection"]
    #[inline(always)]
    pub fn plltrpsc(&mut self) -> PLLTRPSC_W {
        PLLTRPSC_W::new(self)
    }
    #[doc = "Bit 31 - PLLT clock source select"]
    #[inline(always)]
    pub fn plltsel(&mut self) -> PLLTSEL_W {
        PLLTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plltcfg](index.html) module"]
pub struct PLLTCFG_SPEC;
impl crate::RegisterSpec for PLLTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plltcfg::R](R) reader structure"]
impl crate::Readable for PLLTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plltcfg::W](W) writer structure"]
impl crate::Writable for PLLTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLTCFG to value 0x2000_3010"]
impl crate::Resettable for PLLTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_3010
    }
}
