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
#[doc = "Early wakeup interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIE_A {
    #[doc = "1: interrupt occurs whenever the counter reaches the value 0x40"]
    ENABLE = 1,
}
impl From<EWIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIE` reader - Early wakeup interrupt enable"]
pub type EWIE_R = crate::BitReader<EWIE_A>;
impl EWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EWIE_A> {
        match self.bits {
            true => Some(EWIE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWIE_A::ENABLE
    }
}
#[doc = "Field `EWIE` writer - Early wakeup interrupt enable"]
pub type EWIE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, EWIE_A, 9>;
impl<'a> EWIE_W<'a> {
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EWIE_A::ENABLE)
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Counter clock (PCLK1 div 4096) div 1"]
    DIV1 = 0,
    #[doc = "1: Counter clock (PCLK1 div 4096) div 2"]
    DIV2 = 1,
    #[doc = "2: Counter clock (PCLK1 div 4096) div 4"]
    DIV4 = 2,
    #[doc = "3: Counter clock (PCLK1 div 4096) div 8"]
    DIV8 = 3,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSC` reader - Prescaler"]
pub type PSC_R = crate::FieldReader<u8, PSC_A>;
impl PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            0 => PSC_A::DIV1,
            1 => PSC_A::DIV2,
            2 => PSC_A::DIV4,
            3 => PSC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PSC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSC_A::DIV8
    }
}
#[doc = "Field `PSC` writer - Prescaler"]
pub type PSC_W<'a> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, PSC_A, 2, 7>;
impl<'a> PSC_W<'a> {
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PSC_A::DIV1)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSC_A::DIV2)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSC_A::DIV4)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSC_A::DIV8)
    }
}
#[doc = "Field `WIN` reader - The Window value"]
pub type WIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIN` writer - The Window value"]
pub type WIN_W<'a> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bit 9 - Early wakeup interrupt enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 0:6 - The Window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - Early wakeup interrupt enable"]
    #[inline(always)]
    pub fn ewie(&mut self) -> EWIE_W {
        EWIE_W::new(self)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Bits 0:6 - The Window value"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
