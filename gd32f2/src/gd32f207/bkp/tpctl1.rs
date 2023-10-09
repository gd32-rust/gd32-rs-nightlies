#[doc = "Register `TPCTL1` reader"]
pub type R = crate::R<TPCTL1_SPEC>;
#[doc = "Register `TPCTL1` writer"]
pub type W = crate::W<TPCTL1_SPEC>;
#[doc = "Field `TPEN1` reader - TAMPER1 detection enable"]
pub type TPEN1_R = crate::BitReader;
#[doc = "Field `TPEN1` writer - TAMPER1 detection enable"]
pub type TPEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPAL1` reader - TAMPER1 pin active level"]
pub type TPAL1_R = crate::BitReader;
#[doc = "Field `TPAL1` writer - TAMPER1 pin active level"]
pub type TPAL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPM2` reader - The second waveform detection enable"]
pub type TPM2_R = crate::BitReader;
#[doc = "Field `TPM2` writer - The second waveform detection enable"]
pub type TPM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPM1` reader - The first waveform detection enable"]
pub type TPM1_R = crate::BitReader;
#[doc = "Field `TPM1` writer - The first waveform detection enable"]
pub type TPM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 8 - TAMPER1 detection enable"]
    #[inline(always)]
    pub fn tpen1(&self) -> TPEN1_R {
        TPEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TAMPER1 pin active level"]
    #[inline(always)]
    pub fn tpal1(&self) -> TPAL1_R {
        TPAL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - The second waveform detection enable"]
    #[inline(always)]
    pub fn tpm2(&self) -> TPM2_R {
        TPM2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The first waveform detection enable"]
    #[inline(always)]
    pub fn tpm1(&self) -> TPM1_R {
        TPM1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - TAMPER1 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpen1(&mut self) -> TPEN1_W<TPCTL1_SPEC, 8> {
        TPEN1_W::new(self)
    }
    #[doc = "Bit 9 - TAMPER1 pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal1(&mut self) -> TPAL1_W<TPCTL1_SPEC, 9> {
        TPAL1_W::new(self)
    }
    #[doc = "Bit 14 - The second waveform detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpm2(&mut self) -> TPM2_W<TPCTL1_SPEC, 14> {
        TPM2_W::new(self)
    }
    #[doc = "Bit 15 - The first waveform detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpm1(&mut self) -> TPM1_W<TPCTL1_SPEC, 15> {
        TPM1_W::new(self)
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
#[doc = "Tamper pin control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCTL1_SPEC;
impl crate::RegisterSpec for TPCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpctl1::R`](R) reader structure"]
impl crate::Readable for TPCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpctl1::W`](W) writer structure"]
impl crate::Writable for TPCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPCTL1 to value 0"]
impl crate::Resettable for TPCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
