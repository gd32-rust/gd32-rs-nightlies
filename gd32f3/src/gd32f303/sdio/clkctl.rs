#[doc = "Register `CLKCTL` reader"]
pub struct R(crate::R<CLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCTL` writer"]
pub struct W(crate::W<CLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCTL_SPEC>;
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
impl From<crate::W<CLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_8` reader - MSB of Clock division"]
pub type DIV_8_R = crate::BitReader<bool>;
#[doc = "Field `DIV_8` writer - MSB of Clock division"]
pub type DIV_8_W<'a> = crate::BitWriter<'a, u32, CLKCTL_SPEC, bool, 31>;
#[doc = "Field `HWCLKEN` reader - Hardware Clock Control enable bit"]
pub type HWCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `HWCLKEN` writer - Hardware Clock Control enable bit"]
pub type HWCLKEN_W<'a> = crate::BitWriter<'a, u32, CLKCTL_SPEC, bool, 14>;
#[doc = "Field `CLKEDGE` reader - SDIO_CLK clock edge selection bit"]
pub type CLKEDGE_R = crate::BitReader<bool>;
#[doc = "Field `CLKEDGE` writer - SDIO_CLK clock edge selection bit"]
pub type CLKEDGE_W<'a> = crate::BitWriter<'a, u32, CLKCTL_SPEC, bool, 13>;
#[doc = "Field `BUSMODE` reader - SDIO card bus mode control bit"]
pub type BUSMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSMODE` writer - SDIO card bus mode control bit"]
pub type BUSMODE_W<'a> = crate::FieldWriter<'a, u32, CLKCTL_SPEC, u8, u8, 2, 11>;
#[doc = "Field `CLKBYP` reader - Clock bypass enable bit"]
pub type CLKBYP_R = crate::BitReader<bool>;
#[doc = "Field `CLKBYP` writer - Clock bypass enable bit"]
pub type CLKBYP_W<'a> = crate::BitWriter<'a, u32, CLKCTL_SPEC, bool, 10>;
#[doc = "Field `CLKPWRSAV` reader - SDIO_CLK clock dynamic switch on/off for power saving"]
pub type CLKPWRSAV_R = crate::BitReader<bool>;
#[doc = "Field `CLKPWRSAV` writer - SDIO_CLK clock dynamic switch on/off for power saving"]
pub type CLKPWRSAV_W<'a> = crate::BitWriter<'a, u32, CLKCTL_SPEC, bool, 9>;
#[doc = "Field `CLKEN` reader - SDIO_CLK clock output enable bit"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - SDIO_CLK clock output enable bit"]
pub type CLKEN_W<'a> = crate::BitWriter<'a, u32, CLKCTL_SPEC, bool, 8>;
#[doc = "Field `DIV_0_7` reader - Clock division"]
pub type DIV_0_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV_0_7` writer - Clock division"]
pub type DIV_0_7_W<'a> = crate::FieldWriter<'a, u32, CLKCTL_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    pub fn div_8(&self) -> DIV_8_R {
        DIV_8_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    pub fn hwclken(&self) -> HWCLKEN_R {
        HWCLKEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    pub fn clkedge(&self) -> CLKEDGE_R {
        CLKEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    pub fn busmode(&self) -> BUSMODE_R {
        BUSMODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    pub fn clkbyp(&self) -> CLKBYP_R {
        CLKBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    pub fn clkpwrsav(&self) -> CLKPWRSAV_R {
        CLKPWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn div_0_7(&self) -> DIV_0_7_R {
        DIV_0_7_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    pub fn div_8(&mut self) -> DIV_8_W {
        DIV_8_W::new(self)
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    pub fn hwclken(&mut self) -> HWCLKEN_W {
        HWCLKEN_W::new(self)
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    pub fn clkedge(&mut self) -> CLKEDGE_W {
        CLKEDGE_W::new(self)
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    pub fn busmode(&mut self) -> BUSMODE_W {
        BUSMODE_W::new(self)
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    pub fn clkbyp(&mut self) -> CLKBYP_W {
        CLKBYP_W::new(self)
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    pub fn clkpwrsav(&mut self) -> CLKPWRSAV_W {
        CLKPWRSAV_W::new(self)
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W::new(self)
    }
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn div_0_7(&mut self) -> DIV_0_7_W {
        DIV_0_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctl](index.html) module"]
pub struct CLKCTL_SPEC;
impl crate::RegisterSpec for CLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkctl::R](R) reader structure"]
impl crate::Readable for CLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkctl::W](W) writer structure"]
impl crate::Writable for CLKCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for CLKCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
