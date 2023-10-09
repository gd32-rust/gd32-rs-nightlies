#[doc = "Register `ST1EXEVFCFG1` reader"]
pub type R = crate::R<ST1EXEVFCFG1_SPEC>;
#[doc = "Register `ST1EXEVFCFG1` writer"]
pub type W = crate::W<ST1EXEVFCFG1_SPEC>;
#[doc = "Field `EXEV5MEEN` reader - External event 5 memorized enable"]
pub type EXEV5MEEN_R = crate::BitReader;
#[doc = "Field `EXEV5MEEN` writer - External event 5 memorized enable"]
pub type EXEV5MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV5FM` reader - External event 5 filter mode"]
pub type EXEV5FM_R = crate::FieldReader;
#[doc = "Field `EXEV5FM` writer - External event 5 filter mode"]
pub type EXEV5FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV6MEEN` reader - External event 6 memorized enable"]
pub type EXEV6MEEN_R = crate::BitReader;
#[doc = "Field `EXEV6MEEN` writer - External event 6 memorized enable"]
pub type EXEV6MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV6FM` reader - External event 6 filter mode"]
pub type EXEV6FM_R = crate::FieldReader;
#[doc = "Field `EXEV6FM` writer - External event 6 filter mode"]
pub type EXEV6FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV7MEEN` reader - External event 7 memorized enable"]
pub type EXEV7MEEN_R = crate::BitReader;
#[doc = "Field `EXEV7MEEN` writer - External event 7 memorized enable"]
pub type EXEV7MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV7FM` reader - External event 7 filter mode"]
pub type EXEV7FM_R = crate::FieldReader;
#[doc = "Field `EXEV7FM` writer - External event 7 filter mode"]
pub type EXEV7FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV8MEEN` reader - External event 8 memorized enable"]
pub type EXEV8MEEN_R = crate::BitReader;
#[doc = "Field `EXEV8MEEN` writer - External event 8 memorized enable"]
pub type EXEV8MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV8FM` reader - External event 8 filter mode"]
pub type EXEV8FM_R = crate::FieldReader;
#[doc = "Field `EXEV8FM` writer - External event 8 filter mode"]
pub type EXEV8FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXEV9MEEN` reader - External event 9 memorized enable"]
pub type EXEV9MEEN_R = crate::BitReader;
#[doc = "Field `EXEV9MEEN` writer - External event 9 memorized enable"]
pub type EXEV9MEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV9FM` reader - External event 9 filter mode"]
pub type EXEV9FM_R = crate::FieldReader;
#[doc = "Field `EXEV9FM` writer - External event 9 filter mode"]
pub type EXEV9FM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - External event 5 memorized enable"]
    #[inline(always)]
    pub fn exev5meen(&self) -> EXEV5MEEN_R {
        EXEV5MEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External event 5 filter mode"]
    #[inline(always)]
    pub fn exev5fm(&self) -> EXEV5FM_R {
        EXEV5FM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External event 6 memorized enable"]
    #[inline(always)]
    pub fn exev6meen(&self) -> EXEV6MEEN_R {
        EXEV6MEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External event 6 filter mode"]
    #[inline(always)]
    pub fn exev6fm(&self) -> EXEV6FM_R {
        EXEV6FM_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External event 7 memorized enable"]
    #[inline(always)]
    pub fn exev7meen(&self) -> EXEV7MEEN_R {
        EXEV7MEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External event 7 filter mode"]
    #[inline(always)]
    pub fn exev7fm(&self) -> EXEV7FM_R {
        EXEV7FM_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External event 8 memorized enable"]
    #[inline(always)]
    pub fn exev8meen(&self) -> EXEV8MEEN_R {
        EXEV8MEEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External event 8 filter mode"]
    #[inline(always)]
    pub fn exev8fm(&self) -> EXEV8FM_R {
        EXEV8FM_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External event 9 memorized enable"]
    #[inline(always)]
    pub fn exev9meen(&self) -> EXEV9MEEN_R {
        EXEV9MEEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External event 9 filter mode"]
    #[inline(always)]
    pub fn exev9fm(&self) -> EXEV9FM_R {
        EXEV9FM_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External event 5 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev5meen(&mut self) -> EXEV5MEEN_W<ST1EXEVFCFG1_SPEC, 0> {
        EXEV5MEEN_W::new(self)
    }
    #[doc = "Bits 1:4 - External event 5 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev5fm(&mut self) -> EXEV5FM_W<ST1EXEVFCFG1_SPEC, 1> {
        EXEV5FM_W::new(self)
    }
    #[doc = "Bit 6 - External event 6 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev6meen(&mut self) -> EXEV6MEEN_W<ST1EXEVFCFG1_SPEC, 6> {
        EXEV6MEEN_W::new(self)
    }
    #[doc = "Bits 7:10 - External event 6 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev6fm(&mut self) -> EXEV6FM_W<ST1EXEVFCFG1_SPEC, 7> {
        EXEV6FM_W::new(self)
    }
    #[doc = "Bit 12 - External event 7 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev7meen(&mut self) -> EXEV7MEEN_W<ST1EXEVFCFG1_SPEC, 12> {
        EXEV7MEEN_W::new(self)
    }
    #[doc = "Bits 13:16 - External event 7 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev7fm(&mut self) -> EXEV7FM_W<ST1EXEVFCFG1_SPEC, 13> {
        EXEV7FM_W::new(self)
    }
    #[doc = "Bit 18 - External event 8 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev8meen(&mut self) -> EXEV8MEEN_W<ST1EXEVFCFG1_SPEC, 18> {
        EXEV8MEEN_W::new(self)
    }
    #[doc = "Bits 19:22 - External event 8 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev8fm(&mut self) -> EXEV8FM_W<ST1EXEVFCFG1_SPEC, 19> {
        EXEV8FM_W::new(self)
    }
    #[doc = "Bit 24 - External event 9 memorized enable"]
    #[inline(always)]
    #[must_use]
    pub fn exev9meen(&mut self) -> EXEV9MEEN_W<ST1EXEVFCFG1_SPEC, 24> {
        EXEV9MEEN_W::new(self)
    }
    #[doc = "Bits 25:28 - External event 9 filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn exev9fm(&mut self) -> EXEV9FM_W<ST1EXEVFCFG1_SPEC, 25> {
        EXEV9FM_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx external event filter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1exevfcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1exevfcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1EXEVFCFG1_SPEC;
impl crate::RegisterSpec for ST1EXEVFCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1exevfcfg1::R`](R) reader structure"]
impl crate::Readable for ST1EXEVFCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1exevfcfg1::W`](W) writer structure"]
impl crate::Writable for ST1EXEVFCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1EXEVFCFG1 to value 0"]
impl crate::Resettable for ST1EXEVFCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
