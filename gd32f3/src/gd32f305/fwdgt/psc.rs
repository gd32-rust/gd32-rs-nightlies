#[doc = "Register `PSC` reader"]
pub struct R(crate::R<PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSC` writer"]
pub struct W(crate::W<PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSC_SPEC>;
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
impl From<crate::W<PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Free watchdog timer prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Divider /4"]
    DIVIDEBY4 = 0,
    #[doc = "1: Divider /8"]
    DIVIDEBY8 = 1,
    #[doc = "2: Divider /16"]
    DIVIDEBY16 = 2,
    #[doc = "3: Divider /32"]
    DIVIDEBY32 = 3,
    #[doc = "4: Divider /64"]
    DIVIDEBY64 = 4,
    #[doc = "5: Divider /128"]
    DIVIDEBY128 = 5,
    #[doc = "6: Divider /256"]
    DIVIDEBY256 = 6,
    #[doc = "7: Divider /256"]
    DIVIDEBY256BIS = 7,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSC` reader - Free watchdog timer prescaler selection"]
pub type PSC_R = crate::FieldReader<u8, PSC_A>;
impl PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            0 => PSC_A::DIVIDEBY4,
            1 => PSC_A::DIVIDEBY8,
            2 => PSC_A::DIVIDEBY16,
            3 => PSC_A::DIVIDEBY32,
            4 => PSC_A::DIVIDEBY64,
            5 => PSC_A::DIVIDEBY128,
            6 => PSC_A::DIVIDEBY256,
            7 => PSC_A::DIVIDEBY256BIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY4`"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == PSC_A::DIVIDEBY4
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY8`"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == PSC_A::DIVIDEBY8
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY16`"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == PSC_A::DIVIDEBY16
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY32`"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == PSC_A::DIVIDEBY32
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY64`"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == PSC_A::DIVIDEBY64
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY128`"]
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == PSC_A::DIVIDEBY128
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY256`"]
    #[inline(always)]
    pub fn is_divide_by256(&self) -> bool {
        *self == PSC_A::DIVIDEBY256
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY256BIS`"]
    #[inline(always)]
    pub fn is_divide_by256bis(&self) -> bool {
        *self == PSC_A::DIVIDEBY256BIS
    }
}
#[doc = "Field `PSC` writer - Free watchdog timer prescaler selection"]
pub type PSC_W<'a> = crate::FieldWriterSafe<'a, u32, PSC_SPEC, u8, PSC_A, 3, 0>;
impl<'a> PSC_W<'a> {
    #[doc = "Divider /4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY4)
    }
    #[doc = "Divider /8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY8)
    }
    #[doc = "Divider /16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY16)
    }
    #[doc = "Divider /32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY32)
    }
    #[doc = "Divider /64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY64)
    }
    #[doc = "Divider /128"]
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY128)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY256)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256bis(self) -> &'a mut W {
        self.variant(PSC_A::DIVIDEBY256BIS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Free watchdog timer prescaler selection"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Free watchdog timer prescaler selection"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](index.html) module"]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psc::R](R) reader structure"]
impl crate::Readable for PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psc::W](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
