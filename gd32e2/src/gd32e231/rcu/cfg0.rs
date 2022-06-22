#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLDV` reader - The CK_PLL divide by 1 or 2 for CK_OUT"]
pub type PLLDV_R = crate::BitReader<bool>;
#[doc = "Field `PLLDV` writer - The CK_PLL divide by 1 or 2 for CK_OUT"]
pub type PLLDV_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 31>;
#[doc = "Field `CKOUTDIV` reader - The CK_OUT divider which the CK_OUT frequency can be reduced"]
pub type CKOUTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUTDIV` writer - The CK_OUT divider which the CK_OUT frequency can be reduced"]
pub type CKOUTDIV_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 3, 28>;
#[doc = "Field `PLLMF_MSB` reader - Bit 4 of PLLMF register"]
pub type PLLMF_MSB_R = crate::BitReader<bool>;
#[doc = "Field `PLLMF_MSB` writer - Bit 4 of PLLMF register"]
pub type PLLMF_MSB_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 27>;
#[doc = "Field `CKOUTSEL` reader - CK_OUT Clock Source Selection"]
pub type CKOUTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUTSEL` writer - CK_OUT Clock Source Selection"]
pub type CKOUTSEL_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 3, 24>;
#[doc = "Field `PLLMF` reader - PLL multiply factor"]
pub type PLLMF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMF` writer - PLL multiply factor"]
pub type PLLMF_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 4, 18>;
#[doc = "Field `PLLPREDV` reader - HXTAL divider for PLL source clock selection."]
pub type PLLPREDV_R = crate::BitReader<bool>;
#[doc = "Field `PLLPREDV` writer - HXTAL divider for PLL source clock selection."]
pub type PLLPREDV_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 17>;
#[doc = "Field `PLLSEL` reader - PLL Clock Source Selection"]
pub type PLLSEL_R = crate::BitReader<bool>;
#[doc = "Field `PLLSEL` writer - PLL Clock Source Selection"]
pub type PLLSEL_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 16>;
#[doc = "Field `ADCPSC` reader - ADC clock prescaler selection"]
pub type ADCPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCPSC` writer - ADC clock prescaler selection"]
pub type ADCPSC_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 2, 14>;
#[doc = "Field `APB2PSC` reader - APB2 prescaler selection"]
pub type APB2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB2PSC` writer - APB2 prescaler selection"]
pub type APB2PSC_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 3, 11>;
#[doc = "Field `APB1PSC` reader - APB1 prescaler selection"]
pub type APB1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB1PSC` writer - APB1 prescaler selection"]
pub type APB1PSC_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 3, 8>;
#[doc = "Field `AHBPSC` reader - AHB prescaler selection"]
pub type AHBPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBPSC` writer - AHB prescaler selection"]
pub type AHBPSC_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 4, 4>;
#[doc = "Field `SCSS` reader - System clock switch status"]
pub type SCSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCS` reader - System clock switch"]
pub type SCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCS` writer - System clock switch"]
pub type SCS_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bit 31 - The CK_PLL divide by 1 or 2 for CK_OUT"]
    #[inline(always)]
    pub fn plldv(&self) -> PLLDV_R {
        PLLDV_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 28:30 - The CK_OUT divider which the CK_OUT frequency can be reduced"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF register"]
    #[inline(always)]
    pub fn pllmf_msb(&self) -> PLLMF_MSB_R {
        PLLMF_MSB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 24:26 - CK_OUT Clock Source Selection"]
    #[inline(always)]
    pub fn ckoutsel(&self) -> CKOUTSEL_R {
        CKOUTSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 18:21 - PLL multiply factor"]
    #[inline(always)]
    pub fn pllmf(&self) -> PLLMF_R {
        PLLMF_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - HXTAL divider for PLL source clock selection."]
    #[inline(always)]
    pub fn pllpredv(&self) -> PLLPREDV_R {
        PLLPREDV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PLLSEL_R {
        PLLSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&self) -> ADCPSC_R {
        ADCPSC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> APB2PSC_R {
        APB2PSC_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> APB1PSC_R {
        APB1PSC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AHBPSC_R {
        AHBPSC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> SCSS_R {
        SCSS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - The CK_PLL divide by 1 or 2 for CK_OUT"]
    #[inline(always)]
    pub fn plldv(&mut self) -> PLLDV_W {
        PLLDV_W::new(self)
    }
    #[doc = "Bits 28:30 - The CK_OUT divider which the CK_OUT frequency can be reduced"]
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W {
        CKOUTDIV_W::new(self)
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF register"]
    #[inline(always)]
    pub fn pllmf_msb(&mut self) -> PLLMF_MSB_W {
        PLLMF_MSB_W::new(self)
    }
    #[doc = "Bits 24:26 - CK_OUT Clock Source Selection"]
    #[inline(always)]
    pub fn ckoutsel(&mut self) -> CKOUTSEL_W {
        CKOUTSEL_W::new(self)
    }
    #[doc = "Bits 18:21 - PLL multiply factor"]
    #[inline(always)]
    pub fn pllmf(&mut self) -> PLLMF_W {
        PLLMF_W::new(self)
    }
    #[doc = "Bit 17 - HXTAL divider for PLL source clock selection."]
    #[inline(always)]
    pub fn pllpredv(&mut self) -> PLLPREDV_W {
        PLLPREDV_W::new(self)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&mut self) -> PLLSEL_W {
        PLLSEL_W::new(self)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&mut self) -> ADCPSC_W {
        ADCPSC_W::new(self)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&mut self) -> APB2PSC_W {
        APB2PSC_W::new(self)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&mut self) -> APB1PSC_W {
        APB1PSC_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&mut self) -> AHBPSC_W {
        AHBPSC_W::new(self)
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 0 (RCU_CFG0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
