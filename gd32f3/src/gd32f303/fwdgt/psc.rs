#[doc = "Register `PSC` reader"]
pub type R = crate::R<PSC_SPEC>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PSC_SPEC>;
#[doc = "Field `PSC` reader - Free watchdog timer prescaler selection"]
pub type PSC_R = crate::FieldReader<PSC_A>;
#[doc = "Free watchdog timer prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Divider /4"]
    DIVIDE_BY4 = 0,
    #[doc = "1: Divider /8"]
    DIVIDE_BY8 = 1,
    #[doc = "2: Divider /16"]
    DIVIDE_BY16 = 2,
    #[doc = "3: Divider /32"]
    DIVIDE_BY32 = 3,
    #[doc = "4: Divider /64"]
    DIVIDE_BY64 = 4,
    #[doc = "5: Divider /128"]
    DIVIDE_BY128 = 5,
    #[doc = "6: Divider /256"]
    DIVIDE_BY256 = 6,
    #[doc = "7: Divider /256"]
    DIVIDE_BY256BIS = 7,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSC_A {
    type Ux = u8;
}
impl PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            0 => PSC_A::DIVIDE_BY4,
            1 => PSC_A::DIVIDE_BY8,
            2 => PSC_A::DIVIDE_BY16,
            3 => PSC_A::DIVIDE_BY32,
            4 => PSC_A::DIVIDE_BY64,
            5 => PSC_A::DIVIDE_BY128,
            6 => PSC_A::DIVIDE_BY256,
            7 => PSC_A::DIVIDE_BY256BIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Divider /4"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == PSC_A::DIVIDE_BY4
    }
    #[doc = "Divider /8"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == PSC_A::DIVIDE_BY8
    }
    #[doc = "Divider /16"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == PSC_A::DIVIDE_BY16
    }
    #[doc = "Divider /32"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == PSC_A::DIVIDE_BY32
    }
    #[doc = "Divider /64"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == PSC_A::DIVIDE_BY64
    }
    #[doc = "Divider /128"]
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == PSC_A::DIVIDE_BY128
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn is_divide_by256(&self) -> bool {
        *self == PSC_A::DIVIDE_BY256
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn is_divide_by256bis(&self) -> bool {
        *self == PSC_A::DIVIDE_BY256BIS
    }
}
#[doc = "Field `PSC` writer - Free watchdog timer prescaler selection"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, PSC_A>;
impl<'a, REG, const O: u8> PSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divider /4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY4)
    }
    #[doc = "Divider /8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY8)
    }
    #[doc = "Divider /16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY16)
    }
    #[doc = "Divider /32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY32)
    }
    #[doc = "Divider /64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY64)
    }
    #[doc = "Divider /128"]
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY128)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY256)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256bis(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIVIDE_BY256BIS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Free watchdog timer prescaler selection"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Free watchdog timer prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSC_SPEC, 0> {
        PSC_W::new(self)
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
#[doc = "Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
