#[doc = "Register `EXEVCFG0` reader"]
pub type R = crate::R<EXEVCFG0_SPEC>;
#[doc = "Register `EXEVCFG0` writer"]
pub type W = crate::W<EXEVCFG0_SPEC>;
#[doc = "Field `EXEV0SRC` reader - External event 0 source"]
pub type EXEV0SRC_R = crate::FieldReader;
#[doc = "Field `EXEV0SRC` writer - External event 0 source"]
pub type EXEV0SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV0P` reader - External event 0 polarity"]
pub type EXEV0P_R = crate::BitReader;
#[doc = "Field `EXEV0P` writer - External event 0 polarity"]
pub type EXEV0P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV0EG` reader - External event 0 edge sensitivity"]
pub type EXEV0EG_R = crate::FieldReader;
#[doc = "Field `EXEV0EG` writer - External event 0 edge sensitivity"]
pub type EXEV0EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV1SRC` reader - External event 1 source"]
pub type EXEV1SRC_R = crate::FieldReader;
#[doc = "Field `EXEV1SRC` writer - External event 1 source"]
pub type EXEV1SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV1P` reader - External event 1 polarity"]
pub type EXEV1P_R = crate::BitReader;
#[doc = "Field `EXEV1P` writer - External event 1 polarity"]
pub type EXEV1P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV1EG` reader - External event 1 edge sensitivity"]
pub type EXEV1EG_R = crate::FieldReader;
#[doc = "Field `EXEV1EG` writer - External event 1 edge sensitivity"]
pub type EXEV1EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV2SRC` reader - External event 2 source"]
pub type EXEV2SRC_R = crate::FieldReader;
#[doc = "Field `EXEV2SRC` writer - External event 2 source"]
pub type EXEV2SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV2P` reader - External event 2 polarity"]
pub type EXEV2P_R = crate::BitReader;
#[doc = "Field `EXEV2P` writer - External event 2 polarity"]
pub type EXEV2P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV2EG` reader - External event 2 edge sensitivity"]
pub type EXEV2EG_R = crate::FieldReader;
#[doc = "Field `EXEV2EG` writer - External event 2 edge sensitivity"]
pub type EXEV2EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV3SRC` reader - External event 3 source"]
pub type EXEV3SRC_R = crate::FieldReader;
#[doc = "Field `EXEV3SRC` writer - External event 3 source"]
pub type EXEV3SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV3P` reader - External event 3 polarity"]
pub type EXEV3P_R = crate::BitReader;
#[doc = "Field `EXEV3P` writer - External event 3 polarity"]
pub type EXEV3P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV3EG` reader - External event 3 edge sensitivity"]
pub type EXEV3EG_R = crate::FieldReader;
#[doc = "Field `EXEV3EG` writer - External event 3 edge sensitivity"]
pub type EXEV3EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV4SRC` reader - External event 4 source"]
pub type EXEV4SRC_R = crate::FieldReader;
#[doc = "Field `EXEV4SRC` writer - External event 4 source"]
pub type EXEV4SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV4P` reader - External event 4 polarity"]
pub type EXEV4P_R = crate::BitReader;
#[doc = "Field `EXEV4P` writer - External event 4 polarity"]
pub type EXEV4P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV4EG` reader - External event 4 edge sensitivity"]
pub type EXEV4EG_R = crate::FieldReader;
#[doc = "Field `EXEV4EG` writer - External event 4 edge sensitivity"]
pub type EXEV4EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - External event 0 source"]
    #[inline(always)]
    pub fn exev0src(&self) -> EXEV0SRC_R {
        EXEV0SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External event 0 polarity"]
    #[inline(always)]
    pub fn exev0p(&self) -> EXEV0P_R {
        EXEV0P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - External event 0 edge sensitivity"]
    #[inline(always)]
    pub fn exev0eg(&self) -> EXEV0EG_R {
        EXEV0EG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External event 1 source"]
    #[inline(always)]
    pub fn exev1src(&self) -> EXEV1SRC_R {
        EXEV1SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - External event 1 polarity"]
    #[inline(always)]
    pub fn exev1p(&self) -> EXEV1P_R {
        EXEV1P_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - External event 1 edge sensitivity"]
    #[inline(always)]
    pub fn exev1eg(&self) -> EXEV1EG_R {
        EXEV1EG_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External event 2 source"]
    #[inline(always)]
    pub fn exev2src(&self) -> EXEV2SRC_R {
        EXEV2SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External event 2 polarity"]
    #[inline(always)]
    pub fn exev2p(&self) -> EXEV2P_R {
        EXEV2P_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - External event 2 edge sensitivity"]
    #[inline(always)]
    pub fn exev2eg(&self) -> EXEV2EG_R {
        EXEV2EG_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 18:19 - External event 3 source"]
    #[inline(always)]
    pub fn exev3src(&self) -> EXEV3SRC_R {
        EXEV3SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - External event 3 polarity"]
    #[inline(always)]
    pub fn exev3p(&self) -> EXEV3P_R {
        EXEV3P_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - External event 3 edge sensitivity"]
    #[inline(always)]
    pub fn exev3eg(&self) -> EXEV3EG_R {
        EXEV3EG_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External event 4 source"]
    #[inline(always)]
    pub fn exev4src(&self) -> EXEV4SRC_R {
        EXEV4SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External event 4 polarity"]
    #[inline(always)]
    pub fn exev4p(&self) -> EXEV4P_R {
        EXEV4P_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External event 4 edge sensitivity"]
    #[inline(always)]
    pub fn exev4eg(&self) -> EXEV4EG_R {
        EXEV4EG_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External event 0 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev0src(&mut self) -> EXEV0SRC_W<EXEVCFG0_SPEC, 0> {
        EXEV0SRC_W::new(self)
    }
    #[doc = "Bit 2 - External event 0 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev0p(&mut self) -> EXEV0P_W<EXEVCFG0_SPEC, 2> {
        EXEV0P_W::new(self)
    }
    #[doc = "Bits 3:4 - External event 0 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev0eg(&mut self) -> EXEV0EG_W<EXEVCFG0_SPEC, 3> {
        EXEV0EG_W::new(self)
    }
    #[doc = "Bits 6:7 - External event 1 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev1src(&mut self) -> EXEV1SRC_W<EXEVCFG0_SPEC, 6> {
        EXEV1SRC_W::new(self)
    }
    #[doc = "Bit 8 - External event 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev1p(&mut self) -> EXEV1P_W<EXEVCFG0_SPEC, 8> {
        EXEV1P_W::new(self)
    }
    #[doc = "Bits 9:10 - External event 1 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev1eg(&mut self) -> EXEV1EG_W<EXEVCFG0_SPEC, 9> {
        EXEV1EG_W::new(self)
    }
    #[doc = "Bits 12:13 - External event 2 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev2src(&mut self) -> EXEV2SRC_W<EXEVCFG0_SPEC, 12> {
        EXEV2SRC_W::new(self)
    }
    #[doc = "Bit 14 - External event 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev2p(&mut self) -> EXEV2P_W<EXEVCFG0_SPEC, 14> {
        EXEV2P_W::new(self)
    }
    #[doc = "Bits 15:16 - External event 2 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev2eg(&mut self) -> EXEV2EG_W<EXEVCFG0_SPEC, 15> {
        EXEV2EG_W::new(self)
    }
    #[doc = "Bits 18:19 - External event 3 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev3src(&mut self) -> EXEV3SRC_W<EXEVCFG0_SPEC, 18> {
        EXEV3SRC_W::new(self)
    }
    #[doc = "Bit 20 - External event 3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev3p(&mut self) -> EXEV3P_W<EXEVCFG0_SPEC, 20> {
        EXEV3P_W::new(self)
    }
    #[doc = "Bits 21:22 - External event 3 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev3eg(&mut self) -> EXEV3EG_W<EXEVCFG0_SPEC, 21> {
        EXEV3EG_W::new(self)
    }
    #[doc = "Bits 24:25 - External event 4 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev4src(&mut self) -> EXEV4SRC_W<EXEVCFG0_SPEC, 24> {
        EXEV4SRC_W::new(self)
    }
    #[doc = "Bit 26 - External event 4 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev4p(&mut self) -> EXEV4P_W<EXEVCFG0_SPEC, 26> {
        EXEV4P_W::new(self)
    }
    #[doc = "Bits 27:28 - External event 4 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev4eg(&mut self) -> EXEV4EG_W<EXEVCFG0_SPEC, 27> {
        EXEV4EG_W::new(self)
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
#[doc = "SHRTIMER external event configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exevcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exevcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXEVCFG0_SPEC;
impl crate::RegisterSpec for EXEVCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exevcfg0::R`](R) reader structure"]
impl crate::Readable for EXEVCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exevcfg0::W`](W) writer structure"]
impl crate::Writable for EXEVCFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXEVCFG0 to value 0"]
impl crate::Resettable for EXEVCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
