#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<CLKCTL_SPEC>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<CLKCTL_SPEC>;
#[doc = "Field `DIV_0_7` reader - Clock division"]
pub type DIV_0_7_R = crate::FieldReader;
#[doc = "Field `DIV_0_7` writer - Clock division"]
pub type DIV_0_7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLKEN` reader - SDIO_CLK clock output enable bit"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - SDIO_CLK clock output enable bit"]
pub type CLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKPWRSAV` reader - SDIO_CLK clock dynamic switch on/off for power saving"]
pub type CLKPWRSAV_R = crate::BitReader;
#[doc = "Field `CLKPWRSAV` writer - SDIO_CLK clock dynamic switch on/off for power saving"]
pub type CLKPWRSAV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKBYP` reader - Clock bypass enable bit"]
pub type CLKBYP_R = crate::BitReader;
#[doc = "Field `CLKBYP` writer - Clock bypass enable bit"]
pub type CLKBYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSMODE` reader - SDIO card bus mode control bit"]
pub type BUSMODE_R = crate::FieldReader;
#[doc = "Field `BUSMODE` writer - SDIO card bus mode control bit"]
pub type BUSMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CLKEDGE` reader - SDIO_CLK clock edge selection bit"]
pub type CLKEDGE_R = crate::BitReader;
#[doc = "Field `CLKEDGE` writer - SDIO_CLK clock edge selection bit"]
pub type CLKEDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HWCLKEN` reader - Hardware Clock Control enable bit"]
pub type HWCLKEN_R = crate::BitReader;
#[doc = "Field `HWCLKEN` writer - Hardware Clock Control enable bit"]
pub type HWCLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIV_8` reader - MSB of Clock division"]
pub type DIV_8_R = crate::BitReader;
#[doc = "Field `DIV_8` writer - MSB of Clock division"]
pub type DIV_8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn div_0_7(&self) -> DIV_0_7_R {
        DIV_0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    pub fn clkpwrsav(&self) -> CLKPWRSAV_R {
        CLKPWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    pub fn clkbyp(&self) -> CLKBYP_R {
        CLKBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    pub fn busmode(&self) -> BUSMODE_R {
        BUSMODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    pub fn clkedge(&self) -> CLKEDGE_R {
        CLKEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    pub fn hwclken(&self) -> HWCLKEN_R {
        HWCLKEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    pub fn div_8(&self) -> DIV_8_R {
        DIV_8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_7(&mut self) -> DIV_0_7_W<CLKCTL_SPEC, 0> {
        DIV_0_7_W::new(self)
    }
    #[doc = "Bit 8 - SDIO_CLK clock output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CLKCTL_SPEC, 8> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 9 - SDIO_CLK clock dynamic switch on/off for power saving"]
    #[inline(always)]
    #[must_use]
    pub fn clkpwrsav(&mut self) -> CLKPWRSAV_W<CLKCTL_SPEC, 9> {
        CLKPWRSAV_W::new(self)
    }
    #[doc = "Bit 10 - Clock bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkbyp(&mut self) -> CLKBYP_W<CLKCTL_SPEC, 10> {
        CLKBYP_W::new(self)
    }
    #[doc = "Bits 11:12 - SDIO card bus mode control bit"]
    #[inline(always)]
    #[must_use]
    pub fn busmode(&mut self) -> BUSMODE_W<CLKCTL_SPEC, 11> {
        BUSMODE_W::new(self)
    }
    #[doc = "Bit 13 - SDIO_CLK clock edge selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkedge(&mut self) -> CLKEDGE_W<CLKCTL_SPEC, 13> {
        CLKEDGE_W::new(self)
    }
    #[doc = "Bit 14 - Hardware Clock Control enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn hwclken(&mut self) -> HWCLKEN_W<CLKCTL_SPEC, 14> {
        HWCLKEN_W::new(self)
    }
    #[doc = "Bit 31 - MSB of Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn div_8(&mut self) -> DIV_8_W<CLKCTL_SPEC, 31> {
        DIV_8_W::new(self)
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
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTL_SPEC;
impl crate::RegisterSpec for CLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for CLKCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for CLKCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for CLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
