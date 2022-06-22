#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LMEN_R = crate::BitReader<bool>;
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LMEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 14>;
#[doc = "Field `STB` reader - STOP bits length"]
pub type STB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STB` writer - STOP bits length"]
pub type STB_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 2, 12>;
#[doc = "Field `CKEN` reader - CK pin enable"]
pub type CKEN_R = crate::BitReader<bool>;
#[doc = "Field `CKEN` writer - CK pin enable"]
pub type CKEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 11>;
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CPL_R = crate::BitReader<bool>;
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CPL_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 10>;
#[doc = "Field `CPH` reader - Clock phase"]
pub type CPH_R = crate::BitReader<bool>;
#[doc = "Field `CPH` writer - Clock phase"]
pub type CPH_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 9>;
#[doc = "Field `CLEN` reader - CK Length"]
pub type CLEN_R = crate::BitReader<bool>;
#[doc = "Field `CLEN` writer - CK Length"]
pub type CLEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 8>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader<bool>;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 6>;
#[doc = "Field `LBLEN` reader - LIN break frame length"]
pub type LBLEN_R = crate::BitReader<bool>;
#[doc = "Field `LBLEN` writer - LIN break frame length"]
pub type LBLEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 5>;
#[doc = "Field `ADDR` reader - Address of the USART"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - Address of the USART"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LMEN_R {
        LMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&self) -> STB_R {
        STB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CPH_R {
        CPH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&self) -> LBLEN_R {
        LBLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&mut self) -> LMEN_W {
        LMEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&mut self) -> STB_W {
        STB_W::new(self)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W {
        CKEN_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&mut self) -> CPH_W {
        CPH_W::new(self)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    pub fn clen(&mut self) -> CLEN_W {
        CLEN_W::new(self)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W {
        LBDIE_W::new(self)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&mut self) -> LBLEN_W {
        LBLEN_W::new(self)
    }
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
