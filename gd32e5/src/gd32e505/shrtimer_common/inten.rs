#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `SYSFLTIE` reader - System fault interrupt enable"]
pub type SYSFLTIE_R = crate::BitReader;
#[doc = "Field `SYSFLTIE` writer - System fault interrupt enable"]
pub type SYSFLTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLLCALIE` reader - DLL calibration completed interrupt enable"]
pub type DLLCALIE_R = crate::BitReader;
#[doc = "Field `DLLCALIE` writer - DLL calibration completed interrupt enable"]
pub type DLLCALIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BMPERIE` reader - Bunch mode period interrupt enable"]
pub type BMPERIE_R = crate::BitReader;
#[doc = "Field `BMPERIE` writer - Bunch mode period interrupt enable"]
pub type BMPERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - System fault interrupt enable"]
    #[inline(always)]
    pub fn sysfltie(&self) -> SYSFLTIE_R {
        SYSFLTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL calibration completed interrupt enable"]
    #[inline(always)]
    pub fn dllcalie(&self) -> DLLCALIE_R {
        DLLCALIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bunch mode period interrupt enable"]
    #[inline(always)]
    pub fn bmperie(&self) -> BMPERIE_R {
        BMPERIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - System fault interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysfltie(&mut self) -> SYSFLTIE_W<INTEN_SPEC, 5> {
        SYSFLTIE_W::new(self)
    }
    #[doc = "Bit 16 - DLL calibration completed interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dllcalie(&mut self) -> DLLCALIE_W<INTEN_SPEC, 16> {
        DLLCALIE_W::new(self)
    }
    #[doc = "Bit 17 - Bunch mode period interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmperie(&mut self) -> BMPERIE_W<INTEN_SPEC, 17> {
        BMPERIE_W::new(self)
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
#[doc = "SHRTIMER interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
