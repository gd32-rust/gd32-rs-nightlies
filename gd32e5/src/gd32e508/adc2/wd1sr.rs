#[doc = "Register `WD1SR` reader"]
pub type R = crate::R<WD1SR_SPEC>;
#[doc = "Register `WD1SR` writer"]
pub type W = crate::W<WD1SR_SPEC>;
#[doc = "Field `AWD1CS` reader - Analog watchdog 1 channel selection"]
pub type AWD1CS_R = crate::FieldReader<u32>;
#[doc = "Field `AWD1CS` writer - Analog watchdog 1 channel selection"]
pub type AWD1CS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
impl R {
    #[doc = "Bits 0:17 - Analog watchdog 1 channel selection"]
    #[inline(always)]
    pub fn awd1cs(&self) -> AWD1CS_R {
        AWD1CS_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Analog watchdog 1 channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awd1cs(&mut self) -> AWD1CS_W<WD1SR_SPEC, 0> {
        AWD1CS_W::new(self)
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
#[doc = "Watchdog 1 Channel Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd1sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd1sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WD1SR_SPEC;
impl crate::RegisterSpec for WD1SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd1sr::R`](R) reader structure"]
impl crate::Readable for WD1SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wd1sr::W`](W) writer structure"]
impl crate::Writable for WD1SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WD1SR to value 0"]
impl crate::Resettable for WD1SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
