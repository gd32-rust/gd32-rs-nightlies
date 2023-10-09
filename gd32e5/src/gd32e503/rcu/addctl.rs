#[doc = "Register `ADDCTL` reader"]
pub type R = crate::R<ADDCTL_SPEC>;
#[doc = "Register `ADDCTL` writer"]
pub type W = crate::W<ADDCTL_SPEC>;
#[doc = "Field `CK48MSEL` reader - 48MHz clock selection"]
pub type CK48MSEL_R = crate::FieldReader;
#[doc = "Field `CK48MSEL` writer - 48MHz clock selection"]
pub type CK48MSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IRC48MEN` reader - Internal 48MHz RC oscillator enable"]
pub type IRC48MEN_R = crate::BitReader;
#[doc = "Field `IRC48MEN` writer - Internal 48MHz RC oscillator enable"]
pub type IRC48MEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC48MSTB` reader - Internal 48MHz RC oscillator clock stabilization Flag"]
pub type IRC48MSTB_R = crate::BitReader;
#[doc = "Field `IRC48MCALIB` reader - Internal 48MHz RC oscillator calibration value register"]
pub type IRC48MCALIB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new((self.bits & 3) as u8)
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
    #[doc = "Bits 0:1 - 48MHz clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<ADDCTL_SPEC, 0> {
        CK48MSEL_W::new(self)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc48men(&mut self) -> IRC48MEN_W<ADDCTL_SPEC, 16> {
        IRC48MEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Additional clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDCTL_SPEC;
impl crate::RegisterSpec for ADDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addctl::R`](R) reader structure"]
impl crate::Readable for ADDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addctl::W`](W) writer structure"]
impl crate::Writable for ADDCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDCTL to value 0x8000_0000"]
impl crate::Resettable for ADDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
