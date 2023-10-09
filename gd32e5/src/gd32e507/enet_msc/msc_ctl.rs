#[doc = "Register `MSC_CTL` reader"]
pub type R = crate::R<MSC_CTL_SPEC>;
#[doc = "Register `MSC_CTL` writer"]
pub type W = crate::W<MSC_CTL_SPEC>;
#[doc = "Field `CTR` reader - Counter reset"]
pub type CTR_R = crate::BitReader;
#[doc = "Field `CTR` writer - Counter reset"]
pub type CTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTSR` reader - Counter stop rollover"]
pub type CTSR_R = crate::BitReader;
#[doc = "Field `CTSR` writer - Counter stop rollover"]
pub type CTSR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTOR` reader - Reset on read"]
pub type RTOR_R = crate::BitReader;
#[doc = "Field `RTOR` writer - Reset on read"]
pub type RTOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCFZ` reader - MSC counter freeze"]
pub type MCFZ_R = crate::BitReader;
#[doc = "Field `MCFZ` writer - MSC counter freeze"]
pub type MCFZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PMC` writer - Preset MSC counter"]
pub type PMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AFHPM` reader - Almost full or half preset mode"]
pub type AFHPM_R = crate::BitReader;
#[doc = "Field `AFHPM` writer - Almost full or half preset mode"]
pub type AFHPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 5 - Almost full or half preset mode"]
    #[inline(always)]
    pub fn afhpm(&self) -> AFHPM_R {
        AFHPM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<MSC_CTL_SPEC, 0> {
        CTR_W::new(self)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    #[must_use]
    pub fn ctsr(&mut self) -> CTSR_W<MSC_CTL_SPEC, 1> {
        CTSR_W::new(self)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn rtor(&mut self) -> RTOR_W<MSC_CTL_SPEC, 2> {
        RTOR_W::new(self)
    }
    #[doc = "Bit 3 - MSC counter freeze"]
    #[inline(always)]
    #[must_use]
    pub fn mcfz(&mut self) -> MCFZ_W<MSC_CTL_SPEC, 3> {
        MCFZ_W::new(self)
    }
    #[doc = "Bit 4 - Preset MSC counter"]
    #[inline(always)]
    #[must_use]
    pub fn pmc(&mut self) -> PMC_W<MSC_CTL_SPEC, 4> {
        PMC_W::new(self)
    }
    #[doc = "Bit 5 - Almost full or half preset mode"]
    #[inline(always)]
    #[must_use]
    pub fn afhpm(&mut self) -> AFHPM_W<MSC_CTL_SPEC, 5> {
        AFHPM_W::new(self)
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
#[doc = "Ethernet MSC control register (MSC_CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msc_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_CTL_SPEC;
impl crate::RegisterSpec for MSC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_ctl::R`](R) reader structure"]
impl crate::Readable for MSC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msc_ctl::W`](W) writer structure"]
impl crate::Writable for MSC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSC_CTL to value 0"]
impl crate::Resettable for MSC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
