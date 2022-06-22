#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The output value selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSEL_A {
    #[doc = "0: Normal behaviour"]
    NORMAL = 0,
    #[doc = "1: If POEN and IOS is 0 the output is disabled"]
    DISABLED = 1,
}
impl From<OUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSEL` reader - The output value selection"]
pub type OUTSEL_R = crate::BitReader<OUTSEL_A>;
impl OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSEL_A {
        match self.bits {
            false => OUTSEL_A::NORMAL,
            true => OUTSEL_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OUTSEL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUTSEL_A::DISABLED
    }
}
#[doc = "Field `OUTSEL` writer - The output value selection"]
pub type OUTSEL_W<'a> = crate::BitWriter<'a, u16, CFG_SPEC, OUTSEL_A, 0>;
impl<'a> OUTSEL_W<'a> {
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OUTSEL_A::NORMAL)
    }
    #[doc = "If POEN and IOS is 0 the output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OUTSEL_A::DISABLED)
    }
}
#[doc = "Write Capture/Compare register selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHVSEL_A {
    #[doc = "0: Normal behaviour"]
    NORMAL = 0,
    #[doc = "1: Duplicate writes to CHxVAL are ignored"]
    IGNORESAME = 1,
}
impl From<CHVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CHVSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHVSEL` reader - Write Capture/Compare register selection"]
pub type CHVSEL_R = crate::BitReader<CHVSEL_A>;
impl CHVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHVSEL_A {
        match self.bits {
            false => CHVSEL_A::NORMAL,
            true => CHVSEL_A::IGNORESAME,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHVSEL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `IGNORESAME`"]
    #[inline(always)]
    pub fn is_ignore_same(&self) -> bool {
        *self == CHVSEL_A::IGNORESAME
    }
}
#[doc = "Field `CHVSEL` writer - Write Capture/Compare register selection"]
pub type CHVSEL_W<'a> = crate::BitWriter<'a, u16, CFG_SPEC, CHVSEL_A, 1>;
impl<'a> CHVSEL_W<'a> {
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHVSEL_A::NORMAL)
    }
    #[doc = "Duplicate writes to CHxVAL are ignored"]
    #[inline(always)]
    pub fn ignore_same(self) -> &'a mut W {
        self.variant(CHVSEL_A::IGNORESAME)
    }
}
impl R {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Capture/Compare register selection"]
    #[inline(always)]
    pub fn chvsel(&self) -> CHVSEL_R {
        CHVSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W::new(self)
    }
    #[doc = "Bit 1 - Write Capture/Compare register selection"]
    #[inline(always)]
    pub fn chvsel(&mut self) -> CHVSEL_W {
        CHVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
