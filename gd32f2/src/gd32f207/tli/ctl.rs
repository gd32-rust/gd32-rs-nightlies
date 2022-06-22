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
#[doc = "Field `HPPS` reader - Horizontal Pulse Polarity Selection"]
pub type HPPS_R = crate::BitReader<bool>;
#[doc = "Field `HPPS` writer - Horizontal Pulse Polarity Selection"]
pub type HPPS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 31>;
#[doc = "Field `VPPS` reader - Vertical Pulse Polarity Selection"]
pub type VPPS_R = crate::BitReader<bool>;
#[doc = "Field `VPPS` writer - Vertical Pulse Polarity Selection"]
pub type VPPS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 30>;
#[doc = "Field `DEPS` reader - Data Enable Polarity Selection"]
pub type DEPS_R = crate::BitReader<bool>;
#[doc = "Field `DEPS` writer - Data Enable Polarity Selection"]
pub type DEPS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 29>;
#[doc = "Field `CLKPS` reader - Pixel Clock Polarity Selection"]
pub type CLKPS_R = crate::BitReader<bool>;
#[doc = "Field `CLKPS` writer - Pixel Clock Polarity Selection"]
pub type CLKPS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 28>;
#[doc = "Field `DFEN` reader - Dither Function Enable"]
pub type DFEN_R = crate::BitReader<bool>;
#[doc = "Field `DFEN` writer - Dither Function Enable"]
pub type DFEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 16>;
#[doc = "Field `RDB` reader - Red channel Dither Bits Number"]
pub type RDB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GDB` reader - Green channel Dither Bits Number"]
pub type GDB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BDB` reader - Blue channel Dither Bits Number"]
pub type BDB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLIEN` reader - TLI enable bit"]
pub type TLIEN_R = crate::BitReader<bool>;
#[doc = "Field `TLIEN` writer - TLI enable bit"]
pub type TLIEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 31 - Horizontal Pulse Polarity Selection"]
    #[inline(always)]
    pub fn hpps(&self) -> HPPS_R {
        HPPS_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Pulse Polarity Selection"]
    #[inline(always)]
    pub fn vpps(&self) -> VPPS_R {
        VPPS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity Selection"]
    #[inline(always)]
    pub fn deps(&self) -> DEPS_R {
        DEPS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity Selection"]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 16 - Dither Function Enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Red channel Dither Bits Number"]
    #[inline(always)]
    pub fn rdb(&self) -> RDB_R {
        RDB_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Green channel Dither Bits Number"]
    #[inline(always)]
    pub fn gdb(&self) -> GDB_R {
        GDB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Blue channel Dither Bits Number"]
    #[inline(always)]
    pub fn bdb(&self) -> BDB_R {
        BDB_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 0 - TLI enable bit"]
    #[inline(always)]
    pub fn tlien(&self) -> TLIEN_R {
        TLIEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Horizontal Pulse Polarity Selection"]
    #[inline(always)]
    pub fn hpps(&mut self) -> HPPS_W {
        HPPS_W::new(self)
    }
    #[doc = "Bit 30 - Vertical Pulse Polarity Selection"]
    #[inline(always)]
    pub fn vpps(&mut self) -> VPPS_W {
        VPPS_W::new(self)
    }
    #[doc = "Bit 29 - Data Enable Polarity Selection"]
    #[inline(always)]
    pub fn deps(&mut self) -> DEPS_W {
        DEPS_W::new(self)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity Selection"]
    #[inline(always)]
    pub fn clkps(&mut self) -> CLKPS_W {
        CLKPS_W::new(self)
    }
    #[doc = "Bit 16 - Dither Function Enable"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W {
        DFEN_W::new(self)
    }
    #[doc = "Bit 0 - TLI enable bit"]
    #[inline(always)]
    pub fn tlien(&mut self) -> TLIEN_W {
        TLIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
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
#[doc = "`reset()` method sets CTL to value 0x2220"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2220
    }
}
