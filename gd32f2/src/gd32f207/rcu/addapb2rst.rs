#[doc = "Register `ADDAPB2RST` reader"]
pub type R = crate::R<ADDAPB2RST_SPEC>;
#[doc = "Register `ADDAPB2RST` writer"]
pub type W = crate::W<ADDAPB2RST_SPEC>;
#[doc = "Field `USART5RST` reader - USART5 reset"]
pub type USART5RST_R = crate::BitReader;
#[doc = "Field `USART5RST` writer - USART5 reset"]
pub type USART5RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TLIRST` reader - TLI reset"]
pub type TLIRST_R = crate::BitReader;
#[doc = "Field `TLIRST` writer - TLI reset"]
pub type TLIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PHRST` reader - GPIO port H reset"]
pub type PHRST_R = crate::BitReader;
#[doc = "Field `PHRST` writer - GPIO port H reset"]
pub type PHRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIRST` reader - GPIO port I reset"]
pub type PIRST_R = crate::BitReader;
#[doc = "Field `PIRST` writer - GPIO port I reset"]
pub type PIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 24 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI reset"]
    #[inline(always)]
    pub fn tlirst(&self) -> TLIRST_R {
        TLIRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO port H reset"]
    #[inline(always)]
    pub fn phrst(&self) -> PHRST_R {
        PHRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO port I reset"]
    #[inline(always)]
    pub fn pirst(&self) -> PIRST_R {
        PIRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart5rst(&mut self) -> USART5RST_W<ADDAPB2RST_SPEC, 24> {
        USART5RST_W::new(self)
    }
    #[doc = "Bit 26 - TLI reset"]
    #[inline(always)]
    #[must_use]
    pub fn tlirst(&mut self) -> TLIRST_W<ADDAPB2RST_SPEC, 26> {
        TLIRST_W::new(self)
    }
    #[doc = "Bit 30 - GPIO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn phrst(&mut self) -> PHRST_W<ADDAPB2RST_SPEC, 30> {
        PHRST_W::new(self)
    }
    #[doc = "Bit 31 - GPIO port I reset"]
    #[inline(always)]
    #[must_use]
    pub fn pirst(&mut self) -> PIRST_W<ADDAPB2RST_SPEC, 31> {
        PIRST_W::new(self)
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
#[doc = "APB2 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDAPB2RST_SPEC;
impl crate::RegisterSpec for ADDAPB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb2rst::R`](R) reader structure"]
impl crate::Readable for ADDAPB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addapb2rst::W`](W) writer structure"]
impl crate::Writable for ADDAPB2RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDAPB2RST to value 0"]
impl crate::Resettable for ADDAPB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
