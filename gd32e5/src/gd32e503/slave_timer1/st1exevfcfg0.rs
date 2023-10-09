#[doc = "Register `ST1EXEVFCFG0` reader"]
pub type R = crate::R<ST1EXEVFCFG0_SPEC>;
#[doc = "Register `ST1EXEVFCFG0` writer"]
pub type W = crate::W<ST1EXEVFCFG0_SPEC>;
#[doc = "Field `EXEV0MEEN` reader - External event 0 memorized enable"]
pub type EXEV0MEEN_R = crate::BitReader;
#[doc = "Field `EXEV0MEEN` writer - External event 0 memorized enable"]
pub type EXEV0MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV0FM` reader - External event 0 filter mode"]
pub type EXEV0FM_R = crate::FieldReader;
#[doc = "Field `EXEV0FM` writer - External event 0 filter mode"]
pub type EXEV0FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV1MEEN` reader - External event 1 memorized enable"]
pub type EXEV1MEEN_R = crate::BitReader;
#[doc = "Field `EXEV1MEEN` writer - External event 1 memorized enable"]
pub type EXEV1MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV1FM` reader - External event 1 filter mode"]
pub type EXEV1FM_R = crate::FieldReader;
#[doc = "Field `EXEV1FM` writer - External event 1 filter mode"]
pub type EXEV1FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV2MEEN` reader - External event 2 memorized enable"]
pub type EXEV2MEEN_R = crate::BitReader;
#[doc = "Field `EXEV2MEEN` writer - External event 2 memorized enable"]
pub type EXEV2MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV2FM` reader - External event 2 filter mode"]
pub type EXEV2FM_R = crate::FieldReader;
#[doc = "Field `EXEV2FM` writer - External event 2 filter mode"]
pub type EXEV2FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV3MEEN` reader - External event 3 memorized enable"]
pub type EXEV3MEEN_R = crate::BitReader;
#[doc = "Field `EXEV3MEEN` writer - External event 3 memorized enable"]
pub type EXEV3MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV3FM` reader - External event 3 filter mode"]
pub type EXEV3FM_R = crate::FieldReader;
#[doc = "Field `EXEV3FM` writer - External event 3 filter mode"]
pub type EXEV3FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV4MEEN` reader - External event 4 memorized enable"]
pub type EXEV4MEEN_R = crate::BitReader;
#[doc = "Field `EXEV4MEEN` writer - External event 4 memorized enable"]
pub type EXEV4MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV4FM` reader - External event 4 filter mode"]
pub type EXEV4FM_R = crate::FieldReader;
#[doc = "Field `EXEV4FM` writer - External event 4 filter mode"]
pub type EXEV4FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - External event 0 memorized enable"]
    #[inline(always)]
    pub fn exev0meen(&self) -> EXEV0MEEN_R {
        EXEV0MEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External event 0 filter mode"]
    #[inline(always)]
    pub fn exev0fm(&self) -> EXEV0FM_R {
        EXEV0FM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External event 1 memorized enable"]
    #[inline(always)]
    pub fn exev1meen(&self) -> EXEV1MEEN_R {
        EXEV1MEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External event 1 filter mode"]
    #[inline(always)]
    pub fn exev1fm(&self) -> EXEV1FM_R {
        EXEV1FM_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External event 2 memorized enable"]
    #[inline(always)]
    pub fn exev2meen(&self) -> EXEV2MEEN_R {
        EXEV2MEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External event 2 filter mode"]
    #[inline(always)]
    pub fn exev2fm(&self) -> EXEV2FM_R {
        EXEV2FM_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External event 3 memorized enable"]
    #[inline(always)]
    pub fn exev3meen(&self) -> EXEV3MEEN_R {
        EXEV3MEEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External event 3 filter mode"]
    #[inline(always)]
    pub fn exev3fm(&self) -> EXEV3FM_R {
        EXEV3FM_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External event 4 memorized enable"]
    #[inline(always)]
    pub fn exev4meen(&self) -> EXEV4MEEN_R {
        EXEV4MEEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External event 4 filter mode"]
    #[inline(always)]
    pub fn exev4fm(&self) -> EXEV4FM_R {
        EXEV4FM_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External event 0 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev0meen(&mut self) -> EXEV0MEEN_W<ST1EXEVFCFG0_SPEC, 0> {
        EXEV0MEEN_W::new(self)
    }
    #[doc = "Bits 1:4 - External event 0 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev0fm(&mut self) -> EXEV0FM_W<ST1EXEVFCFG0_SPEC, 1> {
        EXEV0FM_W::new(self)
    }
    #[doc = "Bit 6 - External event 1 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev1meen(&mut self) -> EXEV1MEEN_W<ST1EXEVFCFG0_SPEC, 6> {
        EXEV1MEEN_W::new(self)
    }
    #[doc = "Bits 7:10 - External event 1 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev1fm(&mut self) -> EXEV1FM_W<ST1EXEVFCFG0_SPEC, 7> {
        EXEV1FM_W::new(self)
    }
    #[doc = "Bit 12 - External event 2 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev2meen(&mut self) -> EXEV2MEEN_W<ST1EXEVFCFG0_SPEC, 12> {
        EXEV2MEEN_W::new(self)
    }
    #[doc = "Bits 13:16 - External event 2 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev2fm(&mut self) -> EXEV2FM_W<ST1EXEVFCFG0_SPEC, 13> {
        EXEV2FM_W::new(self)
    }
    #[doc = "Bit 18 - External event 3 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev3meen(&mut self) -> EXEV3MEEN_W<ST1EXEVFCFG0_SPEC, 18> {
        EXEV3MEEN_W::new(self)
    }
    #[doc = "Bits 19:22 - External event 3 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev3fm(&mut self) -> EXEV3FM_W<ST1EXEVFCFG0_SPEC, 19> {
        EXEV3FM_W::new(self)
    }
    #[doc = "Bit 24 - External event 4 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev4meen(&mut self) -> EXEV4MEEN_W<ST1EXEVFCFG0_SPEC, 24> {
        EXEV4MEEN_W::new(self)
    }
    #[doc = "Bits 25:28 - External event 4 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev4fm(&mut self) -> EXEV4FM_W<ST1EXEVFCFG0_SPEC, 25> {
        EXEV4FM_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1exevfcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1exevfcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1EXEVFCFG0_SPEC;
impl crate::RegisterSpec for ST1EXEVFCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1exevfcfg0::R`](R) reader structure"]
impl crate::Readable for ST1EXEVFCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1exevfcfg0::W`](W) writer structure"]
impl crate::Writable for ST1EXEVFCFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1EXEVFCFG0 to value 0"]
impl crate::Resettable for ST1EXEVFCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
