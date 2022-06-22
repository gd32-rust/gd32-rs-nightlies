#[doc = "Register `SADDR1` reader"]
pub struct R(crate::R<SADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR1` writer"]
pub struct W(crate::W<SADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR1_SPEC>;
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
impl From<crate::W<SADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS2` reader - Interface address"]
pub type ADDRESS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS2` writer - Interface address"]
pub type ADDRESS2_W<'a> = crate::FieldWriterSafe<'a, u32, SADDR1_SPEC, u8, u8, 7, 1>;
#[doc = "Dual addressing mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUADEN_A {
    #[doc = "0: Single addressing mode"]
    SINGLE = 0,
    #[doc = "1: Dual addressing mode"]
    DUAL = 1,
}
impl From<DUADEN_A> for bool {
    #[inline(always)]
    fn from(variant: DUADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUADEN` reader - Dual addressing mode enable"]
pub type DUADEN_R = crate::BitReader<DUADEN_A>;
impl DUADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUADEN_A {
        match self.bits {
            false => DUADEN_A::SINGLE,
            true => DUADEN_A::DUAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DUADEN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DUADEN_A::DUAL
    }
}
#[doc = "Field `DUADEN` writer - Dual addressing mode enable"]
pub type DUADEN_W<'a> = crate::BitWriter<'a, u32, SADDR1_SPEC, DUADEN_A, 0>;
impl<'a> DUADEN_W<'a> {
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DUADEN_A::SINGLE)
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DUADEN_A::DUAL)
    }
}
impl R {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn address2(&self) -> ADDRESS2_R {
        ADDRESS2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn duaden(&self) -> DUADEN_R {
        DUADEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn address2(&mut self) -> ADDRESS2_W {
        ADDRESS2_W::new(self)
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn duaden(&mut self) -> DUADEN_W {
        DUADEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr1](index.html) module"]
pub struct SADDR1_SPEC;
impl crate::RegisterSpec for SADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddr1::R](R) reader structure"]
impl crate::Readable for SADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr1::W](W) writer structure"]
impl crate::Writable for SADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDR1 to value 0"]
impl crate::Resettable for SADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
