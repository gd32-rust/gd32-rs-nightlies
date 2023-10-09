#[doc = "Register `SADDR0` reader"]
pub type R = crate::R<SADDR0_SPEC>;
#[doc = "Register `SADDR0` writer"]
pub type W = crate::W<SADDR0_SPEC>;
#[doc = "Field `ADDRESS` reader - Interface address"]
pub type ADDRESS_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS` writer - Interface address"]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 10, O, u16>;
#[doc = "Field `ADDFORMAT` reader - Addressing mode (slave mode)"]
pub type ADDFORMAT_R = crate::BitReader<ADDFORMAT_A>;
#[doc = "Addressing mode (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDFORMAT_A {
    #[doc = "0: 7-bit slave address (note that you'll need to shift the address by 1b)"]
    ADD7 = 0,
    #[doc = "1: 10-bit slave address"]
    ADD10 = 1,
}
impl From<ADDFORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: ADDFORMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDFORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDFORMAT_A {
        match self.bits {
            false => ADDFORMAT_A::ADD7,
            true => ADDFORMAT_A::ADD10,
        }
    }
    #[doc = "7-bit slave address (note that you'll need to shift the address by 1b)"]
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == ADDFORMAT_A::ADD7
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == ADDFORMAT_A::ADD10
    }
}
#[doc = "Field `ADDFORMAT` writer - Addressing mode (slave mode)"]
pub type ADDFORMAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADDFORMAT_A>;
impl<'a, REG, const O: u8> ADDFORMAT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit slave address (note that you'll need to shift the address by 1b)"]
    #[inline(always)]
    pub fn add7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDFORMAT_A::ADD7)
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn add10(self) -> &'a mut crate::W<REG> {
        self.variant(ADDFORMAT_A::ADD10)
    }
}
impl R {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addformat(&self) -> ADDFORMAT_R {
        ADDFORMAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<SADDR0_SPEC, 0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn addformat(&mut self) -> ADDFORMAT_W<SADDR0_SPEC, 15> {
        ADDFORMAT_W::new(self)
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
#[doc = "Own address register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDR0_SPEC;
impl crate::RegisterSpec for SADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr0::R`](R) reader structure"]
impl crate::Readable for SADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddr0::W`](W) writer structure"]
impl crate::Writable for SADDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDR0 to value 0"]
impl crate::Resettable for SADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
