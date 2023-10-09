#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `DPMOD1` reader - Enable deep-sleep 1 mode"]
pub type DPMOD1_R = crate::BitReader;
#[doc = "Field `DPMOD1` writer - Enable deep-sleep 1 mode"]
pub type DPMOD1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPMOD2` reader - Enable deep-sleep 2 mode"]
pub type DPMOD2_R = crate::BitReader;
#[doc = "Field `DPMOD2` writer - Enable deep-sleep 2 mode"]
pub type DPMOD2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable deep-sleep 1 mode"]
    #[inline(always)]
    pub fn dpmod1(&self) -> DPMOD1_R {
        DPMOD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable deep-sleep 2 mode"]
    #[inline(always)]
    pub fn dpmod2(&self) -> DPMOD2_R {
        DPMOD2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable deep-sleep 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn dpmod1(&mut self) -> DPMOD1_W<CTL1_SPEC, 0> {
        DPMOD1_W::new(self)
    }
    #[doc = "Bit 1 - Enable deep-sleep 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn dpmod2(&mut self) -> DPMOD2_W<CTL1_SPEC, 1> {
        DPMOD2_W::new(self)
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
#[doc = "power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
