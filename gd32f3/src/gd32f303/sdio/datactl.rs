#[doc = "Register `DATACTL` reader"]
pub struct R(crate::R<DATACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATACTL` writer"]
pub struct W(crate::W<DATACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATACTL_SPEC>;
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
impl From<crate::W<DATACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAEN` reader - Data transfer enabled bit"]
pub type DATAEN_R = crate::BitReader<bool>;
#[doc = "Field `DATAEN` writer - Data transfer enabled bit"]
pub type DATAEN_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 0>;
#[doc = "Field `DATADIR` reader - Data transfer direction"]
pub type DATADIR_R = crate::BitReader<bool>;
#[doc = "Field `DATADIR` writer - Data transfer direction"]
pub type DATADIR_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 1>;
#[doc = "Field `TRANSMOD` reader - Data transfer mode"]
pub type TRANSMOD_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMOD` writer - Data transfer mode"]
pub type TRANSMOD_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 2>;
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 3>;
#[doc = "Field `BLKSZ` reader - Data block size"]
pub type BLKSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLKSZ` writer - Data block size"]
pub type BLKSZ_W<'a> = crate::FieldWriter<'a, u32, DATACTL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `RWEN` reader - Read wait mode enabled"]
pub type RWEN_R = crate::BitReader<bool>;
#[doc = "Field `RWEN` writer - Read wait mode enabled"]
pub type RWEN_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 8>;
#[doc = "Field `RWSTOP` reader - Read wait stop"]
pub type RWSTOP_R = crate::BitReader<bool>;
#[doc = "Field `RWSTOP` writer - Read wait stop"]
pub type RWSTOP_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 9>;
#[doc = "Field `RWTYPE` reader - Read wait type"]
pub type RWTYPE_R = crate::BitReader<bool>;
#[doc = "Field `RWTYPE` writer - Read wait type"]
pub type RWTYPE_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 10>;
#[doc = "Field `IOEN` reader - SD I/O specific function enable"]
pub type IOEN_R = crate::BitReader<bool>;
#[doc = "Field `IOEN` writer - SD I/O specific function enable"]
pub type IOEN_W<'a> = crate::BitWriter<'a, u32, DATACTL_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Data transfer enabled bit"]
    #[inline(always)]
    pub fn dataen(&self) -> DATAEN_R {
        DATAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn datadir(&self) -> DATADIR_R {
        DATADIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn transmod(&self) -> TRANSMOD_R {
        TRANSMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn blksz(&self) -> BLKSZ_R {
        BLKSZ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait mode enabled"]
    #[inline(always)]
    pub fn rwen(&self) -> RWEN_R {
        RWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read wait type"]
    #[inline(always)]
    pub fn rwtype(&self) -> RWTYPE_R {
        RWTYPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O specific function enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enabled bit"]
    #[inline(always)]
    pub fn dataen(&mut self) -> DATAEN_W {
        DATAEN_W::new(self)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn datadir(&mut self) -> DATADIR_W {
        DATADIR_W::new(self)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn transmod(&mut self) -> TRANSMOD_W {
        TRANSMOD_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn blksz(&mut self) -> BLKSZ_W {
        BLKSZ_W::new(self)
    }
    #[doc = "Bit 8 - Read wait mode enabled"]
    #[inline(always)]
    pub fn rwen(&mut self) -> RWEN_W {
        RWEN_W::new(self)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W {
        RWSTOP_W::new(self)
    }
    #[doc = "Bit 10 - Read wait type"]
    #[inline(always)]
    pub fn rwtype(&mut self) -> RWTYPE_W {
        RWTYPE_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O specific function enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IOEN_W {
        IOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datactl](index.html) module"]
pub struct DATACTL_SPEC;
impl crate::RegisterSpec for DATACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datactl::R](R) reader structure"]
impl crate::Readable for DATACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datactl::W](W) writer structure"]
impl crate::Writable for DATACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATACTL to value 0"]
impl crate::Resettable for DATACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
