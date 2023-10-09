#[doc = "Register `STATC` reader"]
pub type R = crate::R<STATC_SPEC>;
#[doc = "Register `STATC` writer"]
pub type W = crate::W<STATC_SPEC>;
#[doc = "Field `SOFC` reader - Start of frame flag clear"]
pub type SOFC_R = crate::BitReader;
#[doc = "Field `SOFC` writer - Start of frame flag clear"]
pub type SOFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDC` reader - SLCD data update done clear bit"]
pub type UPDC_R = crate::BitReader;
#[doc = "Field `UPDC` writer - SLCD data update done clear bit"]
pub type UPDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    pub fn sofc(&self) -> SOFC_R {
        SOFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SLCD data update done clear bit"]
    #[inline(always)]
    pub fn updc(&self) -> UPDC_R {
        UPDC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn sofc(&mut self) -> SOFC_W<STATC_SPEC, 1> {
        SOFC_W::new(self)
    }
    #[doc = "Bit 3 - SLCD data update done clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn updc(&mut self) -> UPDC_W<STATC_SPEC, 3> {
        UPDC_W::new(self)
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
#[doc = "SLCD status flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATC_SPEC;
impl crate::RegisterSpec for STATC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statc::R`](R) reader structure"]
impl crate::Readable for STATC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`statc::W`](W) writer structure"]
impl crate::Writable for STATC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATC to value 0"]
impl crate::Resettable for STATC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
