#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAUEN` reader - Cryptographic module enable"]
pub type CAUEN_R = crate::BitReader<bool>;
#[doc = "Field `CAUEN` writer - Cryptographic module enable"]
pub type CAUEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 15>;
#[doc = "Field `FFLUSH` writer - FIFO flush"]
pub type FFLUSH_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 14>;
#[doc = "Field `KEYM` reader - AES key size mode configuration"]
pub type KEYM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEYM` writer - AES key size mode configuration"]
pub type KEYM_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `DATAM` reader - Data swapping type mode configuration"]
pub type DATAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAM` writer - Data swapping type mode configuration"]
pub type DATAM_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, 6>;
#[doc = "Field `ALGM` reader - Encryption/decryption algorithm mode"]
pub type ALGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALGM` writer - Encryption/decryption algorithm mode"]
pub type ALGM_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, 3>;
#[doc = "Field `CAUDIR` reader - CAU direction"]
pub type CAUDIR_R = crate::BitReader<bool>;
#[doc = "Field `CAUDIR` writer - CAU direction"]
pub type CAUDIR_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 15 - Cryptographic module enable"]
    #[inline(always)]
    pub fn cauen(&self) -> CAUEN_R {
        CAUEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:9 - AES key size mode configuration"]
    #[inline(always)]
    pub fn keym(&self) -> KEYM_R {
        KEYM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Data swapping type mode configuration"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Encryption/decryption algorithm mode"]
    #[inline(always)]
    pub fn algm(&self) -> ALGM_R {
        ALGM_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 2 - CAU direction"]
    #[inline(always)]
    pub fn caudir(&self) -> CAUDIR_R {
        CAUDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Cryptographic module enable"]
    #[inline(always)]
    pub fn cauen(&mut self) -> CAUEN_W {
        CAUEN_W::new(self)
    }
    #[doc = "Bit 14 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W {
        FFLUSH_W::new(self)
    }
    #[doc = "Bits 8:9 - AES key size mode configuration"]
    #[inline(always)]
    pub fn keym(&mut self) -> KEYM_W {
        KEYM_W::new(self)
    }
    #[doc = "Bits 6:7 - Data swapping type mode configuration"]
    #[inline(always)]
    pub fn datam(&mut self) -> DATAM_W {
        DATAM_W::new(self)
    }
    #[doc = "Bits 3:5 - Encryption/decryption algorithm mode"]
    #[inline(always)]
    pub fn algm(&mut self) -> ALGM_W {
        ALGM_W::new(self)
    }
    #[doc = "Bit 2 - CAU direction"]
    #[inline(always)]
    pub fn caudir(&mut self) -> CAUDIR_W {
        CAUDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAU control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
