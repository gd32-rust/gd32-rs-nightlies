#[doc = "Register `SNCTL2` reader"]
pub struct R(crate::R<SNCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNCTL2` writer"]
pub struct W(crate::W<SNCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNCTL2_SPEC>;
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
impl From<crate::W<SNCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCWR` reader - Synchronous write"]
pub type SYNCWR_R = crate::BitReader<bool>;
#[doc = "Field `SYNCWR` writer - Synchronous write"]
pub type SYNCWR_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 19>;
#[doc = "Field `ASYNCWAIT` reader - Asynchronous wait"]
pub type ASYNCWAIT_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCWAIT` writer - Asynchronous wait"]
pub type ASYNCWAIT_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 15>;
#[doc = "Field `EXMODEN` reader - Extended mode enable"]
pub type EXMODEN_R = crate::BitReader<bool>;
#[doc = "Field `EXMODEN` writer - Extended mode enable"]
pub type EXMODEN_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 14>;
#[doc = "Field `NRWTEN` reader - NWAIT signal enable"]
pub type NRWTEN_R = crate::BitReader<bool>;
#[doc = "Field `NRWTEN` writer - NWAIT signal enable"]
pub type NRWTEN_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 13>;
#[doc = "Field `WREN` reader - Write enable"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - Write enable"]
pub type WREN_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 12>;
#[doc = "Field `NRWTCFG` reader - NWAIT signal configuration, only work in synchronous mode"]
pub type NRWTCFG_R = crate::BitReader<bool>;
#[doc = "Field `NRWTCFG` writer - NWAIT signal configuration, only work in synchronous mode"]
pub type NRWTCFG_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 11>;
#[doc = "Field `WRAPEN` reader - Wrapped burst mode enable"]
pub type WRAPEN_R = crate::BitReader<bool>;
#[doc = "Field `WRAPEN` writer - Wrapped burst mode enable"]
pub type WRAPEN_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 10>;
#[doc = "Field `NRWTPOL` reader - NWAIT signal polarity"]
pub type NRWTPOL_R = crate::BitReader<bool>;
#[doc = "Field `NRWTPOL` writer - NWAIT signal polarity"]
pub type NRWTPOL_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 9>;
#[doc = "Field `SBRSTEN` reader - Synchronous burst enable"]
pub type SBRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `SBRSTEN` writer - Synchronous burst enable"]
pub type SBRSTEN_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 8>;
#[doc = "Field `NREN` reader - NOR Flash access enable"]
pub type NREN_R = crate::BitReader<bool>;
#[doc = "Field `NREN` writer - NOR Flash access enable"]
pub type NREN_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 6>;
#[doc = "Field `NRW` reader - NOR bank memory data bus width"]
pub type NRW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRW` writer - NOR bank memory data bus width"]
pub type NRW_W<'a> = crate::FieldWriter<'a, u32, SNCTL2_SPEC, u8, u8, 2, 4>;
#[doc = "Field `NRTP` reader - NOR bank memory type"]
pub type NRTP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRTP` writer - NOR bank memory type"]
pub type NRTP_W<'a> = crate::FieldWriter<'a, u32, SNCTL2_SPEC, u8, u8, 2, 2>;
#[doc = "Field `NRMUX` reader - NOR bank memory address/data multiplexing"]
pub type NRMUX_R = crate::BitReader<bool>;
#[doc = "Field `NRMUX` writer - NOR bank memory address/data multiplexing"]
pub type NRMUX_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 1>;
#[doc = "Field `NRBKEN` reader - NOR bank enable"]
pub type NRBKEN_R = crate::BitReader<bool>;
#[doc = "Field `NRBKEN` writer - NOR bank enable"]
pub type NRBKEN_W<'a> = crate::BitWriter<'a, u32, SNCTL2_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 19 - Synchronous write"]
    #[inline(always)]
    pub fn syncwr(&self) -> SYNCWR_R {
        SYNCWR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    pub fn exmoden(&self) -> EXMODEN_R {
        EXMODEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    pub fn nrwten(&self) -> NRWTEN_R {
        NRWTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - NWAIT signal configuration, only work in synchronous mode"]
    #[inline(always)]
    pub fn nrwtcfg(&self) -> NRWTCFG_R {
        NRWTCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Wrapped burst mode enable"]
    #[inline(always)]
    pub fn wrapen(&self) -> WRAPEN_R {
        WRAPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    pub fn nrwtpol(&self) -> NRWTPOL_R {
        NRWTPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    pub fn sbrsten(&self) -> SBRSTEN_R {
        SBRSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    pub fn nren(&self) -> NREN_R {
        NREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    pub fn nrw(&self) -> NRW_R {
        NRW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    pub fn nrtp(&self) -> NRTP_R {
        NRTP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    pub fn nrmux(&self) -> NRMUX_R {
        NRMUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    pub fn nrbken(&self) -> NRBKEN_R {
        NRBKEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - Synchronous write"]
    #[inline(always)]
    pub fn syncwr(&mut self) -> SYNCWR_W {
        SYNCWR_W::new(self)
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W {
        ASYNCWAIT_W::new(self)
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    pub fn exmoden(&mut self) -> EXMODEN_W {
        EXMODEN_W::new(self)
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    pub fn nrwten(&mut self) -> NRWTEN_W {
        NRWTEN_W::new(self)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W::new(self)
    }
    #[doc = "Bit 11 - NWAIT signal configuration, only work in synchronous mode"]
    #[inline(always)]
    pub fn nrwtcfg(&mut self) -> NRWTCFG_W {
        NRWTCFG_W::new(self)
    }
    #[doc = "Bit 10 - Wrapped burst mode enable"]
    #[inline(always)]
    pub fn wrapen(&mut self) -> WRAPEN_W {
        WRAPEN_W::new(self)
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    pub fn nrwtpol(&mut self) -> NRWTPOL_W {
        NRWTPOL_W::new(self)
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    pub fn sbrsten(&mut self) -> SBRSTEN_W {
        SBRSTEN_W::new(self)
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    pub fn nren(&mut self) -> NREN_W {
        NREN_W::new(self)
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    pub fn nrw(&mut self) -> NRW_W {
        NRW_W::new(self)
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    pub fn nrtp(&mut self) -> NRTP_W {
        NRTP_W::new(self)
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    pub fn nrmux(&mut self) -> NRMUX_W {
        NRMUX_W::new(self)
    }
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    pub fn nrbken(&mut self) -> NRBKEN_W {
        NRBKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR flash control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl2](index.html) module"]
pub struct SNCTL2_SPEC;
impl crate::RegisterSpec for SNCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snctl2::R](R) reader structure"]
impl crate::Readable for SNCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snctl2::W](W) writer structure"]
impl crate::Writable for SNCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNCTL2 to value 0x30d2"]
impl crate::Resettable for SNCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30d2
    }
}
