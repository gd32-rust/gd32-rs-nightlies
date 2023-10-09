#[doc = "Register `OVSAMPCTL` reader"]
pub type R = crate::R<OVSAMPCTL_SPEC>;
#[doc = "Register `OVSAMPCTL` writer"]
pub type W = crate::W<OVSAMPCTL_SPEC>;
#[doc = "Field `OVSEN` reader - Oversampling Enable"]
pub type OVSEN_R = crate::BitReader<OVSEN_A>;
#[doc = "Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVSEN_A {
    #[doc = "0: Oversampling disabled"]
    DISABLED = 0,
    #[doc = "1: Oversampling enabled"]
    ENABLED = 1,
}
impl From<OVSEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSEN_A {
        match self.bits {
            false => OVSEN_A::DISABLED,
            true => OVSEN_A::ENABLED,
        }
    }
    #[doc = "Oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVSEN_A::DISABLED
    }
    #[doc = "Oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVSEN_A::ENABLED
    }
}
#[doc = "Field `OVSEN` writer - Oversampling Enable"]
pub type OVSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OVSEN_A>;
impl<'a, REG, const O: u8> OVSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSEN_A::DISABLED)
    }
    #[doc = "Oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSEN_A::ENABLED)
    }
}
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OVSR_R = crate::FieldReader<OVSR_A>;
#[doc = "Oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: 2x"]
    TIMES2 = 0,
    #[doc = "1: 4x"]
    TIMES4 = 1,
    #[doc = "2: 8x"]
    TIMES8 = 2,
    #[doc = "3: 16x"]
    TIMES16 = 3,
    #[doc = "4: 32x"]
    TIMES32 = 4,
    #[doc = "5: 64x"]
    TIMES64 = 5,
    #[doc = "6: 128x"]
    TIMES128 = 6,
    #[doc = "7: 256x"]
    TIMES256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSR_A {
    type Ux = u8;
}
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::TIMES2,
            1 => OVSR_A::TIMES4,
            2 => OVSR_A::TIMES8,
            3 => OVSR_A::TIMES16,
            4 => OVSR_A::TIMES32,
            5 => OVSR_A::TIMES64,
            6 => OVSR_A::TIMES128,
            7 => OVSR_A::TIMES256,
            _ => unreachable!(),
        }
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_times2(&self) -> bool {
        *self == OVSR_A::TIMES2
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_times4(&self) -> bool {
        *self == OVSR_A::TIMES4
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_times8(&self) -> bool {
        *self == OVSR_A::TIMES8
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_times16(&self) -> bool {
        *self == OVSR_A::TIMES16
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn is_times32(&self) -> bool {
        *self == OVSR_A::TIMES32
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn is_times64(&self) -> bool {
        *self == OVSR_A::TIMES64
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn is_times128(&self) -> bool {
        *self == OVSR_A::TIMES128
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn is_times256(&self) -> bool {
        *self == OVSR_A::TIMES256
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio"]
pub type OVSR_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, OVSR_A>;
impl<'a, REG, const O: u8> OVSR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2x"]
    #[inline(always)]
    pub fn times2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn times4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn times8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES8)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn times16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES16)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn times32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES32)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn times64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES64)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn times128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES128)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn times256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::TIMES256)
    }
}
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OVSS_R = crate::FieldReader;
#[doc = "Field `OVSS` writer - Oversampling shift"]
pub type OVSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TOVS` reader - Triggered Oversampling"]
pub type TOVS_R = crate::BitReader<TOVS_A>;
#[doc = "Triggered Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVS_A {
    #[doc = "0: All oversampled conversions are done consecutively"]
    CONSECUTIVE = 0,
    #[doc = "1: Each oversampled conversion needs a trigger"]
    INDIVIDUAL = 1,
}
impl From<TOVS_A> for bool {
    #[inline(always)]
    fn from(variant: TOVS_A) -> Self {
        variant as u8 != 0
    }
}
impl TOVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOVS_A {
        match self.bits {
            false => TOVS_A::CONSECUTIVE,
            true => TOVS_A::INDIVIDUAL,
        }
    }
    #[doc = "All oversampled conversions are done consecutively"]
    #[inline(always)]
    pub fn is_consecutive(&self) -> bool {
        *self == TOVS_A::CONSECUTIVE
    }
    #[doc = "Each oversampled conversion needs a trigger"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        *self == TOVS_A::INDIVIDUAL
    }
}
#[doc = "Field `TOVS` writer - Triggered Oversampling"]
pub type TOVS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TOVS_A>;
impl<'a, REG, const O: u8> TOVS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions are done consecutively"]
    #[inline(always)]
    pub fn consecutive(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS_A::CONSECUTIVE)
    }
    #[doc = "Each oversampled conversion needs a trigger"]
    #[inline(always)]
    pub fn individual(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS_A::INDIVIDUAL)
    }
}
#[doc = "Field `DRES` reader - ADC resolution"]
pub type DRES_R = crate::FieldReader<DRES_A>;
#[doc = "ADC resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRES_A {
    #[doc = "0: 12 bit resolution"]
    BITS12 = 0,
    #[doc = "1: 10 bit resolution"]
    BITS10 = 1,
    #[doc = "2: 8 bit resolution"]
    BITS8 = 2,
    #[doc = "3: 6 bit resolution"]
    BITS6 = 3,
}
impl From<DRES_A> for u8 {
    #[inline(always)]
    fn from(variant: DRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRES_A {
    type Ux = u8;
}
impl DRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRES_A {
        match self.bits {
            0 => DRES_A::BITS12,
            1 => DRES_A::BITS10,
            2 => DRES_A::BITS8,
            3 => DRES_A::BITS6,
            _ => unreachable!(),
        }
    }
    #[doc = "12 bit resolution"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == DRES_A::BITS12
    }
    #[doc = "10 bit resolution"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == DRES_A::BITS10
    }
    #[doc = "8 bit resolution"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == DRES_A::BITS8
    }
    #[doc = "6 bit resolution"]
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == DRES_A::BITS6
    }
}
#[doc = "Field `DRES` writer - ADC resolution"]
pub type DRES_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DRES_A>;
impl<'a, REG, const O: u8> DRES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12 bit resolution"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::BITS12)
    }
    #[doc = "10 bit resolution"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::BITS10)
    }
    #[doc = "8 bit resolution"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::BITS8)
    }
    #[doc = "6 bit resolution"]
    #[inline(always)]
    pub fn bits6(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::BITS6)
    }
}
impl R {
    #[doc = "Bit 0 - Oversampling Enable"]
    #[inline(always)]
    pub fn ovsen(&self) -> OVSEN_R {
        OVSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - ADC resolution"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovsen(&mut self) -> OVSEN_W<OVSAMPCTL_SPEC, 0> {
        OVSEN_W::new(self)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<OVSAMPCTL_SPEC, 2> {
        OVSR_W::new(self)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<OVSAMPCTL_SPEC, 5> {
        OVSS_W::new(self)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<OVSAMPCTL_SPEC, 9> {
        TOVS_W::new(self)
    }
    #[doc = "Bits 12:13 - ADC resolution"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DRES_W<OVSAMPCTL_SPEC, 12> {
        DRES_W::new(self)
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
#[doc = "Oversample control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsampctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovsampctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVSAMPCTL_SPEC;
impl crate::RegisterSpec for OVSAMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsampctl::R`](R) reader structure"]
impl crate::Readable for OVSAMPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ovsampctl::W`](W) writer structure"]
impl crate::Writable for OVSAMPCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OVSAMPCTL to value 0"]
impl crate::Resettable for OVSAMPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
