#[doc = "Register `MSC_CTL` reader"]
pub struct R(crate::R<MSC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSC_CTL` writer"]
pub struct W(crate::W<MSC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSC_CTL_SPEC>;
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
impl From<crate::W<MSC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR` reader - Counter reset"]
pub type CTR_R = crate::BitReader<bool>;
#[doc = "Field `CTR` writer - Counter reset"]
pub type CTR_W<'a> = crate::BitWriter<'a, u32, MSC_CTL_SPEC, bool, 0>;
#[doc = "Field `CTSR` reader - Counter stop rollover"]
pub type CTSR_R = crate::BitReader<bool>;
#[doc = "Field `CTSR` writer - Counter stop rollover"]
pub type CTSR_W<'a> = crate::BitWriter<'a, u32, MSC_CTL_SPEC, bool, 1>;
#[doc = "Field `RTOR` reader - Reset on read"]
pub type RTOR_R = crate::BitReader<bool>;
#[doc = "Field `RTOR` writer - Reset on read"]
pub type RTOR_W<'a> = crate::BitWriter<'a, u32, MSC_CTL_SPEC, bool, 2>;
#[doc = "Field `MCFZ` reader - MSC counter freeze"]
pub type MCFZ_R = crate::BitReader<bool>;
#[doc = "Field `MCFZ` writer - MSC counter freeze"]
pub type MCFZ_W<'a> = crate::BitWriter<'a, u32, MSC_CTL_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn ctsr(&self) -> CTSR_R {
        CTSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn rtor(&self) -> RTOR_R {
        RTOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSC counter freeze"]
    #[inline(always)]
    pub fn mcfz(&self) -> MCFZ_R {
        MCFZ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W::new(self)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn ctsr(&mut self) -> CTSR_W {
        CTSR_W::new(self)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn rtor(&mut self) -> RTOR_W {
        RTOR_W::new(self)
    }
    #[doc = "Bit 3 - MSC counter freeze"]
    #[inline(always)]
    pub fn mcfz(&mut self) -> MCFZ_W {
        MCFZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MSC control register (MSC_CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_ctl](index.html) module"]
pub struct MSC_CTL_SPEC;
impl crate::RegisterSpec for MSC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_ctl::R](R) reader structure"]
impl crate::Readable for MSC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msc_ctl::W](W) writer structure"]
impl crate::Writable for MSC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSC_CTL to value 0"]
impl crate::Resettable for MSC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
