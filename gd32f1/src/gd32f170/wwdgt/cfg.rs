#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `WIN` reader - The Window value"]
pub type WIN_R = crate::FieldReader;
#[doc = "Field `WIN` writer - The Window value"]
pub type WIN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 7, O>;
#[doc = "Field `PSC` reader - Prescaler"]
pub type PSC_R = crate::FieldReader<PSC_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Counter clock (PCLK1 div 4096) div 1"]
    DIV1 = 0,
    #[doc = "1: Counter clock (PCLK1 div 4096) div 2"]
    DIV2 = 1,
    #[doc = "2: Counter clock (PCLK1 div 4096) div 4"]
    DIV4 = 2,
    #[doc = "3: Counter clock (PCLK1 div 4096) div 8"]
    DIV8 = 3,
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
            0 => PSC_A::DIV1,
            1 => PSC_A::DIV2,
            2 => PSC_A::DIV4,
            3 => PSC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PSC_A::DIV1
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSC_A::DIV2
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSC_A::DIV4
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSC_A::DIV8
    }
}
#[doc = "Field `PSC` writer - Prescaler"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PSC_A>;
impl<'a, REG, const O: u8> PSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV1)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV2)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV4)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV8)
    }
}
#[doc = "Field `EWIE` reader - Early wakeup interrupt enable"]
pub type EWIE_R = crate::BitReader<EWIEW_A>;
#[doc = "Early wakeup interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIEW_A {
    #[doc = "1: interrupt occurs whenever the counter reaches the value 0x40"]
    ENABLE = 1,
}
impl From<EWIEW_A> for bool {
    #[inline(always)]
    fn from(variant: EWIEW_A) -> Self {
        variant as u8 != 0
    }
}
impl EWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EWIEW_A> {
        match self.bits {
            true => Some(EWIEW_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWIEW_A::ENABLE
    }
}
#[doc = "Field `EWIE` writer - Early wakeup interrupt enable"]
pub type EWIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EWIEW_A>;
impl<'a, REG, const O: u8> EWIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EWIEW_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:6 - The Window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The Window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<CFG_SPEC, 0> {
        WIN_W::new(self)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<CFG_SPEC, 7> {
        PSC_W::new(self)
    }
    #[doc = "Bit 9 - Early wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<CFG_SPEC, 9> {
        EWIE_W::new(self)
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
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
