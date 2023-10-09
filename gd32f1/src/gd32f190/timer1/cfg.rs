#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `CHVSEL` reader - Configuration register"]
pub type CHVSEL_R = crate::BitReader<CHVSEL_A>;
#[doc = "Configuration register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHVSEL_A {
    #[doc = "0: Normal behaviour"]
    NORMAL = 0,
    #[doc = "1: Duplicate writes to CHxVAL are ignored"]
    IGNORE_SAME = 1,
}
impl From<CHVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CHVSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CHVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHVSEL_A {
        match self.bits {
            false => CHVSEL_A::NORMAL,
            true => CHVSEL_A::IGNORE_SAME,
        }
    }
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHVSEL_A::NORMAL
    }
    #[doc = "Duplicate writes to CHxVAL are ignored"]
    #[inline(always)]
    pub fn is_ignore_same(&self) -> bool {
        *self == CHVSEL_A::IGNORE_SAME
    }
}
#[doc = "Field `CHVSEL` writer - Configuration register"]
pub type CHVSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHVSEL_A>;
impl<'a, REG, const O: u8> CHVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CHVSEL_A::NORMAL)
    }
    #[doc = "Duplicate writes to CHxVAL are ignored"]
    #[inline(always)]
    pub fn ignore_same(self) -> &'a mut crate::W<REG> {
        self.variant(CHVSEL_A::IGNORE_SAME)
    }
}
impl R {
    #[doc = "Bit 1 - Configuration register"]
    #[inline(always)]
    pub fn chvsel(&self) -> CHVSEL_R {
        CHVSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Configuration register"]
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u16;
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
