#[doc = "Register `CHCTL0_Input` reader"]
pub type R = crate::R<CHCTL0_INPUT_SPEC>;
#[doc = "Register `CHCTL0_Input` writer"]
pub type W = crate::W<CHCTL0_INPUT_SPEC>;
#[doc = "Field `CH0MS` reader - Channel 0 mode selection"]
pub type CH0MS_R = crate::FieldReader<CH0MS_A>;
#[doc = "Channel 0 mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH0MS_A {
    #[doc = "0: Channel is configured as output"]
    OUTPUT = 0,
    #[doc = "1: Channel is configured as input, ISx is connected to CI0FEx"]
    CI0 = 1,
    #[doc = "2: Channel is configured as input, ISx is connected to CI1FEx"]
    CI1 = 2,
    #[doc = "3: Channel is configured as input, ISx is connected to ITS"]
    ITS = 3,
}
impl From<CH0MS_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0MS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH0MS_A {
    type Ux = u8;
}
impl CH0MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0MS_A {
        match self.bits {
            0 => CH0MS_A::OUTPUT,
            1 => CH0MS_A::CI0,
            2 => CH0MS_A::CI1,
            3 => CH0MS_A::ITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CH0MS_A::OUTPUT
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FEx"]
    #[inline(always)]
    pub fn is_ci0(&self) -> bool {
        *self == CH0MS_A::CI0
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FEx"]
    #[inline(always)]
    pub fn is_ci1(&self) -> bool {
        *self == CH0MS_A::CI1
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn is_its(&self) -> bool {
        *self == CH0MS_A::ITS
    }
}
#[doc = "Field `CH0MS` writer - Channel 0 mode selection"]
pub type CH0MS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CH0MS_A>;
impl<'a, REG, const O: u8> CH0MS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::OUTPUT)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI0FEx"]
    #[inline(always)]
    pub fn ci0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::CI0)
    }
    #[doc = "Channel is configured as input, ISx is connected to CI1FEx"]
    #[inline(always)]
    pub fn ci1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::CI1)
    }
    #[doc = "Channel is configured as input, ISx is connected to ITS"]
    #[inline(always)]
    pub fn its(self) -> &'a mut crate::W<REG> {
        self.variant(CH0MS_A::ITS)
    }
}
#[doc = "Field `CH0CAPPSC` reader - Channel 0 input capture prescaler"]
pub type CH0CAPPSC_R = crate::FieldReader<CH0CAPPSC_A>;
#[doc = "Channel 0 input capture prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH0CAPPSC_A {
    #[doc = "0: Prescaler disabled, capture on every edge"]
    DIV1 = 0,
    #[doc = "1: Capture every 2 edges"]
    DIV2 = 1,
    #[doc = "2: Capture every 4 edges"]
    DIV4 = 2,
    #[doc = "3: Capture every 8 edges"]
    DIV8 = 3,
}
impl From<CH0CAPPSC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0CAPPSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH0CAPPSC_A {
    type Ux = u8;
}
impl CH0CAPPSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0CAPPSC_A {
        match self.bits {
            0 => CH0CAPPSC_A::DIV1,
            1 => CH0CAPPSC_A::DIV2,
            2 => CH0CAPPSC_A::DIV4,
            3 => CH0CAPPSC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler disabled, capture on every edge"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CH0CAPPSC_A::DIV1
    }
    #[doc = "Capture every 2 edges"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CH0CAPPSC_A::DIV2
    }
    #[doc = "Capture every 4 edges"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CH0CAPPSC_A::DIV4
    }
    #[doc = "Capture every 8 edges"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CH0CAPPSC_A::DIV8
    }
}
#[doc = "Field `CH0CAPPSC` writer - Channel 0 input capture prescaler"]
pub type CH0CAPPSC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CH0CAPPSC_A>;
impl<'a, REG, const O: u8> CH0CAPPSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler disabled, capture on every edge"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPPSC_A::DIV1)
    }
    #[doc = "Capture every 2 edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPPSC_A::DIV2)
    }
    #[doc = "Capture every 4 edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPPSC_A::DIV4)
    }
    #[doc = "Capture every 8 edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPPSC_A::DIV8)
    }
}
#[doc = "Field `CH0CAPFLT` reader - Channel 0 input capture filter control"]
pub type CH0CAPFLT_R = crate::FieldReader<CH0CAPFLT_A>;
#[doc = "Channel 0 input capture filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH0CAPFLT_A {
    #[doc = "0: Filter disabled. fSAMP=fDTS, N=1"]
    NO_FILTER = 0,
    #[doc = "1: fSAMP=fTIMER_CK, N=2"]
    TIMER_CK_N2 = 1,
    #[doc = "2: fSAMP=fTIMER_CK, N=4"]
    TIMER_CK_N4 = 2,
    #[doc = "3: fSAMP=fTIMER_CK, N=8"]
    TIMER_CK_N8 = 3,
    #[doc = "4: fSAMP=fDTS/2, N=6"]
    FDTS_DIV2_N6 = 4,
    #[doc = "5: fSAMP=fDTS/2, N=8"]
    FDTS_DIV2_N8 = 5,
    #[doc = "6: fSAMP=fDTS/4, N=6"]
    FDTS_DIV4_N6 = 6,
    #[doc = "7: fSAMP=fDTS/4, N=8"]
    FDTS_DIV4_N8 = 7,
    #[doc = "8: fSAMP=fDTS/8, N=6"]
    FDTS_DIV8_N6 = 8,
    #[doc = "9: fSAMP=fDTS/8, N=8"]
    FDTS_DIV8_N8 = 9,
    #[doc = "10: fSAMP=fDTS/16, N=5"]
    FDTS_DIV16_N5 = 10,
    #[doc = "11: fSAMP=fDTS/16, N=6"]
    FDTS_DIV16_N6 = 11,
    #[doc = "12: fSAMP=fDTS/16, N=8"]
    FDTS_DIV16_N8 = 12,
    #[doc = "13: fSAMP=fDTS/32, N=5"]
    FDTS_DIV32_N5 = 13,
    #[doc = "14: fSAMP=fDTS/32, N=6"]
    FDTS_DIV32_N6 = 14,
    #[doc = "15: fSAMP=fDTS/32, N=8"]
    FDTS_DIV32_N8 = 15,
}
impl From<CH0CAPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0CAPFLT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH0CAPFLT_A {
    type Ux = u8;
}
impl CH0CAPFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0CAPFLT_A {
        match self.bits {
            0 => CH0CAPFLT_A::NO_FILTER,
            1 => CH0CAPFLT_A::TIMER_CK_N2,
            2 => CH0CAPFLT_A::TIMER_CK_N4,
            3 => CH0CAPFLT_A::TIMER_CK_N8,
            4 => CH0CAPFLT_A::FDTS_DIV2_N6,
            5 => CH0CAPFLT_A::FDTS_DIV2_N8,
            6 => CH0CAPFLT_A::FDTS_DIV4_N6,
            7 => CH0CAPFLT_A::FDTS_DIV4_N8,
            8 => CH0CAPFLT_A::FDTS_DIV8_N6,
            9 => CH0CAPFLT_A::FDTS_DIV8_N8,
            10 => CH0CAPFLT_A::FDTS_DIV16_N5,
            11 => CH0CAPFLT_A::FDTS_DIV16_N6,
            12 => CH0CAPFLT_A::FDTS_DIV16_N8,
            13 => CH0CAPFLT_A::FDTS_DIV32_N5,
            14 => CH0CAPFLT_A::FDTS_DIV32_N6,
            15 => CH0CAPFLT_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == CH0CAPFLT_A::NO_FILTER
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn is_timer_ck_n2(&self) -> bool {
        *self == CH0CAPFLT_A::TIMER_CK_N2
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn is_timer_ck_n4(&self) -> bool {
        *self == CH0CAPFLT_A::TIMER_CK_N4
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn is_timer_ck_n8(&self) -> bool {
        *self == CH0CAPFLT_A::TIMER_CK_N8
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV2_N6
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV2_N8
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV4_N6
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV4_N8
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV8_N6
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV8_N8
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV16_N5
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV16_N6
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV16_N8
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV32_N5
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV32_N6
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == CH0CAPFLT_A::FDTS_DIV32_N8
    }
}
#[doc = "Field `CH0CAPFLT` writer - Channel 0 input capture filter control"]
pub type CH0CAPFLT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, CH0CAPFLT_A>;
impl<'a, REG, const O: u8> CH0CAPFLT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter disabled. fSAMP=fDTS, N=1"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::NO_FILTER)
    }
    #[doc = "fSAMP=fTIMER_CK, N=2"]
    #[inline(always)]
    pub fn timer_ck_n2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::TIMER_CK_N2)
    }
    #[doc = "fSAMP=fTIMER_CK, N=4"]
    #[inline(always)]
    pub fn timer_ck_n4(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::TIMER_CK_N4)
    }
    #[doc = "fSAMP=fTIMER_CK, N=8"]
    #[inline(always)]
    pub fn timer_ck_n8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::TIMER_CK_N8)
    }
    #[doc = "fSAMP=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMP=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMP=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMP=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMP=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMP=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMP=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMP=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMP=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMP=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMP=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMP=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(CH0CAPFLT_A::FDTS_DIV32_N8)
    }
}
#[doc = "Field `CH1CAPFLT` reader - Channel 1 input capture filter control"]
pub use CH0CAPFLT_R as CH1CAPFLT_R;
#[doc = "Field `CH1CAPFLT` writer - Channel 1 input capture filter control"]
pub use CH0CAPFLT_W as CH1CAPFLT_W;
#[doc = "Field `CH1CAPPSC` reader - Channel 1 input capture prescaler"]
pub use CH0CAPPSC_R as CH1CAPPSC_R;
#[doc = "Field `CH1CAPPSC` writer - Channel 1 input capture prescaler"]
pub use CH0CAPPSC_W as CH1CAPPSC_W;
#[doc = "Field `CH1MS` reader - Channel 1 mode selection"]
pub use CH0MS_R as CH1MS_R;
#[doc = "Field `CH1MS` writer - Channel 1 mode selection"]
pub use CH0MS_W as CH1MS_W;
impl R {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> CH0CAPPSC_R {
        CH0CAPPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> CH0CAPFLT_R {
        CH0CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> CH1MS_R {
        CH1MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&self) -> CH1CAPPSC_R {
        CH1CAPPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    pub fn ch1capflt(&self) -> CH1CAPFLT_R {
        CH1CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ms(&mut self) -> CH0MS_W<CHCTL0_INPUT_SPEC, 0> {
        CH0MS_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cappsc(&mut self) -> CH0CAPPSC_W<CHCTL0_INPUT_SPEC, 2> {
        CH0CAPPSC_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch0capflt(&mut self) -> CH0CAPFLT_W<CHCTL0_INPUT_SPEC, 4> {
        CH0CAPFLT_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ms(&mut self) -> CH1MS_W<CHCTL0_INPUT_SPEC, 8> {
        CH1MS_W::new(self)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cappsc(&mut self) -> CH1CAPPSC_W<CHCTL0_INPUT_SPEC, 10> {
        CH1CAPPSC_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch1capflt(&mut self) -> CH1CAPFLT_W<CHCTL0_INPUT_SPEC, 12> {
        CH1CAPFLT_W::new(self)
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
#[doc = "Channel control register 0 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL0_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl0_input::R`](R) reader structure"]
impl crate::Readable for CHCTL0_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl0_input::W`](W) writer structure"]
impl crate::Writable for CHCTL0_INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL0_Input to value 0"]
impl crate::Resettable for CHCTL0_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
