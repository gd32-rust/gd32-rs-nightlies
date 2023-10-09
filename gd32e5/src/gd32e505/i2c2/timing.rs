#[doc = "Register `TIMING` reader"]
pub type R = crate::R<TIMING_SPEC>;
#[doc = "Register `TIMING` writer"]
pub type W = crate::W<TIMING_SPEC>;
#[doc = "Field `SCLL` reader - SCL low period"]
pub type SCLL_R = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low period"]
pub type SCLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SCLH` reader - SCL high period"]
pub type SCLH_R = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high period"]
pub type SCLH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SDADELY` reader - Data hold time"]
pub type SDADELY_R = crate::FieldReader;
#[doc = "Field `SDADELY` writer - Data hold time"]
pub type SDADELY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SCLDELY` reader - Data setup time"]
pub type SCLDELY_R = crate::FieldReader;
#[doc = "Field `SCLDELY` writer - Data setup time"]
pub type SCLDELY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PSC` reader - Timing prescaler"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Timing prescaler"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:7 - SCL low period"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high period"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    pub fn sdadely(&self) -> SDADELY_R {
        SDADELY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    pub fn scldely(&self) -> SCLDELY_R {
        SCLDELY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low period"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<TIMING_SPEC, 0> {
        SCLL_W::new(self)
    }
    #[doc = "Bits 8:15 - SCL high period"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<TIMING_SPEC, 8> {
        SCLH_W::new(self)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    #[must_use]
    pub fn sdadely(&mut self) -> SDADELY_W<TIMING_SPEC, 16> {
        SDADELY_W::new(self)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn scldely(&mut self) -> SCLDELY_W<TIMING_SPEC, 20> {
        SCLDELY_W::new(self)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<TIMING_SPEC, 28> {
        PSC_W::new(self)
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
#[doc = "Timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing::R`](R) reader structure"]
impl crate::Readable for TIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timing::W`](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
