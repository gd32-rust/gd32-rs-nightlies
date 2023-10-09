#[doc = "Register `INIT` reader"]
pub type R = crate::R<INIT_SPEC>;
#[doc = "Register `INIT` writer"]
pub type W = crate::W<INIT_SPEC>;
#[doc = "Field `SQPI_CMDBIT` reader - Bit number of SQPI controller command phase"]
pub type SQPI_CMDBIT_R = crate::FieldReader;
#[doc = "Field `SQPI_CMDBIT` writer - Bit number of SQPI controller command phase"]
pub type SQPI_CMDBIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SQPI_CLKDIV` reader - Clock divider for SQPI output clock"]
pub type SQPI_CLKDIV_R = crate::FieldReader;
#[doc = "Field `SQPI_CLKDIV` writer - Clock divider for SQPI output clock"]
pub type SQPI_CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SQPI_ADDRBIT` reader - Bit number of SPI PSRAM address phase."]
pub type SQPI_ADDRBIT_R = crate::FieldReader;
#[doc = "Field `SQPI_ADDRBIT` writer - Bit number of SPI PSRAM address phase."]
pub type SQPI_ADDRBIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQPI_IDLEN` reader - SQPI controller external memory ID length"]
pub type SQPI_IDLEN_R = crate::FieldReader;
#[doc = "Field `SQPI_IDLEN` writer - SQPI controller external memory ID length"]
pub type SQPI_IDLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SQPI_PL` reader - Read data sample polarity."]
pub type SQPI_PL_R = crate::BitReader;
#[doc = "Field `SQPI_PL` writer - Read data sample polarity."]
pub type SQPI_PL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 16:17 - Bit number of SQPI controller command phase"]
    #[inline(always)]
    pub fn sqpi_cmdbit(&self) -> SQPI_CMDBIT_R {
        SQPI_CMDBIT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Clock divider for SQPI output clock"]
    #[inline(always)]
    pub fn sqpi_clkdiv(&self) -> SQPI_CLKDIV_R {
        SQPI_CLKDIV_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - Bit number of SPI PSRAM address phase."]
    #[inline(always)]
    pub fn sqpi_addrbit(&self) -> SQPI_ADDRBIT_R {
        SQPI_ADDRBIT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - SQPI controller external memory ID length"]
    #[inline(always)]
    pub fn sqpi_idlen(&self) -> SQPI_IDLEN_R {
        SQPI_IDLEN_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Read data sample polarity."]
    #[inline(always)]
    pub fn sqpi_pl(&self) -> SQPI_PL_R {
        SQPI_PL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Bit number of SQPI controller command phase"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_cmdbit(&mut self) -> SQPI_CMDBIT_W<INIT_SPEC, 16> {
        SQPI_CMDBIT_W::new(self)
    }
    #[doc = "Bits 18:23 - Clock divider for SQPI output clock"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_clkdiv(&mut self) -> SQPI_CLKDIV_W<INIT_SPEC, 18> {
        SQPI_CLKDIV_W::new(self)
    }
    #[doc = "Bits 24:28 - Bit number of SPI PSRAM address phase."]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_addrbit(&mut self) -> SQPI_ADDRBIT_W<INIT_SPEC, 24> {
        SQPI_ADDRBIT_W::new(self)
    }
    #[doc = "Bits 29:30 - SQPI controller external memory ID length"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_idlen(&mut self) -> SQPI_IDLEN_W<INIT_SPEC, 29> {
        SQPI_IDLEN_W::new(self)
    }
    #[doc = "Bit 31 - Read data sample polarity."]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_pl(&mut self) -> SQPI_PL_W<INIT_SPEC, 31> {
        SQPI_PL_W::new(self)
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
#[doc = "SQPI Initial Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INIT_SPEC;
impl crate::RegisterSpec for INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`init::R`](R) reader structure"]
impl crate::Readable for INIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`init::W`](W) writer structure"]
impl crate::Writable for INIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INIT to value 0x1801_0004"]
impl crate::Resettable for INIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x1801_0004;
}
