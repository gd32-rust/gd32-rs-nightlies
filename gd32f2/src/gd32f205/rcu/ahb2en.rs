#[doc = "Register `AHB2EN` reader"]
pub type R = crate::R<AHB2EN_SPEC>;
#[doc = "Register `AHB2EN` writer"]
pub type W = crate::W<AHB2EN_SPEC>;
#[doc = "Field `DCIEN` reader - DCI clock enable"]
pub type DCIEN_R = crate::BitReader;
#[doc = "Field `DCIEN` writer - DCI clock enable"]
pub type DCIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAUEN` reader - CAU clock enable"]
pub type CAUEN_R = crate::BitReader;
#[doc = "Field `CAUEN` writer - CAU clock enable"]
pub type CAUEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HAUEN` reader - HAU clock enable"]
pub type HAUEN_R = crate::BitReader;
#[doc = "Field `HAUEN` writer - HAU clock enable"]
pub type HAUEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRNGEN` reader - TRNG clock enable"]
pub type TRNGEN_R = crate::BitReader;
#[doc = "Field `TRNGEN` writer - TRNG clock enable"]
pub type TRNGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DCIEN_R {
        DCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CAU clock enable"]
    #[inline(always)]
    pub fn cauen(&self) -> CAUEN_R {
        CAUEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HAU clock enable"]
    #[inline(always)]
    pub fn hauen(&self) -> HAUEN_R {
        HAUEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&self) -> TRNGEN_R {
        TRNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcien(&mut self) -> DCIEN_W<AHB2EN_SPEC, 0> {
        DCIEN_W::new(self)
    }
    #[doc = "Bit 4 - CAU clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cauen(&mut self) -> CAUEN_W<AHB2EN_SPEC, 4> {
        CAUEN_W::new(self)
    }
    #[doc = "Bit 5 - HAU clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hauen(&mut self) -> HAUEN_W<AHB2EN_SPEC, 5> {
        HAUEN_W::new(self)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn trngen(&mut self) -> TRNGEN_W<AHB2EN_SPEC, 6> {
        TRNGEN_W::new(self)
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
#[doc = "AHB2 enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2EN_SPEC;
impl crate::RegisterSpec for AHB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2en::R`](R) reader structure"]
impl crate::Readable for AHB2EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2en::W`](W) writer structure"]
impl crate::Writable for AHB2EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2EN to value 0"]
impl crate::Resettable for AHB2EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
