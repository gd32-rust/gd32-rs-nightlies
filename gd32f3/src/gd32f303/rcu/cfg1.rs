#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `ADCPSC_3` reader - Bit 3 of ADCPSC"]
pub type ADCPSC_3_R = crate::BitReader;
#[doc = "Field `ADCPSC_3` writer - Bit 3 of ADCPSC"]
pub type ADCPSC_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLPRESEL` reader - PLL Clock Source Selection"]
pub type PLLPRESEL_R = crate::BitReader;
#[doc = "Field `PLLPRESEL` writer - PLL Clock Source Selection"]
pub type PLLPRESEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 29 - Bit 3 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_3(&self) -> ADCPSC_3_R {
        ADCPSC_3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllpresel(&self) -> PLLPRESEL_R {
        PLLPRESEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Bit 3 of ADCPSC"]
    #[inline(always)]
    #[must_use]
    pub fn adcpsc_3(&mut self) -> ADCPSC_3_W<CFG1_SPEC, 29> {
        ADCPSC_3_W::new(self)
    }
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pllpresel(&mut self) -> PLLPRESEL_W<CFG1_SPEC, 30> {
        PLLPRESEL_W::new(self)
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
#[doc = "Clock Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
