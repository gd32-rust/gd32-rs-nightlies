#[doc = "Register `VKEY` reader"]
pub type R = crate::R<VKEY_SPEC>;
#[doc = "Register `VKEY` writer"]
pub type W = crate::W<VKEY_SPEC>;
#[doc = "Field `KEY` reader - The key of RCU_PDVSEL and RCU_DSV register"]
pub type KEY_R = crate::FieldReader<KEYW_A>;
#[doc = "The key of RCU_PDVSEL and RCU_DSV register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum KEYW_A {
    #[doc = "439041101: Allow PDVSEL and DSV to be written"]
    ENABLE = 439041101,
}
impl From<KEYW_A> for u32 {
    #[inline(always)]
    fn from(variant: KEYW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYW_A {
    type Ux = u32;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYW_A> {
        match self.bits {
            439041101 => Some(KEYW_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Allow PDVSEL and DSV to be written"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == KEYW_A::ENABLE
    }
}
#[doc = "Field `KEY` writer - The key of RCU_PDVSEL and RCU_DSV register"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, KEYW_A>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Allow PDVSEL and DSV to be written"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(KEYW_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:31 - The key of RCU_PDVSEL and RCU_DSV register"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The key of RCU_PDVSEL and RCU_DSV register"]
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
#[doc = "Voltage key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vkey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vkey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VKEY_SPEC;
impl crate::RegisterSpec for VKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vkey::R`](R) reader structure"]
impl crate::Readable for VKEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vkey::W`](W) writer structure"]
impl crate::Writable for VKEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VKEY to value 0"]
impl crate::Resettable for VKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
