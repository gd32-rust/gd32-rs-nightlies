#[doc = "Register `NPINTEN3` reader"]
pub struct R(crate::R<NPINTEN3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NPINTEN3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NPINTEN3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NPINTEN3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NPINTEN3` writer"]
pub struct W(crate::W<NPINTEN3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NPINTEN3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NPINTEN3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NPINTEN3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFEPT` reader - FIFO empty flag"]
pub type FFEPT_R = crate::BitReader<bool>;
#[doc = "Field `INTFEN` reader - Interrupt falling edge detection enable"]
pub type INTFEN_R = crate::BitReader<bool>;
#[doc = "Field `INTFEN` writer - Interrupt falling edge detection enable"]
pub type INTFEN_W<'a> = crate::BitWriter<'a, u32, NPINTEN3_SPEC, bool, 5>;
#[doc = "Field `INTHEN` reader - Interrupt high-level detection enable"]
pub type INTHEN_R = crate::BitReader<bool>;
#[doc = "Field `INTHEN` writer - Interrupt high-level detection enable"]
pub type INTHEN_W<'a> = crate::BitWriter<'a, u32, NPINTEN3_SPEC, bool, 4>;
#[doc = "Field `INTREN` reader - Interrupt rising edge detection enable bit"]
pub type INTREN_R = crate::BitReader<bool>;
#[doc = "Field `INTREN` writer - Interrupt rising edge detection enable bit"]
pub type INTREN_W<'a> = crate::BitWriter<'a, u32, NPINTEN3_SPEC, bool, 3>;
#[doc = "Field `INTFS` reader - Interrupt falling edge status"]
pub type INTFS_R = crate::BitReader<bool>;
#[doc = "Field `INTFS` writer - Interrupt falling edge status"]
pub type INTFS_W<'a> = crate::BitWriter<'a, u32, NPINTEN3_SPEC, bool, 2>;
#[doc = "Field `INTHS` reader - Interrupt high-level status"]
pub type INTHS_R = crate::BitReader<bool>;
#[doc = "Field `INTHS` writer - Interrupt high-level status"]
pub type INTHS_W<'a> = crate::BitWriter<'a, u32, NPINTEN3_SPEC, bool, 1>;
#[doc = "Field `INTRS` reader - Interrupt rising edge status"]
pub type INTRS_R = crate::BitReader<bool>;
#[doc = "Field `INTRS` writer - Interrupt rising edge status"]
pub type INTRS_W<'a> = crate::BitWriter<'a, u32, NPINTEN3_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 6 - FIFO empty flag"]
    #[inline(always)]
    pub fn ffept(&self) -> FFEPT_R {
        FFEPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    pub fn intfen(&self) -> INTFEN_R {
        INTFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    pub fn inthen(&self) -> INTHEN_R {
        INTHEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn intren(&self) -> INTREN_R {
        INTREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn intfs(&self) -> INTFS_R {
        INTFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn inths(&self) -> INTHS_R {
        INTHS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn intrs(&self) -> INTRS_R {
        INTRS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Interrupt falling edge detection enable"]
    #[inline(always)]
    pub fn intfen(&mut self) -> INTFEN_W {
        INTFEN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable"]
    #[inline(always)]
    pub fn inthen(&mut self) -> INTHEN_W {
        INTHEN_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn intren(&mut self) -> INTREN_W {
        INTREN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn intfs(&mut self) -> INTFS_W {
        INTFS_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn inths(&mut self) -> INTHS_W {
        INTHS_W::new(self)
    }
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn intrs(&mut self) -> INTRS_W {
        INTRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND flash/PC card interrupt enable register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npinten3](index.html) module"]
pub struct NPINTEN3_SPEC;
impl crate::RegisterSpec for NPINTEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [npinten3::R](R) reader structure"]
impl crate::Readable for NPINTEN3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [npinten3::W](W) writer structure"]
impl crate::Writable for NPINTEN3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NPINTEN3 to value 0x40"]
impl crate::Resettable for NPINTEN3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
