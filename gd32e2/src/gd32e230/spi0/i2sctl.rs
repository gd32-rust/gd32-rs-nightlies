#[doc = "Register `I2SCTL` reader"]
pub struct R(crate::R<I2SCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCTL` writer"]
pub struct W(crate::W<I2SCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCTL_SPEC>;
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
impl From<crate::W<I2SCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2SSEL` reader - I2S mode selection"]
pub type I2SSEL_R = crate::BitReader<bool>;
#[doc = "Field `I2SSEL` writer - I2S mode selection"]
pub type I2SSEL_W<'a> = crate::BitWriter<'a, u32, I2SCTL_SPEC, bool, 11>;
#[doc = "Field `I2SEN` reader - I2S Enable"]
pub type I2SEN_R = crate::BitReader<bool>;
#[doc = "Field `I2SEN` writer - I2S Enable"]
pub type I2SEN_W<'a> = crate::BitWriter<'a, u32, I2SCTL_SPEC, bool, 10>;
#[doc = "Field `I2SOPMOD` reader - I2S configuration mode"]
pub type I2SOPMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SOPMOD` writer - I2S configuration mode"]
pub type I2SOPMOD_W<'a> = crate::FieldWriter<'a, u32, I2SCTL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `PCMSMOD` reader - PCM frame synchronization"]
pub type PCMSMOD_R = crate::BitReader<bool>;
#[doc = "Field `PCMSMOD` writer - PCM frame synchronization"]
pub type PCMSMOD_W<'a> = crate::BitWriter<'a, u32, I2SCTL_SPEC, bool, 7>;
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a> = crate::FieldWriter<'a, u32, I2SCTL_SPEC, u8, u8, 2, 4>;
#[doc = "Field `CKPL` reader - Idle state clock polarity"]
pub type CKPL_R = crate::BitReader<bool>;
#[doc = "Field `CKPL` writer - Idle state clock polarity"]
pub type CKPL_W<'a> = crate::BitWriter<'a, u32, I2SCTL_SPEC, bool, 3>;
#[doc = "Field `DTLEN` reader - Data length to be transferred"]
pub type DTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTLEN` writer - Data length to be transferred"]
pub type DTLEN_W<'a> = crate::FieldWriter<'a, u32, I2SCTL_SPEC, u8, u8, 2, 1>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader<bool>;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a> = crate::BitWriter<'a, u32, I2SCTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2SSEL_R {
        I2SSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2sopmod(&self) -> I2SOPMOD_R {
        I2SOPMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsmod(&self) -> PCMSMOD_R {
        PCMSMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn dtlen(&self) -> DTLEN_R {
        DTLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2ssel(&mut self) -> I2SSEL_W {
        I2SSEL_W::new(self)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W::new(self)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2sopmod(&mut self) -> I2SOPMOD_W {
        I2SOPMOD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsmod(&mut self) -> PCMSMOD_W {
        PCMSMOD_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    pub fn ckpl(&mut self) -> CKPL_W {
        CKPL_W::new(self)
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn dtlen(&mut self) -> DTLEN_W {
        DTLEN_W::new(self)
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sctl](index.html) module"]
pub struct I2SCTL_SPEC;
impl crate::RegisterSpec for I2SCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sctl::R](R) reader structure"]
impl crate::Readable for I2SCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sctl::W](W) writer structure"]
impl crate::Writable for I2SCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SCTL to value 0"]
impl crate::Resettable for I2SCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
