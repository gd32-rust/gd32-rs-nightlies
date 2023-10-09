#[doc = "Register `NPINTEN2` reader"]
pub type R = crate::R<NPINTEN2_SPEC>;
#[doc = "Register `NPINTEN2` writer"]
pub type W = crate::W<NPINTEN2_SPEC>;
#[doc = "Field `INTRS` reader - Interrupt rising edge status"]
pub type INTRS_R = crate::BitReader;
#[doc = "Field `INTRS` writer - Interrupt rising edge status"]
pub type INTRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTHS` reader - Interrupt high-level status"]
pub type INTHS_R = crate::BitReader;
#[doc = "Field `INTHS` writer - Interrupt high-level status"]
pub type INTHS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTFS` reader - Interrupt falling edge status"]
pub type INTFS_R = crate::BitReader;
#[doc = "Field `INTFS` writer - Interrupt falling edge status"]
pub type INTFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTREN` reader - Interrupt rising edge detection enable bit"]
pub type INTREN_R = crate::BitReader;
#[doc = "Field `INTREN` writer - Interrupt rising edge detection enable bit"]
pub type INTREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTHEN` reader - Interrupt high-level detection enable"]
pub type INTHEN_R = crate::BitReader;
#[doc = "Field `INTHEN` writer - Interrupt high-level detection enable"]
pub type INTHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTFEN` reader - Interrupt falling edge detection enable"]
pub type INTFEN_R = crate::BitReader;
#[doc = "Field `INTFEN` writer - Interrupt falling edge detection enable"]
pub type INTFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FFEPT` reader - FIFO empty flag"]
pub type FFEPT_R = crate::BitReader;
#[doc = "Field `FFEPT` writer - FIFO empty flag"]
pub type FFEPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn intrs(&self) -> INTRS_R {
        INTRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn inths(&self) -> INTHS_R {
        INTHS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn intfs(&self) -> INTFS_R {
        INTFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn intren(&self) -> INTREN_R {
        INTREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    pub fn inthen(&self) -> INTHEN_R {
        INTHEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    pub fn intfen(&self) -> INTFEN_R {
        INTFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty flag"]
    #[inline(always)]
    pub fn ffept(&self) -> FFEPT_R {
        FFEPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    #[must_use]
    pub fn intrs(&mut self) -> INTRS_W<NPINTEN2_SPEC, 0> {
        INTRS_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    #[must_use]
    pub fn inths(&mut self) -> INTHS_W<NPINTEN2_SPEC, 1> {
        INTHS_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    #[must_use]
    pub fn intfs(&mut self) -> INTFS_W<NPINTEN2_SPEC, 2> {
        INTFS_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn intren(&mut self) -> INTREN_W<NPINTEN2_SPEC, 3> {
        INTREN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn inthen(&mut self) -> INTHEN_W<NPINTEN2_SPEC, 4> {
        INTHEN_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn intfen(&mut self) -> INTFEN_W<NPINTEN2_SPEC, 5> {
        INTFEN_W::new(self)
    }
    #[doc = "Bit 6 - FIFO empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn ffept(&mut self) -> FFEPT_W<NPINTEN2_SPEC, 6> {
        FFEPT_W::new(self)
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
#[doc = "NAND flash/PC card interrupt enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npinten2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npinten2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NPINTEN2_SPEC;
impl crate::RegisterSpec for NPINTEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npinten2::R`](R) reader structure"]
impl crate::Readable for NPINTEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`npinten2::W`](W) writer structure"]
impl crate::Writable for NPINTEN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NPINTEN2 to value 0x42"]
impl crate::Resettable for NPINTEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0x42;
}
