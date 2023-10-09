#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `EWIF` reader - Early wakeup interrupt flag"]
pub type EWIF_R = crate::BitReader<EWIFR_A>;
#[doc = "Early wakeup interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFR_A {
    #[doc = "1: The EWI Interrupt Service Routine has been triggered"]
    PENDING = 1,
    #[doc = "0: The EWI Interrupt Service Routine has been serviced"]
    FINISHED = 0,
}
impl From<EWIFR_A> for bool {
    #[inline(always)]
    fn from(variant: EWIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl EWIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWIFR_A {
        match self.bits {
            true => EWIFR_A::PENDING,
            false => EWIFR_A::FINISHED,
        }
    }
    #[doc = "The EWI Interrupt Service Routine has been triggered"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIFR_A::PENDING
    }
    #[doc = "The EWI Interrupt Service Routine has been serviced"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == EWIFR_A::FINISHED
    }
}
#[doc = "Early wakeup interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFW_AW {
    #[doc = "0: The EWI Interrupt Service Routine has been serviced"]
    FINISHED = 0,
}
impl From<EWIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: EWIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIF` writer - Early wakeup interrupt flag"]
pub type EWIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EWIFW_AW>;
impl<'a, REG, const O: u8> EWIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EWI Interrupt Service Routine has been serviced"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut crate::W<REG> {
        self.variant(EWIFW_AW::FINISHED)
    }
}
impl R {
    #[doc = "Bit 0 - Early wakeup interrupt flag"]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ewif(&mut self) -> EWIF_W<STAT_SPEC, 0> {
        EWIF_W::new(self)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
