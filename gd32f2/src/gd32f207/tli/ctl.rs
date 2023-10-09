#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `TLIEN` reader - TLI enable bit"]
pub type TLIEN_R = crate::BitReader;
#[doc = "Field `TLIEN` writer - TLI enable bit"]
pub type TLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BDB` reader - Blue channel Dither Bits Number"]
pub type BDB_R = crate::FieldReader;
#[doc = "Field `GDB` reader - Green channel Dither Bits Number"]
pub type GDB_R = crate::FieldReader;
#[doc = "Field `RDB` reader - Red channel Dither Bits Number"]
pub type RDB_R = crate::FieldReader;
#[doc = "Field `DFEN` reader - Dither Function Enable"]
pub type DFEN_R = crate::BitReader;
#[doc = "Field `DFEN` writer - Dither Function Enable"]
pub type DFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKPS` reader - Pixel Clock Polarity Selection"]
pub type CLKPS_R = crate::BitReader;
#[doc = "Field `CLKPS` writer - Pixel Clock Polarity Selection"]
pub type CLKPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEPS` reader - Data Enable Polarity Selection"]
pub type DEPS_R = crate::BitReader;
#[doc = "Field `DEPS` writer - Data Enable Polarity Selection"]
pub type DEPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VPPS` reader - Vertical Pulse Polarity Selection"]
pub type VPPS_R = crate::BitReader;
#[doc = "Field `VPPS` writer - Vertical Pulse Polarity Selection"]
pub type VPPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPPS` reader - Horizontal Pulse Polarity Selection"]
pub type HPPS_R = crate::BitReader;
#[doc = "Field `HPPS` writer - Horizontal Pulse Polarity Selection"]
pub type HPPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TLI enable bit"]
    #[inline(always)]
    pub fn tlien(&self) -> TLIEN_R {
        TLIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Blue channel Dither Bits Number"]
    #[inline(always)]
    pub fn bdb(&self) -> BDB_R {
        BDB_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Green channel Dither Bits Number"]
    #[inline(always)]
    pub fn gdb(&self) -> GDB_R {
        GDB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Red channel Dither Bits Number"]
    #[inline(always)]
    pub fn rdb(&self) -> RDB_R {
        RDB_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Dither Function Enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity Selection"]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity Selection"]
    #[inline(always)]
    pub fn deps(&self) -> DEPS_R {
        DEPS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Pulse Polarity Selection"]
    #[inline(always)]
    pub fn vpps(&self) -> VPPS_R {
        VPPS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Horizontal Pulse Polarity Selection"]
    #[inline(always)]
    pub fn hpps(&self) -> HPPS_R {
        HPPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TLI enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tlien(&mut self) -> TLIEN_W<CTL_SPEC, 0> {
        TLIEN_W::new(self)
    }
    #[doc = "Bit 16 - Dither Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DFEN_W<CTL_SPEC, 16> {
        DFEN_W::new(self)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clkps(&mut self) -> CLKPS_W<CTL_SPEC, 28> {
        CLKPS_W::new(self)
    }
    #[doc = "Bit 29 - Data Enable Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn deps(&mut self) -> DEPS_W<CTL_SPEC, 29> {
        DEPS_W::new(self)
    }
    #[doc = "Bit 30 - Vertical Pulse Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vpps(&mut self) -> VPPS_W<CTL_SPEC, 30> {
        VPPS_W::new(self)
    }
    #[doc = "Bit 31 - Horizontal Pulse Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn hpps(&mut self) -> HPPS_W<CTL_SPEC, 31> {
        HPPS_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x2220"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2220;
}
