#[doc = "Register `PCFG` reader"]
pub type R = crate::R<PCFG_SPEC>;
#[doc = "Register `PCFG` writer"]
pub type W = crate::W<PCFG_SPEC>;
#[doc = "Field `PG6_AFCFG` reader - PG6 AF function configuration bitse"]
pub type PG6_AFCFG_R = crate::BitReader;
#[doc = "Field `PG6_AFCFG` writer - PG6 AF function configuration bitse"]
pub type PG6_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PG7_AFCFG` reader - PG7 AF function configuration bitse"]
pub type PG7_AFCFG_R = crate::FieldReader;
#[doc = "Field `PG7_AFCFG` writer - PG7 AF function configuration bitse"]
pub type PG7_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PG9_AFCFG` reader - PG9 AF function configuration bitse"]
pub type PG9_AFCFG_R = crate::FieldReader;
#[doc = "Field `PG9_AFCFG` writer - PG9 AF function configuration bitse"]
pub type PG9_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PG10_AFCFG` reader - PG10 AF function configuration bitse"]
pub type PG10_AFCFG_R = crate::BitReader;
#[doc = "Field `PG10_AFCFG` writer - PG10 AF function configuration bitse"]
pub type PG10_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PG11_AFCFG` reader - PG11 AF function configuration bitse"]
pub type PG11_AFCFG_R = crate::FieldReader;
#[doc = "Field `PG11_AFCFG` writer - PG11 AF function configuration bitse"]
pub type PG11_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PG12_AFCFG` reader - PG12 AF function configuration bitse"]
pub type PG12_AFCFG_R = crate::BitReader;
#[doc = "Field `PG12_AFCFG` writer - PG12 AF function configuration bitse"]
pub type PG12_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PG13_AFCFG` reader - PG13 AF function configuration bitse"]
pub type PG13_AFCFG_R = crate::BitReader;
#[doc = "Field `PG13_AFCFG` writer - PG13 AF function configuration bitse"]
pub type PG13_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PG14_AFCFG` reader - PG14 AF function configuration bitse"]
pub type PG14_AFCFG_R = crate::BitReader;
#[doc = "Field `PG14_AFCFG` writer - PG14 AF function configuration bitse"]
pub type PG14_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 12 - PG6 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg6_afcfg(&self) -> PG6_AFCFG_R {
        PG6_AFCFG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - PG7 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg7_afcfg(&self) -> PG7_AFCFG_R {
        PG7_AFCFG_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PG9 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg9_afcfg(&self) -> PG9_AFCFG_R {
        PG9_AFCFG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - PG10 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg10_afcfg(&self) -> PG10_AFCFG_R {
        PG10_AFCFG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - PG11 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg11_afcfg(&self) -> PG11_AFCFG_R {
        PG11_AFCFG_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PG12 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg12_afcfg(&self) -> PG12_AFCFG_R {
        PG12_AFCFG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PG13 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg13_afcfg(&self) -> PG13_AFCFG_R {
        PG13_AFCFG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - PG14 AF function configuration bitse"]
    #[inline(always)]
    pub fn pg14_afcfg(&self) -> PG14_AFCFG_R {
        PG14_AFCFG_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - PG6 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg6_afcfg(&mut self) -> PG6_AFCFG_W<PCFG_SPEC, 12> {
        PG6_AFCFG_W::new(self)
    }
    #[doc = "Bits 14:15 - PG7 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg7_afcfg(&mut self) -> PG7_AFCFG_W<PCFG_SPEC, 14> {
        PG7_AFCFG_W::new(self)
    }
    #[doc = "Bits 18:19 - PG9 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg9_afcfg(&mut self) -> PG9_AFCFG_W<PCFG_SPEC, 18> {
        PG9_AFCFG_W::new(self)
    }
    #[doc = "Bit 20 - PG10 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg10_afcfg(&mut self) -> PG10_AFCFG_W<PCFG_SPEC, 20> {
        PG10_AFCFG_W::new(self)
    }
    #[doc = "Bits 22:23 - PG11 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg11_afcfg(&mut self) -> PG11_AFCFG_W<PCFG_SPEC, 22> {
        PG11_AFCFG_W::new(self)
    }
    #[doc = "Bit 24 - PG12 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg12_afcfg(&mut self) -> PG12_AFCFG_W<PCFG_SPEC, 24> {
        PG12_AFCFG_W::new(self)
    }
    #[doc = "Bit 26 - PG13 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg13_afcfg(&mut self) -> PG13_AFCFG_W<PCFG_SPEC, 26> {
        PG13_AFCFG_W::new(self)
    }
    #[doc = "Bit 28 - PG14 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pg14_afcfg(&mut self) -> PG14_AFCFG_W<PCFG_SPEC, 28> {
        PG14_AFCFG_W::new(self)
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
#[doc = "AFIO port configuration register G\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFG_SPEC;
impl crate::RegisterSpec for PCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfg::R`](R) reader structure"]
impl crate::Readable for PCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcfg::W`](W) writer structure"]
impl crate::Writable for PCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFG to value 0"]
impl crate::Resettable for PCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
