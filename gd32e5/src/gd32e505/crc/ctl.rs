#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `RST` reader - reset bit"]
pub type RST_R = crate::BitReader<RSTW_A>;
#[doc = "reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTW_A {
    #[doc = "1: Resets the DATA register to IDATA, with no effect on FDATA"]
    RESET = 1,
}
impl From<RSTW_A> for bool {
    #[inline(always)]
    fn from(variant: RSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTW_A> {
        match self.bits {
            true => Some(RSTW_A::RESET),
            _ => None,
        }
    }
    #[doc = "Resets the DATA register to IDATA, with no effect on FDATA"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RSTW_A::RESET
    }
}
#[doc = "Field `RST` writer - reset bit"]
pub type RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RSTW_A>;
impl<'a, REG, const O: u8> RST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the DATA register to IDATA, with no effect on FDATA"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RSTW_A::RESET)
    }
}
#[doc = "Field `PS` reader - Size of polynomial"]
pub type PS_R = crate::FieldReader;
#[doc = "Field `PS` writer - Size of polynomial"]
pub type PS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `REV_I` reader - Reverse type for input data"]
pub type REV_I_R = crate::FieldReader;
#[doc = "Field `REV_I` writer - Reverse type for input data"]
pub type REV_I_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `REV_O` reader - Reverse output data value in bit order"]
pub type REV_O_R = crate::BitReader;
#[doc = "Field `REV_O` writer - Reverse output data value in bit order"]
pub type REV_O_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Size of polynomial"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse type for input data"]
    #[inline(always)]
    pub fn rev_i(&self) -> REV_I_R {
        REV_I_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data value in bit order"]
    #[inline(always)]
    pub fn rev_o(&self) -> REV_O_R {
        REV_O_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<CTL_SPEC, 0> {
        RST_W::new(self)
    }
    #[doc = "Bits 3:4 - Size of polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<CTL_SPEC, 3> {
        PS_W::new(self)
    }
    #[doc = "Bits 5:6 - Reverse type for input data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_i(&mut self) -> REV_I_W<CTL_SPEC, 5> {
        REV_I_W::new(self)
    }
    #[doc = "Bit 7 - Reverse output data value in bit order"]
    #[inline(always)]
    #[must_use]
    pub fn rev_o(&mut self) -> REV_O_W<CTL_SPEC, 7> {
        REV_O_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
