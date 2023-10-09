#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `OUTSEL` reader - The output value selection"]
pub type OUTSEL_R = crate::BitReader;
#[doc = "Field `OUTSEL` writer - The output value selection"]
pub type OUTSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHVSEL` reader - Write CHxVAL register selection"]
pub type CHVSEL_R = crate::BitReader;
#[doc = "Field `CHVSEL` writer - Write CHxVAL register selection"]
pub type CHVSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&self) -> CHVSEL_R {
        CHVSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<CFG_SPEC, 0> {
        OUTSEL_W::new(self)
    }
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    #[must_use]
    pub fn chvsel(&mut self) -> CHVSEL_W<CFG_SPEC, 1> {
        CHVSEL_W::new(self)
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
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
