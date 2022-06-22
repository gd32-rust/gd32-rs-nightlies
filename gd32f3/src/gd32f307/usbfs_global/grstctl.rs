#[doc = "Register `GRSTCTL` reader"]
pub struct R(crate::R<GRSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRSTCTL` writer"]
pub struct W(crate::W<GRSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRSTCTL_SPEC>;
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
impl From<crate::W<GRSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSRST` reader - Core soft reset"]
pub type CSRST_R = crate::BitReader<bool>;
#[doc = "Field `CSRST` writer - Core soft reset"]
pub type CSRST_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 0>;
#[doc = "Field `HCSRST` reader - HCLK soft reset"]
pub type HCSRST_R = crate::BitReader<bool>;
#[doc = "Field `HCSRST` writer - HCLK soft reset"]
pub type HCSRST_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 1>;
#[doc = "Field `HFCRST` reader - Host frame counter reset"]
pub type HFCRST_R = crate::BitReader<bool>;
#[doc = "Field `HFCRST` writer - Host frame counter reset"]
pub type HFCRST_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 2>;
#[doc = "Field `RXFF` reader - RxFIFO flush"]
pub type RXFF_R = crate::BitReader<bool>;
#[doc = "Field `RXFF` writer - RxFIFO flush"]
pub type RXFF_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 4>;
#[doc = "Field `TXFF` reader - TxFIFO flush"]
pub type TXFF_R = crate::BitReader<bool>;
#[doc = "Field `TXFF` writer - TxFIFO flush"]
pub type TXFF_W<'a> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, 5>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TXFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TXFNUM_W<'a> = crate::FieldWriter<'a, u32, GRSTCTL_SPEC, u8, u8, 5, 6>;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    pub fn hcsrst(&self) -> HCSRST_R {
        HCSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn hfcrst(&self) -> HFCRST_R {
        HFCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&mut self) -> CSRST_W {
        CSRST_W::new(self)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    pub fn hcsrst(&mut self) -> HCSRST_W {
        HCSRST_W::new(self)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn hfcrst(&mut self) -> HFCRST_W {
        HFCRST_W::new(self)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxff(&mut self) -> RXFF_W {
        RXFF_W::new(self)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txff(&mut self) -> TXFF_W {
        TXFF_W::new(self)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global reset control register (USBFS_GRSTCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstctl](index.html) module"]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grstctl::R](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grstctl::W](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
