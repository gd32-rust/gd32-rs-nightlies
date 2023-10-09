#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `DDUDR0` reader - DAC0 DMA underrun flag"]
pub type DDUDR0_R = crate::BitReader<DDUDR0_A>;
#[doc = "DAC0 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDUDR0_A {
    #[doc = "0: No DMA underrun error condition occurred"]
    NO_UNDERRUN = 0,
    #[doc = "1: DMA underrun error condition occurred"]
    UNDERRUN = 1,
}
impl From<DDUDR0_A> for bool {
    #[inline(always)]
    fn from(variant: DDUDR0_A) -> Self {
        variant as u8 != 0
    }
}
impl DDUDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDUDR0_A {
        match self.bits {
            false => DDUDR0_A::NO_UNDERRUN,
            true => DDUDR0_A::UNDERRUN,
        }
    }
    #[doc = "No DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == DDUDR0_A::NO_UNDERRUN
    }
    #[doc = "DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == DDUDR0_A::UNDERRUN
    }
}
#[doc = "Field `DDUDR0` writer - DAC0 DMA underrun flag"]
pub type DDUDR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DDUDR0_A>;
impl<'a, REG, const O: u8> DDUDR0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut crate::W<REG> {
        self.variant(DDUDR0_A::NO_UNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut crate::W<REG> {
        self.variant(DDUDR0_A::UNDERRUN)
    }
}
impl R {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr0(&self) -> DDUDR0_R {
        DDUDR0_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn ddudr0(&mut self) -> DDUDR0_W<STAT_SPEC, 13> {
        DDUDR0_W::new(self)
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
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
