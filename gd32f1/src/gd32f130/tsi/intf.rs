#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Field `CTCF` reader - Charge-Transfer complete flag"]
pub type CTCF_R = crate::BitReader;
#[doc = "Field `CTCF` writer - Charge-Transfer complete flag"]
pub type CTCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MNERR` reader - Max count error flag"]
pub type MNERR_R = crate::BitReader;
#[doc = "Field `MNERR` writer - Max count error flag"]
pub type MNERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Charge-Transfer complete flag"]
    #[inline(always)]
    pub fn ctcf(&self) -> CTCF_R {
        CTCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    pub fn mnerr(&self) -> MNERR_R {
        MNERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Charge-Transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<INTF_SPEC, 0> {
        CTCF_W::new(self)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    #[must_use]
    pub fn mnerr(&mut self) -> MNERR_W<INTF_SPEC, 1> {
        MNERR_W::new(self)
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
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
