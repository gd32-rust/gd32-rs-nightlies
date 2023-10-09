#[doc = "Register `DSV` reader"]
pub type R = crate::R<DSV_SPEC>;
#[doc = "Register `DSV` writer"]
pub type W = crate::W<DSV_SPEC>;
#[doc = "Field `DSLPVS` reader - Deep-sleep mode voltage select"]
pub type DSLPVS_R = crate::FieldReader<DSLPVS_A>;
#[doc = "Deep-sleep mode voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSLPVS_A {
    #[doc = "0: The core voltage is 1.8 V in deep-sleep mode"]
    V1_8 = 0,
    #[doc = "1: The core voltage is 1.6 V in deep-sleep mode"]
    V1_6 = 1,
    #[doc = "2: The core voltage is 1.4 V in deep-sleep mode"]
    V1_4 = 2,
    #[doc = "3: The core voltage is 1.2 V in deep-sleep mode"]
    V1_2 = 3,
}
impl From<DSLPVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSLPVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSLPVS_A {
    type Ux = u8;
}
impl DSLPVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSLPVS_A> {
        match self.bits {
            0 => Some(DSLPVS_A::V1_8),
            1 => Some(DSLPVS_A::V1_6),
            2 => Some(DSLPVS_A::V1_4),
            3 => Some(DSLPVS_A::V1_2),
            _ => None,
        }
    }
    #[doc = "The core voltage is 1.8 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == DSLPVS_A::V1_8
    }
    #[doc = "The core voltage is 1.6 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v1_6(&self) -> bool {
        *self == DSLPVS_A::V1_6
    }
    #[doc = "The core voltage is 1.4 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v1_4(&self) -> bool {
        *self == DSLPVS_A::V1_4
    }
    #[doc = "The core voltage is 1.2 V in deep-sleep mode"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == DSLPVS_A::V1_2
    }
}
#[doc = "Field `DSLPVS` writer - Deep-sleep mode voltage select"]
pub type DSLPVS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, DSLPVS_A>;
impl<'a, REG, const O: u8> DSLPVS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The core voltage is 1.8 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut crate::W<REG> {
        self.variant(DSLPVS_A::V1_8)
    }
    #[doc = "The core voltage is 1.6 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_6(self) -> &'a mut crate::W<REG> {
        self.variant(DSLPVS_A::V1_6)
    }
    #[doc = "The core voltage is 1.4 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_4(self) -> &'a mut crate::W<REG> {
        self.variant(DSLPVS_A::V1_4)
    }
    #[doc = "The core voltage is 1.2 V in deep-sleep mode"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut crate::W<REG> {
        self.variant(DSLPVS_A::V1_2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&self) -> DSLPVS_R {
        DSLPVS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Deep-sleep mode voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn dslpvs(&mut self) -> DSLPVS_W<DSV_SPEC, 0> {
        DSLPVS_W::new(self)
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
#[doc = "Deep-sleep mode voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSV_SPEC;
impl crate::RegisterSpec for DSV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv::R`](R) reader structure"]
impl crate::Readable for DSV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsv::W`](W) writer structure"]
impl crate::Writable for DSV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSV to value 0"]
impl crate::Resettable for DSV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
