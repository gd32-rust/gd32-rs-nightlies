#[doc = "Register `I2SPSC` reader"]
pub struct R(crate::R<I2SPSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SPSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SPSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SPSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SPSC` writer"]
pub struct W(crate::W<I2SPSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SPSC_SPEC>;
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
impl From<crate::W<I2SPSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SPSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCKOEN` reader - I2S_MCK output enable"]
pub type MCKOEN_R = crate::BitReader<bool>;
#[doc = "Field `MCKOEN` writer - I2S_MCK output enable"]
pub type MCKOEN_W<'a> = crate::BitWriter<'a, u32, I2SPSC_SPEC, bool, 9>;
#[doc = "Field `OF` reader - Odd factor for the prescaler"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `OF` writer - Odd factor for the prescaler"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, I2SPSC_SPEC, bool, 8>;
#[doc = "Field `DIV` reader - Dividing factor for the prescaler"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Dividing factor for the prescaler"]
pub type DIV_W<'a> = crate::FieldWriter<'a, u32, I2SPSC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    pub fn mckoen(&self) -> MCKOEN_R {
        MCKOEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    pub fn mckoen(&mut self) -> MCKOEN_W {
        MCKOEN_W::new(self)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2spsc](index.html) module"]
pub struct I2SPSC_SPEC;
impl crate::RegisterSpec for I2SPSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2spsc::R](R) reader structure"]
impl crate::Readable for I2SPSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2spsc::W](W) writer structure"]
impl crate::Writable for I2SPSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SPSC to value 0x02"]
impl crate::Resettable for I2SPSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
