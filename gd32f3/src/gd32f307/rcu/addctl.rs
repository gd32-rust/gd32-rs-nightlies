#[doc = "Register `ADDCTL` reader"]
pub struct R(crate::R<ADDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDCTL` writer"]
pub struct W(crate::W<ADDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDCTL_SPEC>;
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
impl From<crate::W<ADDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK48MSEL` reader - 48MHz clock selection"]
pub type CK48MSEL_R = crate::BitReader<bool>;
#[doc = "Field `CK48MSEL` writer - 48MHz clock selection"]
pub type CK48MSEL_W<'a> = crate::BitWriter<'a, u32, ADDCTL_SPEC, bool, 0>;
#[doc = "Field `IRC48MEN` reader - Internal 48MHz RC oscillator enable"]
pub type IRC48MEN_R = crate::BitReader<bool>;
#[doc = "Field `IRC48MEN` writer - Internal 48MHz RC oscillator enable"]
pub type IRC48MEN_W<'a> = crate::BitWriter<'a, u32, ADDCTL_SPEC, bool, 16>;
#[doc = "Field `IRC48MSTB` reader - Internal 48MHz RC oscillator clock stabilization Flag"]
pub type IRC48MSTB_R = crate::BitReader<bool>;
#[doc = "Field `IRC48MCALIB` reader - Internal 48MHz RC oscillator calibration value register"]
pub type IRC48MCALIB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&self) -> IRC48MEN_R {
        IRC48MEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal 48MHz RC oscillator clock stabilization Flag"]
    #[inline(always)]
    pub fn irc48mstb(&self) -> IRC48MSTB_R {
        IRC48MSTB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Internal 48MHz RC oscillator calibration value register"]
    #[inline(always)]
    pub fn irc48mcalib(&self) -> IRC48MCALIB_R {
        IRC48MCALIB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W {
        CK48MSEL_W::new(self)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&mut self) -> IRC48MEN_W {
        IRC48MEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Additional clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addctl](index.html) module"]
pub struct ADDCTL_SPEC;
impl crate::RegisterSpec for ADDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addctl::R](R) reader structure"]
impl crate::Readable for ADDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addctl::W](W) writer structure"]
impl crate::Writable for ADDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDCTL to value 0x8000_0000"]
impl crate::Resettable for ADDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
