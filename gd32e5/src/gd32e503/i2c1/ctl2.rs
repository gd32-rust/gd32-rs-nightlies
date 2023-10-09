#[doc = "Register `CTL2` reader"]
pub type R = crate::R<CTL2_SPEC>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<CTL2_SPEC>;
#[doc = "Field `FMPEN` reader - Fast mode plus enable"]
pub type FMPEN_R = crate::BitReader;
#[doc = "Field `FMPEN` writer - Fast mode plus enable"]
pub type FMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETM` reader - Start Early Termination Mode"]
pub type SETM_R = crate::BitReader;
#[doc = "Field `SETM` writer - Start Early Termination Mode"]
pub type SETM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOEN` reader - Timeout calculation enable"]
pub type TOEN_R = crate::BitReader;
#[doc = "Field `TOEN` writer - Timeout calculation enable"]
pub type TOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RADD` reader - slave address recorde enable"]
pub type RADD_R = crate::BitReader;
#[doc = "Field `RADD` writer - slave address recorde enable"]
pub type RADD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDM` reader - ingnore specify bits"]
pub type ADDM_R = crate::FieldReader;
#[doc = "Field `ADDM` writer - ingnore specify bits"]
pub type ADDM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Early Termination Mode"]
    #[inline(always)]
    pub fn setm(&self) -> SETM_R {
        SETM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout calculation enable"]
    #[inline(always)]
    pub fn toen(&self) -> TOEN_R {
        TOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - slave address recorde enable"]
    #[inline(always)]
    pub fn radd(&self) -> RADD_R {
        RADD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - ingnore specify bits"]
    #[inline(always)]
    pub fn addm(&self) -> ADDM_R {
        ADDM_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FMPEN_W<CTL2_SPEC, 0> {
        FMPEN_W::new(self)
    }
    #[doc = "Bit 1 - Start Early Termination Mode"]
    #[inline(always)]
    #[must_use]
    pub fn setm(&mut self) -> SETM_W<CTL2_SPEC, 1> {
        SETM_W::new(self)
    }
    #[doc = "Bit 4 - Timeout calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn toen(&mut self) -> TOEN_W<CTL2_SPEC, 4> {
        TOEN_W::new(self)
    }
    #[doc = "Bit 8 - slave address recorde enable"]
    #[inline(always)]
    #[must_use]
    pub fn radd(&mut self) -> RADD_W<CTL2_SPEC, 8> {
        RADD_W::new(self)
    }
    #[doc = "Bits 9:15 - ingnore specify bits"]
    #[inline(always)]
    #[must_use]
    pub fn addm(&mut self) -> ADDM_W<CTL2_SPEC, 9> {
        ADDM_W::new(self)
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
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL2_SPEC;
impl crate::RegisterSpec for CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0xfe00"]
impl crate::Resettable for CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfe00;
}
