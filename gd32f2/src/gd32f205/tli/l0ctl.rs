#[doc = "Register `L0CTL` reader"]
pub type R = crate::R<L0CTL_SPEC>;
#[doc = "Register `L0CTL` writer"]
pub type W = crate::W<L0CTL_SPEC>;
#[doc = "Field `LEN` reader - Layer enable"]
pub type LEN_R = crate::BitReader;
#[doc = "Field `LEN` writer - Layer enable"]
pub type LEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKEYEN` reader - Color keying enable"]
pub type CKEYEN_R = crate::BitReader;
#[doc = "Field `CKEYEN` writer - Color keying enable"]
pub type CKEYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LUTEN` reader - LUT enable"]
pub type LUTEN_R = crate::BitReader;
#[doc = "Field `LUTEN` writer - LUT enable"]
pub type LUTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Layer enable"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Color keying enable"]
    #[inline(always)]
    pub fn ckeyen(&self) -> CKEYEN_R {
        CKEYEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LUT enable"]
    #[inline(always)]
    pub fn luten(&self) -> LUTEN_R {
        LUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer enable"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<L0CTL_SPEC, 0> {
        LEN_W::new(self)
    }
    #[doc = "Bit 1 - Color keying enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckeyen(&mut self) -> CKEYEN_W<L0CTL_SPEC, 1> {
        CKEYEN_W::new(self)
    }
    #[doc = "Bit 4 - LUT enable"]
    #[inline(always)]
    #[must_use]
    pub fn luten(&mut self) -> LUTEN_W<L0CTL_SPEC, 4> {
        LUTEN_W::new(self)
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
#[doc = "Layer 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0CTL_SPEC;
impl crate::RegisterSpec for L0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0ctl::R`](R) reader structure"]
impl crate::Readable for L0CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l0ctl::W`](W) writer structure"]
impl crate::Writable for L0CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L0CTL to value 0"]
impl crate::Resettable for L0CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
