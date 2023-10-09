#[doc = "Register `PCFA` reader"]
pub type R = crate::R<PCFA_SPEC>;
#[doc = "Register `PCFA` writer"]
pub type W = crate::W<PCFA_SPEC>;
#[doc = "Field `PA2_AFCFG` reader - PA2 AF function configuration bitse"]
pub type PA2_AFCFG_R = crate::BitReader;
#[doc = "Field `PA2_AFCFG` writer - PA2 AF function configuration bitse"]
pub type PA2_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PA3_AFCFG` reader - PA3 AF function configuration bitse"]
pub type PA3_AFCFG_R = crate::BitReader;
#[doc = "Field `PA3_AFCFG` writer - PA3 AF function configuration bitse"]
pub type PA3_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PA5_AFCFG` reader - PA5 AF function configuration bitse"]
pub type PA5_AFCFG_R = crate::BitReader;
#[doc = "Field `PA5_AFCFG` writer - PA5 AF function configuration bitse"]
pub type PA5_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PA8_AFCFG` reader - PA8 AF function configuration bitse"]
pub type PA8_AFCFG_R = crate::FieldReader;
#[doc = "Field `PA8_AFCFG` writer - PA8 AF function configuration bitse"]
pub type PA8_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PA9_AFCFG` reader - PA9 AF function configuration bitse"]
pub type PA9_AFCFG_R = crate::FieldReader;
#[doc = "Field `PA9_AFCFG` writer - PA9 AF function configuration bitse"]
pub type PA9_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PA10_AFCFG` reader - PA10 AF function configuration bitse"]
pub type PA10_AFCFG_R = crate::FieldReader;
#[doc = "Field `PA10_AFCFG` writer - PA10 AF function configuration bitse"]
pub type PA10_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PA11_AFCFG` reader - PA11 AF function configuration bitse"]
pub type PA11_AFCFG_R = crate::FieldReader;
#[doc = "Field `PA11_AFCFG` writer - PA11 AF function configuration bitse"]
pub type PA11_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PA12_AFCFG` reader - PA12 AF function configuration bitse"]
pub type PA12_AFCFG_R = crate::FieldReader;
#[doc = "Field `PA12_AFCFG` writer - PA12 AF function configuration bitse"]
pub type PA12_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PA15_AFCFG` reader - PA15 AF function configuration bit"]
pub type PA15_AFCFG_R = crate::BitReader;
#[doc = "Field `PA15_AFCFG` writer - PA15 AF function configuration bit"]
pub type PA15_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 4 - PA2 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa2_afcfg(&self) -> PA2_AFCFG_R {
        PA2_AFCFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - PA3 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa3_afcfg(&self) -> PA3_AFCFG_R {
        PA3_AFCFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - PA5 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa5_afcfg(&self) -> PA5_AFCFG_R {
        PA5_AFCFG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:17 - PA8 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa8_afcfg(&self) -> PA8_AFCFG_R {
        PA8_AFCFG_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PA9 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa9_afcfg(&self) -> PA9_AFCFG_R {
        PA9_AFCFG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PA10 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa10_afcfg(&self) -> PA10_AFCFG_R {
        PA10_AFCFG_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PA11 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa11_afcfg(&self) -> PA11_AFCFG_R {
        PA11_AFCFG_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PA12 AF function configuration bitse"]
    #[inline(always)]
    pub fn pa12_afcfg(&self) -> PA12_AFCFG_R {
        PA12_AFCFG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - PA15 AF function configuration bit"]
    #[inline(always)]
    pub fn pa15_afcfg(&self) -> PA15_AFCFG_R {
        PA15_AFCFG_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PA2 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa2_afcfg(&mut self) -> PA2_AFCFG_W<PCFA_SPEC, 4> {
        PA2_AFCFG_W::new(self)
    }
    #[doc = "Bit 6 - PA3 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa3_afcfg(&mut self) -> PA3_AFCFG_W<PCFA_SPEC, 6> {
        PA3_AFCFG_W::new(self)
    }
    #[doc = "Bit 10 - PA5 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa5_afcfg(&mut self) -> PA5_AFCFG_W<PCFA_SPEC, 10> {
        PA5_AFCFG_W::new(self)
    }
    #[doc = "Bits 16:17 - PA8 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa8_afcfg(&mut self) -> PA8_AFCFG_W<PCFA_SPEC, 16> {
        PA8_AFCFG_W::new(self)
    }
    #[doc = "Bits 18:19 - PA9 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa9_afcfg(&mut self) -> PA9_AFCFG_W<PCFA_SPEC, 18> {
        PA9_AFCFG_W::new(self)
    }
    #[doc = "Bits 20:21 - PA10 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa10_afcfg(&mut self) -> PA10_AFCFG_W<PCFA_SPEC, 20> {
        PA10_AFCFG_W::new(self)
    }
    #[doc = "Bits 22:23 - PA11 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa11_afcfg(&mut self) -> PA11_AFCFG_W<PCFA_SPEC, 22> {
        PA11_AFCFG_W::new(self)
    }
    #[doc = "Bits 24:25 - PA12 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pa12_afcfg(&mut self) -> PA12_AFCFG_W<PCFA_SPEC, 24> {
        PA12_AFCFG_W::new(self)
    }
    #[doc = "Bit 30 - PA15 AF function configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pa15_afcfg(&mut self) -> PA15_AFCFG_W<PCFA_SPEC, 30> {
        PA15_AFCFG_W::new(self)
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
#[doc = "AFIO port configuration register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFA_SPEC;
impl crate::RegisterSpec for PCFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfa::R`](R) reader structure"]
impl crate::Readable for PCFA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcfa::W`](W) writer structure"]
impl crate::Writable for PCFA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFA to value 0"]
impl crate::Resettable for PCFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
