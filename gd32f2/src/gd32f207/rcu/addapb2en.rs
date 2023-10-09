#[doc = "Register `ADDAPB2EN` reader"]
pub type R = crate::R<ADDAPB2EN_SPEC>;
#[doc = "Register `ADDAPB2EN` writer"]
pub type W = crate::W<ADDAPB2EN_SPEC>;
#[doc = "Field `USART5EN` reader - USART5 clock enable"]
pub type USART5EN_R = crate::BitReader;
#[doc = "Field `USART5EN` writer - USART5 clock enable"]
pub type USART5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TLIEN` reader - TLI clock enable"]
pub type TLIEN_R = crate::BitReader;
#[doc = "Field `TLIEN` writer - TLI clock enable"]
pub type TLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PHEN` reader - GPIO port H clock enable"]
pub type PHEN_R = crate::BitReader;
#[doc = "Field `PHEN` writer - GPIO port H clock enable"]
pub type PHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIEN` reader - GPIO port I clock enable"]
pub type PIEN_R = crate::BitReader;
#[doc = "Field `PIEN` writer - GPIO port I clock enable"]
pub type PIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 24 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    pub fn tlien(&self) -> TLIEN_R {
        TLIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO port H clock enable"]
    #[inline(always)]
    pub fn phen(&self) -> PHEN_R {
        PHEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO port I clock enable"]
    #[inline(always)]
    pub fn pien(&self) -> PIEN_R {
        PIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5en(&mut self) -> USART5EN_W<ADDAPB2EN_SPEC, 24> {
        USART5EN_W::new(self)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlien(&mut self) -> TLIEN_W<ADDAPB2EN_SPEC, 26> {
        TLIEN_W::new(self)
    }
    #[doc = "Bit 30 - GPIO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn phen(&mut self) -> PHEN_W<ADDAPB2EN_SPEC, 30> {
        PHEN_W::new(self)
    }
    #[doc = "Bit 31 - GPIO port I clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pien(&mut self) -> PIEN_W<ADDAPB2EN_SPEC, 31> {
        PIEN_W::new(self)
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
#[doc = "APB2 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDAPB2EN_SPEC;
impl crate::RegisterSpec for ADDAPB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb2en::R`](R) reader structure"]
impl crate::Readable for ADDAPB2EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addapb2en::W`](W) writer structure"]
impl crate::Writable for ADDAPB2EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDAPB2EN to value 0"]
impl crate::Resettable for ADDAPB2EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
