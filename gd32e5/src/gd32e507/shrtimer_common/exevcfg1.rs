#[doc = "Register `EXEVCFG1` reader"]
pub type R = crate::R<EXEVCFG1_SPEC>;
#[doc = "Register `EXEVCFG1` writer"]
pub type W = crate::W<EXEVCFG1_SPEC>;
#[doc = "Field `EXEV5SRC` reader - External event 0 source"]
pub type EXEV5SRC_R = crate::FieldReader;
#[doc = "Field `EXEV5SRC` writer - External event 0 source"]
pub type EXEV5SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV5P` reader - External event 0 polarity"]
pub type EXEV5P_R = crate::BitReader;
#[doc = "Field `EXEV5P` writer - External event 0 polarity"]
pub type EXEV5P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV5EG` reader - External event 0 edge sensitivity"]
pub type EXEV5EG_R = crate::FieldReader;
#[doc = "Field `EXEV5EG` writer - External event 0 edge sensitivity"]
pub type EXEV5EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV6SRC` reader - External event 1 source"]
pub type EXEV6SRC_R = crate::FieldReader;
#[doc = "Field `EXEV6SRC` writer - External event 1 source"]
pub type EXEV6SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV6P` reader - External event 1 polarity"]
pub type EXEV6P_R = crate::BitReader;
#[doc = "Field `EXEV6P` writer - External event 1 polarity"]
pub type EXEV6P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV6EG` reader - External event 6 edge sensitivity"]
pub type EXEV6EG_R = crate::FieldReader;
#[doc = "Field `EXEV6EG` writer - External event 6 edge sensitivity"]
pub type EXEV6EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV7SRC` reader - External event 7 source"]
pub type EXEV7SRC_R = crate::FieldReader;
#[doc = "Field `EXEV7SRC` writer - External event 7 source"]
pub type EXEV7SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV7P` reader - External event 7polarity"]
pub type EXEV7P_R = crate::BitReader;
#[doc = "Field `EXEV7P` writer - External event 7polarity"]
pub type EXEV7P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV7EG` reader - External event 7 edge sensitivity"]
pub type EXEV7EG_R = crate::FieldReader;
#[doc = "Field `EXEV7EG` writer - External event 7 edge sensitivity"]
pub type EXEV7EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV8SRC` reader - External event 8 source"]
pub type EXEV8SRC_R = crate::FieldReader;
#[doc = "Field `EXEV8SRC` writer - External event 8 source"]
pub type EXEV8SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV8P` reader - External event 8 polarity"]
pub type EXEV8P_R = crate::BitReader;
#[doc = "Field `EXEV8P` writer - External event 8 polarity"]
pub type EXEV8P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV8EG` reader - External event 8 edge sensitivity"]
pub type EXEV8EG_R = crate::FieldReader;
#[doc = "Field `EXEV8EG` writer - External event 8 edge sensitivity"]
pub type EXEV8EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV9SRC` reader - External event 9 source"]
pub type EXEV9SRC_R = crate::FieldReader;
#[doc = "Field `EXEV9SRC` writer - External event 9 source"]
pub type EXEV9SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXEV9P` reader - External event 9 polarity"]
pub type EXEV9P_R = crate::BitReader;
#[doc = "Field `EXEV9P` writer - External event 9 polarity"]
pub type EXEV9P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV9EG` reader - External event 9 edge sensitivity"]
pub type EXEV9EG_R = crate::FieldReader;
#[doc = "Field `EXEV9EG` writer - External event 9 edge sensitivity"]
pub type EXEV9EG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - External event 0 source"]
    #[inline(always)]
    pub fn exev5src(&self) -> EXEV5SRC_R {
        EXEV5SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External event 0 polarity"]
    #[inline(always)]
    pub fn exev5p(&self) -> EXEV5P_R {
        EXEV5P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - External event 0 edge sensitivity"]
    #[inline(always)]
    pub fn exev5eg(&self) -> EXEV5EG_R {
        EXEV5EG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External event 1 source"]
    #[inline(always)]
    pub fn exev6src(&self) -> EXEV6SRC_R {
        EXEV6SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - External event 1 polarity"]
    #[inline(always)]
    pub fn exev6p(&self) -> EXEV6P_R {
        EXEV6P_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - External event 6 edge sensitivity"]
    #[inline(always)]
    pub fn exev6eg(&self) -> EXEV6EG_R {
        EXEV6EG_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External event 7 source"]
    #[inline(always)]
    pub fn exev7src(&self) -> EXEV7SRC_R {
        EXEV7SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External event 7polarity"]
    #[inline(always)]
    pub fn exev7p(&self) -> EXEV7P_R {
        EXEV7P_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - External event 7 edge sensitivity"]
    #[inline(always)]
    pub fn exev7eg(&self) -> EXEV7EG_R {
        EXEV7EG_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 18:19 - External event 8 source"]
    #[inline(always)]
    pub fn exev8src(&self) -> EXEV8SRC_R {
        EXEV8SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - External event 8 polarity"]
    #[inline(always)]
    pub fn exev8p(&self) -> EXEV8P_R {
        EXEV8P_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - External event 8 edge sensitivity"]
    #[inline(always)]
    pub fn exev8eg(&self) -> EXEV8EG_R {
        EXEV8EG_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External event 9 source"]
    #[inline(always)]
    pub fn exev9src(&self) -> EXEV9SRC_R {
        EXEV9SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External event 9 polarity"]
    #[inline(always)]
    pub fn exev9p(&self) -> EXEV9P_R {
        EXEV9P_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External event 9 edge sensitivity"]
    #[inline(always)]
    pub fn exev9eg(&self) -> EXEV9EG_R {
        EXEV9EG_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External event 0 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev5src(&mut self) -> EXEV5SRC_W<EXEVCFG1_SPEC, 0> {
        EXEV5SRC_W::new(self)
    }
    #[doc = "Bit 2 - External event 0 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev5p(&mut self) -> EXEV5P_W<EXEVCFG1_SPEC, 2> {
        EXEV5P_W::new(self)
    }
    #[doc = "Bits 3:4 - External event 0 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev5eg(&mut self) -> EXEV5EG_W<EXEVCFG1_SPEC, 3> {
        EXEV5EG_W::new(self)
    }
    #[doc = "Bits 6:7 - External event 1 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev6src(&mut self) -> EXEV6SRC_W<EXEVCFG1_SPEC, 6> {
        EXEV6SRC_W::new(self)
    }
    #[doc = "Bit 8 - External event 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev6p(&mut self) -> EXEV6P_W<EXEVCFG1_SPEC, 8> {
        EXEV6P_W::new(self)
    }
    #[doc = "Bits 9:10 - External event 6 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev6eg(&mut self) -> EXEV6EG_W<EXEVCFG1_SPEC, 9> {
        EXEV6EG_W::new(self)
    }
    #[doc = "Bits 12:13 - External event 7 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev7src(&mut self) -> EXEV7SRC_W<EXEVCFG1_SPEC, 12> {
        EXEV7SRC_W::new(self)
    }
    #[doc = "Bit 14 - External event 7polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev7p(&mut self) -> EXEV7P_W<EXEVCFG1_SPEC, 14> {
        EXEV7P_W::new(self)
    }
    #[doc = "Bits 15:16 - External event 7 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev7eg(&mut self) -> EXEV7EG_W<EXEVCFG1_SPEC, 15> {
        EXEV7EG_W::new(self)
    }
    #[doc = "Bits 18:19 - External event 8 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev8src(&mut self) -> EXEV8SRC_W<EXEVCFG1_SPEC, 18> {
        EXEV8SRC_W::new(self)
    }
    #[doc = "Bit 20 - External event 8 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev8p(&mut self) -> EXEV8P_W<EXEVCFG1_SPEC, 20> {
        EXEV8P_W::new(self)
    }
    #[doc = "Bits 21:22 - External event 8 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev8eg(&mut self) -> EXEV8EG_W<EXEVCFG1_SPEC, 21> {
        EXEV8EG_W::new(self)
    }
    #[doc = "Bits 24:25 - External event 9 source"]
    #[inline(always)]
    #[must_use]
    pub fn exev9src(&mut self) -> EXEV9SRC_W<EXEVCFG1_SPEC, 24> {
        EXEV9SRC_W::new(self)
    }
    #[doc = "Bit 26 - External event 9 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exev9p(&mut self) -> EXEV9P_W<EXEVCFG1_SPEC, 26> {
        EXEV9P_W::new(self)
    }
    #[doc = "Bits 27:28 - External event 9 edge sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn exev9eg(&mut self) -> EXEV9EG_W<EXEVCFG1_SPEC, 27> {
        EXEV9EG_W::new(self)
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
#[doc = "SHRTIMER external event configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exevcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exevcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXEVCFG1_SPEC;
impl crate::RegisterSpec for EXEVCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exevcfg1::R`](R) reader structure"]
impl crate::Readable for EXEVCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exevcfg1::W`](W) writer structure"]
impl crate::Writable for EXEVCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXEVCFG1 to value 0"]
impl crate::Resettable for EXEVCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
