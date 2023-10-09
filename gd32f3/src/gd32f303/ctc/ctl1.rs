#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `RLVALUE` reader - CTC counter reload value"]
pub type RLVALUE_R = crate::FieldReader<u16>;
#[doc = "Field `RLVALUE` writer - CTC counter reload value"]
pub type RLVALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CKLIM` reader - Clock trim base limit value"]
pub type CKLIM_R = crate::FieldReader;
#[doc = "Field `CKLIM` writer - Clock trim base limit value"]
pub type CKLIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `REFPSC` reader - Reference signal source prescaler"]
pub type REFPSC_R = crate::FieldReader;
#[doc = "Field `REFPSC` writer - Reference signal source prescaler"]
pub type REFPSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `REFSEL` reader - Reference signal source selection"]
pub type REFSEL_R = crate::FieldReader;
#[doc = "Field `REFSEL` writer - Reference signal source selection"]
pub type REFSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `REFPOL` reader - Reference signal source polarity"]
pub type REFPOL_R = crate::BitReader;
#[doc = "Field `REFPOL` writer - Reference signal source polarity"]
pub type REFPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&self) -> RLVALUE_R {
        RLVALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    pub fn cklim(&self) -> CKLIM_R {
        CKLIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    pub fn refpsc(&self) -> REFPSC_R {
        REFPSC_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&self) -> REFPOL_R {
        REFPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    #[must_use]
    pub fn rlvalue(&mut self) -> RLVALUE_W<CTL1_SPEC, 0> {
        RLVALUE_W::new(self)
    }
    #[doc = "Bits 16:23 - Clock trim base limit value"]
    #[inline(always)]
    #[must_use]
    pub fn cklim(&mut self) -> CKLIM_W<CTL1_SPEC, 16> {
        CKLIM_W::new(self)
    }
    #[doc = "Bits 24:26 - Reference signal source prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn refpsc(&mut self) -> REFPSC_W<CTL1_SPEC, 24> {
        REFPSC_W::new(self)
    }
    #[doc = "Bits 28:29 - Reference signal source selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<CTL1_SPEC, 28> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    #[must_use]
    pub fn refpol(&mut self) -> REFPOL_W<CTL1_SPEC, 31> {
        REFPOL_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x2022_bb7f"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2022_bb7f;
}
