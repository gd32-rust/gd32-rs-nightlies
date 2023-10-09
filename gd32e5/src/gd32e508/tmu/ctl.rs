#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `TMUEN` reader - start TMU module calculation"]
pub type TMUEN_R = crate::BitReader;
#[doc = "Field `TMUEN` writer - start TMU module calculation"]
pub type TMUEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - Set the mode of TMU"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Set the mode of TMU"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CFIE` reader - CFIE"]
pub type CFIE_R = crate::BitReader;
#[doc = "Field `CFIE` writer - CFIE"]
pub type CFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFIF` reader - CFIF"]
pub type CFIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - start TMU module calculation"]
    #[inline(always)]
    pub fn tmuen(&self) -> TMUEN_R {
        TMUEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Set the mode of TMU"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - CFIE"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CFIF"]
    #[inline(always)]
    pub fn cfif(&self) -> CFIF_R {
        CFIF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - start TMU module calculation"]
    #[inline(always)]
    #[must_use]
    pub fn tmuen(&mut self) -> TMUEN_W<CTL_SPEC, 0> {
        TMUEN_W::new(self)
    }
    #[doc = "Bits 1:4 - Set the mode of TMU"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTL_SPEC, 1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 5 - CFIE"]
    #[inline(always)]
    #[must_use]
    pub fn cfie(&mut self) -> CFIE_W<CTL_SPEC, 5> {
        CFIE_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
