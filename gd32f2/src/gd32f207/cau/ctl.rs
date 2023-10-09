#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `CAUDIR` reader - CAU direction"]
pub type CAUDIR_R = crate::BitReader;
#[doc = "Field `CAUDIR` writer - CAU direction"]
pub type CAUDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALGM` reader - Encryption/decryption algorithm mode"]
pub type ALGM_R = crate::FieldReader;
#[doc = "Field `ALGM` writer - Encryption/decryption algorithm mode"]
pub type ALGM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DATAM` reader - Data swapping type mode configuration"]
pub type DATAM_R = crate::FieldReader;
#[doc = "Field `DATAM` writer - Data swapping type mode configuration"]
pub type DATAM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `KEYM` reader - AES key size mode configuration"]
pub type KEYM_R = crate::FieldReader;
#[doc = "Field `KEYM` writer - AES key size mode configuration"]
pub type KEYM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FFLUSH` writer - FIFO flush"]
pub type FFLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAUEN` reader - Cryptographic module enable"]
pub type CAUEN_R = crate::BitReader;
#[doc = "Field `CAUEN` writer - Cryptographic module enable"]
pub type CAUEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - CAU direction"]
    #[inline(always)]
    pub fn caudir(&self) -> CAUDIR_R {
        CAUDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Encryption/decryption algorithm mode"]
    #[inline(always)]
    pub fn algm(&self) -> ALGM_R {
        ALGM_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Data swapping type mode configuration"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - AES key size mode configuration"]
    #[inline(always)]
    pub fn keym(&self) -> KEYM_R {
        KEYM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Cryptographic module enable"]
    #[inline(always)]
    pub fn cauen(&self) -> CAUEN_R {
        CAUEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - CAU direction"]
    #[inline(always)]
    #[must_use]
    pub fn caudir(&mut self) -> CAUDIR_W<CTL_SPEC, 2> {
        CAUDIR_W::new(self)
    }
    #[doc = "Bits 3:5 - Encryption/decryption algorithm mode"]
    #[inline(always)]
    #[must_use]
    pub fn algm(&mut self) -> ALGM_W<CTL_SPEC, 3> {
        ALGM_W::new(self)
    }
    #[doc = "Bits 6:7 - Data swapping type mode configuration"]
    #[inline(always)]
    #[must_use]
    pub fn datam(&mut self) -> DATAM_W<CTL_SPEC, 6> {
        DATAM_W::new(self)
    }
    #[doc = "Bits 8:9 - AES key size mode configuration"]
    #[inline(always)]
    #[must_use]
    pub fn keym(&mut self) -> KEYM_W<CTL_SPEC, 8> {
        KEYM_W::new(self)
    }
    #[doc = "Bit 14 - FIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<CTL_SPEC, 14> {
        FFLUSH_W::new(self)
    }
    #[doc = "Bit 15 - Cryptographic module enable"]
    #[inline(always)]
    #[must_use]
    pub fn cauen(&mut self) -> CAUEN_W<CTL_SPEC, 15> {
        CAUEN_W::new(self)
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
#[doc = "CAU control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
