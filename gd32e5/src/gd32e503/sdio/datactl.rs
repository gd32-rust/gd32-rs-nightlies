#[doc = "Register `DATACTL` reader"]
pub type R = crate::R<DATACTL_SPEC>;
#[doc = "Register `DATACTL` writer"]
pub type W = crate::W<DATACTL_SPEC>;
#[doc = "Field `DATAEN` reader - Data transfer enabled bit"]
pub type DATAEN_R = crate::BitReader;
#[doc = "Field `DATAEN` writer - Data transfer enabled bit"]
pub type DATAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATADIR` reader - Data transfer direction"]
pub type DATADIR_R = crate::BitReader;
#[doc = "Field `DATADIR` writer - Data transfer direction"]
pub type DATADIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANSMOD` reader - Data transfer mode"]
pub type TRANSMOD_R = crate::BitReader;
#[doc = "Field `TRANSMOD` writer - Data transfer mode"]
pub type TRANSMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKSZ` reader - Data block size"]
pub type BLKSZ_R = crate::FieldReader;
#[doc = "Field `BLKSZ` writer - Data block size"]
pub type BLKSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RWEN` reader - Read wait mode enabled"]
pub type RWEN_R = crate::BitReader;
#[doc = "Field `RWEN` writer - Read wait mode enabled"]
pub type RWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWSTOP` reader - Read wait stop"]
pub type RWSTOP_R = crate::BitReader;
#[doc = "Field `RWSTOP` writer - Read wait stop"]
pub type RWSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWTYPE` reader - Read wait type"]
pub type RWTYPE_R = crate::BitReader;
#[doc = "Field `RWTYPE` writer - Read wait type"]
pub type RWTYPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOEN` reader - SD I/O specific function enable"]
pub type IOEN_R = crate::BitReader;
#[doc = "Field `IOEN` writer - SD I/O specific function enable"]
pub type IOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn dataen(&mut self) -> DATAEN_W<DATACTL_SPEC, 0> {
        DATAEN_W::new(self)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn datadir(&mut self) -> DATADIR_W<DATACTL_SPEC, 1> {
        DATADIR_W::new(self)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn transmod(&mut self) -> TRANSMOD_W<DATACTL_SPEC, 2> {
        TRANSMOD_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DATACTL_SPEC, 3> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    #[must_use]
    pub fn blksz(&mut self) -> BLKSZ_W<DATACTL_SPEC, 4> {
        BLKSZ_W::new(self)
    }
    #[doc = "Bit 8 - Read wait mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rwen(&mut self) -> RWEN_W<DATACTL_SPEC, 8> {
        RWEN_W::new(self)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<DATACTL_SPEC, 9> {
        RWSTOP_W::new(self)
    }
    #[doc = "Bit 10 - Read wait type"]
    #[inline(always)]
    #[must_use]
    pub fn rwtype(&mut self) -> RWTYPE_W<DATACTL_SPEC, 10> {
        RWTYPE_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O specific function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ioen(&mut self) -> IOEN_W<DATACTL_SPEC, 11> {
        IOEN_W::new(self)
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
#[doc = "Data control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATACTL_SPEC;
impl crate::RegisterSpec for DATACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datactl::R`](R) reader structure"]
impl crate::Readable for DATACTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datactl::W`](W) writer structure"]
impl crate::Writable for DATACTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATACTL to value 0"]
impl crate::Resettable for DATACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
