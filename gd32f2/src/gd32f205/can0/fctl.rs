#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FCTL_SPEC>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FCTL_SPEC>;
#[doc = "Field `FLD` reader - Filter lock disable"]
pub type FLD_R = crate::BitReader<FLD_A>;
#[doc = "Filter lock disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLD_A {
    #[doc = "0: Filter lock disabled"]
    DISABLED = 0,
    #[doc = "1: Filter lock enabled"]
    ENABLED = 1,
}
impl From<FLD_A> for bool {
    #[inline(always)]
    fn from(variant: FLD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLD_A {
        match self.bits {
            false => FLD_A::DISABLED,
            true => FLD_A::ENABLED,
        }
    }
    #[doc = "Filter lock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLD_A::DISABLED
    }
    #[doc = "Filter lock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLD_A::ENABLED
    }
}
#[doc = "Field `FLD` writer - Filter lock disable"]
pub type FLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLD_A>;
impl<'a, REG, const O: u8> FLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter lock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLD_A::DISABLED)
    }
    #[doc = "Filter lock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLD_A::ENABLED)
    }
}
#[doc = "Field `HBC1F` reader - Header bank of CAN1 filter"]
pub type HBC1F_R = crate::FieldReader;
#[doc = "Field `HBC1F` writer - Header bank of CAN1 filter"]
pub type HBC1F_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    pub fn fld(&self) -> FLD_R {
        FLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    pub fn hbc1f(&self) -> HBC1F_R {
        HBC1F_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    #[must_use]
    pub fn fld(&mut self) -> FLD_W<FCTL_SPEC, 0> {
        FLD_W::new(self)
    }
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn hbc1f(&mut self) -> HBC1F_W<FCTL_SPEC, 8> {
        HBC1F_W::new(self)
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
#[doc = "Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL_SPEC;
impl crate::RegisterSpec for FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL to value 0x2a1c_0e01"]
impl crate::Resettable for FCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2a1c_0e01;
}
