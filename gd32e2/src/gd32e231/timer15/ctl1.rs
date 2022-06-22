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
#[doc = "Field `ISO0N` reader - Output Idle state 0"]
pub type ISO0N_R = crate::BitReader<bool>;
#[doc = "Field `ISO0N` writer - Output Idle state 0"]
pub type ISO0N_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 9>;
#[doc = "Field `ISO0` reader - Output Idle state 0"]
pub type ISO0_R = crate::BitReader<bool>;
#[doc = "Field `ISO0` writer - Output Idle state 0"]
pub type ISO0_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 8>;
#[doc = "Field `DMAS` reader - Capture/compare DMA selection"]
pub type DMAS_R = crate::BitReader<bool>;
#[doc = "Field `DMAS` writer - Capture/compare DMA selection"]
pub type DMAS_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 3>;
#[doc = "Field `CCUC` reader - Capture/compare control update selection"]
pub type CCUC_R = crate::BitReader<bool>;
#[doc = "Field `CCUC` writer - Capture/compare control update selection"]
pub type CCUC_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 2>;
#[doc = "Field `CCSE` reader - Capture/compare preloaded control"]
pub type CCSE_R = crate::BitReader<bool>;
#[doc = "Field `CCSE` writer - Capture/compare preloaded control"]
pub type CCSE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 9 - Output Idle state 0"]
    #[inline(always)]
    pub fn iso0n(&self) -> ISO0N_R {
        ISO0N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 0"]
    #[inline(always)]
    pub fn iso0(&self) -> ISO0_R {
        ISO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccuc(&self) -> CCUC_R {
        CCUC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccse(&self) -> CCSE_R {
        CCSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Output Idle state 0"]
    #[inline(always)]
    pub fn iso0n(&mut self) -> ISO0N_W {
        ISO0N_W::new(self)
    }
    #[doc = "Bit 8 - Output Idle state 0"]
    #[inline(always)]
    pub fn iso0(&mut self) -> ISO0_W {
        ISO0_W::new(self)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn dmas(&mut self) -> DMAS_W {
        DMAS_W::new(self)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccuc(&mut self) -> CCUC_W {
        CCUC_W::new(self)
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccse(&mut self) -> CCSE_W {
        CCSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
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
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
