#[doc = "Register `WCMD` reader"]
pub type R = crate::R<WCMD_SPEC>;
#[doc = "Register `WCMD` writer"]
pub type W = crate::W<WCMD_SPEC>;
#[doc = "Field `SPI_WCMD` reader - SQPI write command for AHB write transfer"]
pub type SPI_WCMD_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_WCMD` writer - SQPI write command for AHB write transfer"]
pub type SPI_WCMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `SQPI_WWAITCYCLE` reader - SQPI write command waitcycle number after address phase"]
pub type SQPI_WWAITCYCLE_R = crate::FieldReader;
#[doc = "Field `SQPI_WWAITCYCLE` writer - SQPI write command waitcycle number after address phase"]
pub type SQPI_WWAITCYCLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SQPI_WMODE` reader - SQPI controller write command mode"]
pub type SQPI_WMODE_R = crate::FieldReader;
#[doc = "Field `SQPI_WMODE` writer - SQPI controller write command mode"]
pub type SQPI_WMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SQPI_SC` reader - Send special command"]
pub type SQPI_SC_R = crate::BitReader;
#[doc = "Field `SQPI_SC` writer - Send special command"]
pub type SQPI_SC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - SQPI write command for AHB write transfer"]
    #[inline(always)]
    pub fn spi_wcmd(&self) -> SPI_WCMD_R {
        SPI_WCMD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - SQPI write command waitcycle number after address phase"]
    #[inline(always)]
    pub fn sqpi_wwaitcycle(&self) -> SQPI_WWAITCYCLE_R {
        SQPI_WWAITCYCLE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - SQPI controller write command mode"]
    #[inline(always)]
    pub fn sqpi_wmode(&self) -> SQPI_WMODE_R {
        SQPI_WMODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 31 - Send special command"]
    #[inline(always)]
    pub fn sqpi_sc(&self) -> SQPI_SC_R {
        SQPI_SC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - SQPI write command for AHB write transfer"]
    #[inline(always)]
    #[must_use]
    pub fn spi_wcmd(&mut self) -> SPI_WCMD_W<WCMD_SPEC, 0> {
        SPI_WCMD_W::new(self)
    }
    #[doc = "Bits 16:19 - SQPI write command waitcycle number after address phase"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_wwaitcycle(&mut self) -> SQPI_WWAITCYCLE_W<WCMD_SPEC, 16> {
        SQPI_WWAITCYCLE_W::new(self)
    }
    #[doc = "Bits 20:22 - SQPI controller write command mode"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_wmode(&mut self) -> SQPI_WMODE_W<WCMD_SPEC, 20> {
        SQPI_WMODE_W::new(self)
    }
    #[doc = "Bit 31 - Send special command"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_sc(&mut self) -> SQPI_SC_W<WCMD_SPEC, 31> {
        SQPI_SC_W::new(self)
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
#[doc = "Write Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WCMD_SPEC;
impl crate::RegisterSpec for WCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wcmd::R`](R) reader structure"]
impl crate::Readable for WCMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wcmd::W`](W) writer structure"]
impl crate::Writable for WCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCMD to value 0x0001_0000"]
impl crate::Resettable for WCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
