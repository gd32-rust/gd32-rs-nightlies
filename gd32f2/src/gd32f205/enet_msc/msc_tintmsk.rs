#[doc = "Register `MSC_TINTMSK` reader"]
pub type R = crate::R<MSC_TINTMSK_SPEC>;
#[doc = "Register `MSC_TINTMSK` writer"]
pub type W = crate::W<MSC_TINTMSK_SPEC>;
#[doc = "Field `TGFSCIM` reader - Transmitted good frames single collision interrupt mask"]
pub type TGFSCIM_R = crate::BitReader;
#[doc = "Field `TGFSCIM` writer - Transmitted good frames single collision interrupt mask"]
pub type TGFSCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TGFMSCIM` reader - Transmitted good frames more single interrupt collision mask"]
pub type TGFMSCIM_R = crate::BitReader;
#[doc = "Field `TGFMSCIM` writer - Transmitted good frames more single interrupt collision mask"]
pub type TGFMSCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TGFIM` reader - Transmitted good frames interrupt mask"]
pub type TGFIM_R = crate::BitReader;
#[doc = "Field `TGFIM` writer - Transmitted good frames interrupt mask"]
pub type TGFIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision interrupt mask"]
    #[inline(always)]
    pub fn tgfscim(&self) -> TGFSCIM_R {
        TGFSCIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single interrupt collision mask"]
    #[inline(always)]
    pub fn tgfmscim(&self) -> TGFMSCIM_R {
        TGFMSCIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames interrupt mask"]
    #[inline(always)]
    pub fn tgfim(&self) -> TGFIM_R {
        TGFIM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfscim(&mut self) -> TGFSCIM_W<MSC_TINTMSK_SPEC, 14> {
        TGFSCIM_W::new(self)
    }
    #[doc = "Bit 15 - Transmitted good frames more single interrupt collision mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfmscim(&mut self) -> TGFMSCIM_W<MSC_TINTMSK_SPEC, 15> {
        TGFMSCIM_W::new(self)
    }
    #[doc = "Bit 21 - Transmitted good frames interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfim(&mut self) -> TGFIM_W<MSC_TINTMSK_SPEC, 21> {
        TGFIM_W::new(self)
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
#[doc = "Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_tintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msc_tintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_TINTMSK_SPEC;
impl crate::RegisterSpec for MSC_TINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_tintmsk::R`](R) reader structure"]
impl crate::Readable for MSC_TINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msc_tintmsk::W`](W) writer structure"]
impl crate::Writable for MSC_TINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSC_TINTMSK to value 0"]
impl crate::Resettable for MSC_TINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
