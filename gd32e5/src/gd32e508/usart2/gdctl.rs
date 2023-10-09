#[doc = "Register `GDCTL` reader"]
pub type R = crate::R<GDCTL_SPEC>;
#[doc = "Register `GDCTL` writer"]
pub type W = crate::W<GDCTL_SPEC>;
#[doc = "Field `CDEN` reader - Collision detection enable"]
pub type CDEN_R = crate::BitReader;
#[doc = "Field `CDEN` writer - Collision detection enable"]
pub type CDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CD` writer - Collision detected status"]
pub type CD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDIE` reader - Collision detected interrupt enable"]
pub type CDIE_R = crate::BitReader;
#[doc = "Field `CDIE` writer - Collision detected interrupt enable"]
pub type CDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Collision detection enable"]
    #[inline(always)]
    pub fn cden(&self) -> CDEN_R {
        CDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Collision detected interrupt enable"]
    #[inline(always)]
    pub fn cdie(&self) -> CDIE_R {
        CDIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Collision detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn cden(&mut self) -> CDEN_W<GDCTL_SPEC, 1> {
        CDEN_W::new(self)
    }
    #[doc = "Bit 8 - Collision detected status"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<GDCTL_SPEC, 8> {
        CD_W::new(self)
    }
    #[doc = "Bit 16 - Collision detected interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdie(&mut self) -> CDIE_W<GDCTL_SPEC, 16> {
        CDIE_W::new(self)
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
#[doc = "GD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDCTL_SPEC;
impl crate::RegisterSpec for GDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdctl::R`](R) reader structure"]
impl crate::Readable for GDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdctl::W`](W) writer structure"]
impl crate::Writable for GDCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GDCTL to value 0"]
impl crate::Resettable for GDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
