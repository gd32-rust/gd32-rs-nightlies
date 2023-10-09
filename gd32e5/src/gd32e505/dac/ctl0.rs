#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `DEN0` reader - DAC0 enable"]
pub type DEN0_R = crate::BitReader;
#[doc = "Field `DEN0` writer - DAC0 enable"]
pub type DEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBOFF0` reader - DAC0 output buffer turn off"]
pub type DBOFF0_R = crate::BitReader;
#[doc = "Field `DBOFF0` writer - DAC0 output buffer turn off"]
pub type DBOFF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTEN0` reader - DAC0 trigger enable"]
pub type DTEN0_R = crate::BitReader;
#[doc = "Field `DTEN0` writer - DAC0 trigger enable"]
pub type DTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTSEL0` reader - DAC0 trigger selection"]
pub type DTSEL0_R = crate::FieldReader;
#[doc = "Field `DTSEL0` writer - DAC0 trigger selection"]
pub type DTSEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DWM0` reader - DAC0 noise wave mode"]
pub type DWM0_R = crate::FieldReader;
#[doc = "Field `DWM0` writer - DAC0 noise wave mode"]
pub type DWM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DWBW0` reader - DAC0 noise wave bit width"]
pub type DWBW0_R = crate::FieldReader;
#[doc = "Field `DWBW0` writer - DAC0 noise wave bit width"]
pub type DWBW0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DDMAEN0` reader - DAC0 DMA enable"]
pub type DDMAEN0_R = crate::BitReader;
#[doc = "Field `DDMAEN0` writer - DAC0 DMA enable"]
pub type DDMAEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDUDRIE0` reader - DAC_OUT0 DMA underrun interrupt enable"]
pub type DDUDRIE0_R = crate::BitReader;
#[doc = "Field `DDUDRIE0` writer - DAC_OUT0 DMA underrun interrupt enable"]
pub type DDUDRIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTSEL0_3` reader - DAC_OUT0 trigger selection bit\\[3\\], refer to DTSEL0\\[2:0\\]"]
pub type DTSEL0_3_R = crate::BitReader;
#[doc = "Field `DTSEL0_3` writer - DAC_OUT0 trigger selection bit\\[3\\], refer to DTSEL0\\[2:0\\]"]
pub type DTSEL0_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEN1` reader - DAC1 enable"]
pub type DEN1_R = crate::BitReader;
#[doc = "Field `DEN1` writer - DAC1 enable"]
pub type DEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBOFF1` reader - DAC1 output buffer turn off"]
pub type DBOFF1_R = crate::BitReader;
#[doc = "Field `DBOFF1` writer - DAC1 output buffer turn off"]
pub type DBOFF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTEN1` reader - DAC1 trigger enable"]
pub type DTEN1_R = crate::BitReader;
#[doc = "Field `DTEN1` writer - DAC1 trigger enable"]
pub type DTEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTSEL1` reader - DAC1 trigger selection"]
pub type DTSEL1_R = crate::FieldReader;
#[doc = "Field `DTSEL1` writer - DAC1 trigger selection"]
pub type DTSEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DWM1` reader - DAC1 noise wave mode"]
pub type DWM1_R = crate::FieldReader;
#[doc = "Field `DWM1` writer - DAC1 noise wave mode"]
pub type DWM1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DWBW1` reader - DAC1 noise wave bit width"]
pub type DWBW1_R = crate::FieldReader;
#[doc = "Field `DWBW1` writer - DAC1 noise wave bit width"]
pub type DWBW1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DDMAEN1` reader - DAC1 DMA enable"]
pub type DDMAEN1_R = crate::BitReader;
#[doc = "Field `DDMAEN1` writer - DAC1 DMA enable"]
pub type DDMAEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDUDRIE1` reader - DAC_OUT1 DMA underrun interrupt enable"]
pub type DDUDRIE1_R = crate::BitReader;
#[doc = "Field `DDUDRIE1` writer - DAC_OUT1 DMA underrun interrupt enable"]
pub type DDUDRIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTSEL1_3` reader - DAC_OUT1 trigger selection bit\\[3\\], refer to DTSEL1\\[2:0\\]"]
pub type DTSEL1_3_R = crate::BitReader;
#[doc = "Field `DTSEL1_3` writer - DAC_OUT1 trigger selection bit\\[3\\], refer to DTSEL1\\[2:0\\]"]
pub type DTSEL1_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&self) -> DEN0_R {
        DEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&self) -> DBOFF0_R {
        DBOFF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&self) -> DTEN0_R {
        DTEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&self) -> DTSEL0_R {
        DTSEL0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    pub fn dwm0(&self) -> DWM0_R {
        DWM0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw0(&self) -> DWBW0_R {
        DWBW0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&self) -> DDMAEN0_R {
        DDMAEN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC_OUT0 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn ddudrie0(&self) -> DDUDRIE0_R {
        DDUDRIE0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC_OUT0 trigger selection bit\\[3\\], refer to DTSEL0\\[2:0\\]"]
    #[inline(always)]
    pub fn dtsel0_3(&self) -> DTSEL0_3_R {
        DTSEL0_3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&self) -> DEN1_R {
        DEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&self) -> DBOFF1_R {
        DBOFF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&self) -> DTEN1_R {
        DTEN1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&self) -> DTSEL1_R {
        DTSEL1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    pub fn dwm1(&self) -> DWM1_R {
        DWM1_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw1(&self) -> DWBW1_R {
        DWBW1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&self) -> DDMAEN1_R {
        DDMAEN1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC_OUT1 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn ddudrie1(&self) -> DDUDRIE1_R {
        DDUDRIE1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC_OUT1 trigger selection bit\\[3\\], refer to DTSEL1\\[2:0\\]"]
    #[inline(always)]
    pub fn dtsel1_3(&self) -> DTSEL1_3_R {
        DTSEL1_3_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den0(&mut self) -> DEN0_W<CTL0_SPEC, 0> {
        DEN0_W::new(self)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff0(&mut self) -> DBOFF0_W<CTL0_SPEC, 1> {
        DBOFF0_W::new(self)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten0(&mut self) -> DTEN0_W<CTL0_SPEC, 2> {
        DTEN0_W::new(self)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel0(&mut self) -> DTSEL0_W<CTL0_SPEC, 3> {
        DTSEL0_W::new(self)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm0(&mut self) -> DWM0_W<CTL0_SPEC, 6> {
        DWM0_W::new(self)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw0(&mut self) -> DWBW0_W<CTL0_SPEC, 8> {
        DWBW0_W::new(self)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen0(&mut self) -> DDMAEN0_W<CTL0_SPEC, 12> {
        DDMAEN0_W::new(self)
    }
    #[doc = "Bit 13 - DAC_OUT0 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddudrie0(&mut self) -> DDUDRIE0_W<CTL0_SPEC, 13> {
        DDUDRIE0_W::new(self)
    }
    #[doc = "Bit 14 - DAC_OUT0 trigger selection bit\\[3\\], refer to DTSEL0\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel0_3(&mut self) -> DTSEL0_3_W<CTL0_SPEC, 14> {
        DTSEL0_3_W::new(self)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den1(&mut self) -> DEN1_W<CTL0_SPEC, 16> {
        DEN1_W::new(self)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff1(&mut self) -> DBOFF1_W<CTL0_SPEC, 17> {
        DBOFF1_W::new(self)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten1(&mut self) -> DTEN1_W<CTL0_SPEC, 18> {
        DTEN1_W::new(self)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel1(&mut self) -> DTSEL1_W<CTL0_SPEC, 19> {
        DTSEL1_W::new(self)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm1(&mut self) -> DWM1_W<CTL0_SPEC, 22> {
        DWM1_W::new(self)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw1(&mut self) -> DWBW1_W<CTL0_SPEC, 24> {
        DWBW1_W::new(self)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen1(&mut self) -> DDMAEN1_W<CTL0_SPEC, 28> {
        DDMAEN1_W::new(self)
    }
    #[doc = "Bit 29 - DAC_OUT1 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddudrie1(&mut self) -> DDUDRIE1_W<CTL0_SPEC, 29> {
        DDUDRIE1_W::new(self)
    }
    #[doc = "Bit 30 - DAC_OUT1 trigger selection bit\\[3\\], refer to DTSEL1\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel1_3(&mut self) -> DTSEL1_3_W<CTL0_SPEC, 30> {
        DTSEL1_3_W::new(self)
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
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
