#[doc = "Register `VKEY` writer"]
pub type W = crate::W<VKEY_SPEC>;
#[doc = "The key of RCU_DSV register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum KEYW_AW {
    #[doc = "439041101: Allow PDVSEL and DSV to be written"]
    ENABLE = 439041101,
}
impl From<KEYW_AW> for u32 {
    #[inline(always)]
    fn from(variant: KEYW_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYW_AW {
    type Ux = u32;
}
#[doc = "Field `KEY` writer - The key of RCU_DSV register"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, KEYW_AW>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Allow PDVSEL and DSV to be written"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(KEYW_AW::ENABLE)
    }
}
impl W {
    #[doc = "Bits 0:31 - The key of RCU_DSV register"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<VKEY_SPEC, 0> {
        KEY_W::new(self)
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
#[doc = "Voltage key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VKEY_SPEC;
impl crate::RegisterSpec for VKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`vkey::W`](W) writer structure"]
impl crate::Writable for VKEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VKEY to value 0"]
impl crate::Resettable for VKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
