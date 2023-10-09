#[doc = "Register `PCFE` reader"]
pub type R = crate::R<PCFE_SPEC>;
#[doc = "Register `PCFE` writer"]
pub type W = crate::W<PCFE_SPEC>;
#[doc = "Field `PE0_AFCFG` reader - PE0 AF function configuration bitse"]
pub type PE0_AFCFG_R = crate::FieldReader;
#[doc = "Field `PE0_AFCFG` writer - PE0 AF function configuration bitse"]
pub type PE0_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PE1_AFCFG` reader - PE1 AF function configuration bitse"]
pub type PE1_AFCFG_R = crate::FieldReader;
#[doc = "Field `PE1_AFCFG` writer - PE1 AF function configuration bitse"]
pub type PE1_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PE8_AFCFG` reader - PE8 AF function configuration bitse"]
pub type PE8_AFCFG_R = crate::FieldReader;
#[doc = "Field `PE8_AFCFG` writer - PE8 AF function configuration bitse"]
pub type PE8_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PE9_AFCFG` reader - PE9 AF function configuration bitse"]
pub type PE9_AFCFG_R = crate::FieldReader;
#[doc = "Field `PE9_AFCFG` writer - PE9 AF function configuration bitse"]
pub type PE9_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PE10_AFCFG` reader - PE10 AF function configuration bitse"]
pub type PE10_AFCFG_R = crate::BitReader;
#[doc = "Field `PE10_AFCFG` writer - PE10 AF function configuration bitse"]
pub type PE10_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE11_AFCFG` reader - PE11 AF function configuration bitse"]
pub type PE11_AFCFG_R = crate::FieldReader;
#[doc = "Field `PE11_AFCFG` writer - PE11 AF function configuration bitse"]
pub type PE11_AFCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PE12_AFCFG` reader - PE12 AF function configuration bitse"]
pub type PE12_AFCFG_R = crate::BitReader;
#[doc = "Field `PE12_AFCFG` writer - PE12 AF function configuration bitse"]
pub type PE12_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE13_AFCFG` reader - PE13 AF function configuration bitse"]
pub type PE13_AFCFG_R = crate::BitReader;
#[doc = "Field `PE13_AFCFG` writer - PE13 AF function configuration bitse"]
pub type PE13_AFCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - PE0 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe0_afcfg(&self) -> PE0_AFCFG_R {
        PE0_AFCFG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PE1 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe1_afcfg(&self) -> PE1_AFCFG_R {
        PE1_AFCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PE8 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe8_afcfg(&self) -> PE8_AFCFG_R {
        PE8_AFCFG_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PE9 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe9_afcfg(&self) -> PE9_AFCFG_R {
        PE9_AFCFG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - PE10 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe10_afcfg(&self) -> PE10_AFCFG_R {
        PE10_AFCFG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - PE11 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe11_afcfg(&self) -> PE11_AFCFG_R {
        PE11_AFCFG_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PE12 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe12_afcfg(&self) -> PE12_AFCFG_R {
        PE12_AFCFG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PE13 AF function configuration bitse"]
    #[inline(always)]
    pub fn pe13_afcfg(&self) -> PE13_AFCFG_R {
        PE13_AFCFG_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PE0 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe0_afcfg(&mut self) -> PE0_AFCFG_W<PCFE_SPEC, 0> {
        PE0_AFCFG_W::new(self)
    }
    #[doc = "Bits 2:3 - PE1 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe1_afcfg(&mut self) -> PE1_AFCFG_W<PCFE_SPEC, 2> {
        PE1_AFCFG_W::new(self)
    }
    #[doc = "Bits 16:17 - PE8 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe8_afcfg(&mut self) -> PE8_AFCFG_W<PCFE_SPEC, 16> {
        PE8_AFCFG_W::new(self)
    }
    #[doc = "Bits 18:19 - PE9 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe9_afcfg(&mut self) -> PE9_AFCFG_W<PCFE_SPEC, 18> {
        PE9_AFCFG_W::new(self)
    }
    #[doc = "Bit 20 - PE10 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe10_afcfg(&mut self) -> PE10_AFCFG_W<PCFE_SPEC, 20> {
        PE10_AFCFG_W::new(self)
    }
    #[doc = "Bits 22:23 - PE11 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe11_afcfg(&mut self) -> PE11_AFCFG_W<PCFE_SPEC, 22> {
        PE11_AFCFG_W::new(self)
    }
    #[doc = "Bit 24 - PE12 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe12_afcfg(&mut self) -> PE12_AFCFG_W<PCFE_SPEC, 24> {
        PE12_AFCFG_W::new(self)
    }
    #[doc = "Bit 26 - PE13 AF function configuration bitse"]
    #[inline(always)]
    #[must_use]
    pub fn pe13_afcfg(&mut self) -> PE13_AFCFG_W<PCFE_SPEC, 26> {
        PE13_AFCFG_W::new(self)
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
#[doc = "AFIO port configuration register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFE_SPEC;
impl crate::RegisterSpec for PCFE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfe::R`](R) reader structure"]
impl crate::Readable for PCFE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcfe::W`](W) writer structure"]
impl crate::Writable for PCFE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFE to value 0"]
impl crate::Resettable for PCFE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
