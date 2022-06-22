#[doc = "Register `SADDR0` reader"]
pub struct R(crate::R<SADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR0` writer"]
pub struct W(crate::W<SADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR0_SPEC>;
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
impl From<crate::W<SADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Address mode for the I2C slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDFORMAT_A {
    #[doc = "0: 7-bit slave address"]
    ADD7 = 0,
    #[doc = "1: 10-bit slave address"]
    ADD10 = 1,
}
impl From<ADDFORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: ADDFORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDFORMAT` reader - Address mode for the I2C slave"]
pub type ADDFORMAT_R = crate::BitReader<ADDFORMAT_A>;
impl ADDFORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDFORMAT_A {
        match self.bits {
            false => ADDFORMAT_A::ADD7,
            true => ADDFORMAT_A::ADD10,
        }
    }
    #[doc = "Checks if the value of the field is `ADD7`"]
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == ADDFORMAT_A::ADD7
    }
    #[doc = "Checks if the value of the field is `ADD10`"]
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == ADDFORMAT_A::ADD10
    }
}
#[doc = "Field `ADDFORMAT` writer - Address mode for the I2C slave"]
pub type ADDFORMAT_W<'a> = crate::BitWriter<'a, u32, SADDR0_SPEC, ADDFORMAT_A, 15>;
impl<'a> ADDFORMAT_W<'a> {
    #[doc = "7-bit slave address"]
    #[inline(always)]
    pub fn add7(self) -> &'a mut W {
        self.variant(ADDFORMAT_A::ADD7)
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn add10(self) -> &'a mut W {
        self.variant(ADDFORMAT_A::ADD10)
    }
}
#[doc = "Field `ADDRESS` reader - Highest two bits of a 10-bit address"]
pub type ADDRESS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRESS` writer - Highest two bits of a 10-bit address"]
pub type ADDRESS_W<'a> = crate::FieldWriterSafe<'a, u32, SADDR0_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    pub fn addformat(&self) -> ADDFORMAT_R {
        ADDFORMAT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 0:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    pub fn addformat(&mut self) -> ADDFORMAT_W {
        ADDFORMAT_W::new(self)
    }
    #[doc = "Bits 0:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr0](index.html) module"]
pub struct SADDR0_SPEC;
impl crate::RegisterSpec for SADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddr0::R](R) reader structure"]
impl crate::Readable for SADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr0::W](W) writer structure"]
impl crate::Writable for SADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDR0 to value 0"]
impl crate::Resettable for SADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
