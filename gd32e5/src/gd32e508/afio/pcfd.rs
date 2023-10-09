#[doc = "Register `PCFD` reader"]
pub type R = crate::R<PCFD_SPEC>;
#[doc = "Register `PCFD` writer"]
pub type W = crate::W<PCFD_SPEC>;
#[doc = "Field `PD4_AFCFG` reader - PD4 AF function configuration bitse"]
pub type PD4_AFCFG_R = crate::BitReader;
#[doc = "Field `PD4_AFCFG` writer - PD4 AF function configuration bitse"]
pub type PD4_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD5_AFCFG` reader - PD5 AF function configuration bitse"]
pub type PD5_AFCFG_R = crate::BitReader;
#[doc = "Field `PD5_AFCFG` writer - PD5 AF function configuration bitse"]
pub type PD5_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 8 - PD4 AF function configuration bitse"]
    #[inline(always)]
    pub fn pd4_afcfg(&self) -> PD4_AFCFG_R {
        PD4_AFCFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - PD5 AF function configuration bitse"]
    #[inline(always)]
    pub fn pd5_afcfg(&self) -> PD5_AFCFG_R {
        PD5_AFCFG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PD4 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pd4_afcfg(&mut self) -> PD4_AFCFG_W<PCFD_SPEC, 8> {
        PD4_AFCFG_W::new(self)
    }
    #[doc = "Bit 10 - PD5 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pd5_afcfg(&mut self) -> PD5_AFCFG_W<PCFD_SPEC, 10> {
        PD5_AFCFG_W::new(self)
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
#[doc = "AFIO port configuration register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFD_SPEC;
impl crate::RegisterSpec for PCFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfd::R`](R) reader structure"]
impl crate::Readable for PCFD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcfd::W`](W) writer structure"]
impl crate::Writable for PCFD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFD to value 0"]
impl crate::Resettable for PCFD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
