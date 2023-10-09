#[doc = "Register `ADDCFG` reader"]
pub type R = crate::R<ADDCFG_SPEC>;
#[doc = "Register `ADDCFG` writer"]
pub type W = crate::W<ADDCFG_SPEC>;
#[doc = "Field `PLLUSBPREDV` reader - PLLUSBPREDV division factor"]
pub type PLLUSBPREDV_R = crate::FieldReader;
#[doc = "Field `PLLUSBPREDV` writer - PLLUSBPREDV division factor"]
pub type PLLUSBPREDV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLLUSBPRESEL` reader - PLLUSB clock source preselection"]
pub type PLLUSBPRESEL_R = crate::BitReader;
#[doc = "Field `PLLUSBPRESEL` writer - PLLUSB clock source preselection"]
pub type PLLUSBPRESEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLUSBPREDVSEL` reader - PLLUSBPREDV input Clock Source Selection"]
pub type PLLUSBPREDVSEL_R = crate::BitReader;
#[doc = "Field `PLLUSBPREDVSEL` writer - PLLUSBPREDV input Clock Source Selection"]
pub type PLLUSBPREDVSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLUSBMF` reader - The PLLUSB clock multiplication factor"]
pub type PLLUSBMF_R = crate::FieldReader;
#[doc = "Field `PLLUSBMF` writer - The PLLUSB clock multiplication factor"]
pub type PLLUSBMF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:3 - PLLUSBPREDV division factor"]
    #[inline(always)]
    pub fn pllusbpredv(&self) -> PLLUSBPREDV_R {
        PLLUSBPREDV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PLLUSB clock source preselection"]
    #[inline(always)]
    pub fn pllusbpresel(&self) -> PLLUSBPRESEL_R {
        PLLUSBPRESEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLLUSBPREDV input Clock Source Selection"]
    #[inline(always)]
    pub fn pllusbpredvsel(&self) -> PLLUSBPREDVSEL_R {
        PLLUSBPREDVSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:24 - The PLLUSB clock multiplication factor"]
    #[inline(always)]
    pub fn pllusbmf(&self) -> PLLUSBMF_R {
        PLLUSBMF_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLLUSBPREDV division factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllusbpredv(&mut self) -> PLLUSBPREDV_W<ADDCFG_SPEC, 0> {
        PLLUSBPREDV_W::new(self)
    }
    #[doc = "Bit 16 - PLLUSB clock source preselection"]
    #[inline(always)]
    #[must_use]
    pub fn pllusbpresel(&mut self) -> PLLUSBPRESEL_W<ADDCFG_SPEC, 16> {
        PLLUSBPRESEL_W::new(self)
    }
    #[doc = "Bit 17 - PLLUSBPREDV input Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pllusbpredvsel(&mut self) -> PLLUSBPREDVSEL_W<ADDCFG_SPEC, 17> {
        PLLUSBPREDVSEL_W::new(self)
    }
    #[doc = "Bits 18:24 - The PLLUSB clock multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllusbmf(&mut self) -> PLLUSBMF_W<ADDCFG_SPEC, 18> {
        PLLUSBMF_W::new(self)
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
#[doc = "Additional clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDCFG_SPEC;
impl crate::RegisterSpec for ADDCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addcfg::R`](R) reader structure"]
impl crate::Readable for ADDCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addcfg::W`](W) writer structure"]
impl crate::Writable for ADDCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDCFG to value 0"]
impl crate::Resettable for ADDCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
