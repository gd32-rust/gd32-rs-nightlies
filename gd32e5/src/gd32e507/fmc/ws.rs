#[doc = "Register `WS` reader"]
pub type R = crate::R<WS_SPEC>;
#[doc = "Register `WS` writer"]
pub type W = crate::W<WS_SPEC>;
#[doc = "Field `WSCNT` reader - wait state counter register"]
pub type WSCNT_R = crate::FieldReader<WSCNT_A>;
#[doc = "wait state counter register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WSCNT_A {
    #[doc = "0: 0 wait states added"]
    WS0 = 0,
    #[doc = "1: 1 wait state added"]
    WS1 = 1,
    #[doc = "2: 2 wait states added"]
    WS2 = 2,
}
impl From<WSCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: WSCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WSCNT_A {
    type Ux = u8;
}
impl WSCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSCNT_A> {
        match self.bits {
            0 => Some(WSCNT_A::WS0),
            1 => Some(WSCNT_A::WS1),
            2 => Some(WSCNT_A::WS2),
            _ => None,
        }
    }
    #[doc = "0 wait states added"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == WSCNT_A::WS0
    }
    #[doc = "1 wait state added"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == WSCNT_A::WS1
    }
    #[doc = "2 wait states added"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == WSCNT_A::WS2
    }
}
#[doc = "Field `WSCNT` writer - wait state counter register"]
pub type WSCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WSCNT_A>;
impl<'a, REG, const O: u8> WSCNT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 wait states added"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(WSCNT_A::WS0)
    }
    #[doc = "1 wait state added"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(WSCNT_A::WS1)
    }
    #[doc = "2 wait states added"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(WSCNT_A::WS2)
    }
}
#[doc = "Field `PFEN` reader - Pre-fetch enable"]
pub type PFEN_R = crate::BitReader;
#[doc = "Field `PFEN` writer - Pre-fetch enable"]
pub type PFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICEN` reader - IBUS cache enable"]
pub type ICEN_R = crate::BitReader;
#[doc = "Field `ICEN` writer - IBUS cache enable"]
pub type ICEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCEN` reader - DBUS cache enable"]
pub type DCEN_R = crate::BitReader;
#[doc = "Field `DCEN` writer - DBUS cache enable"]
pub type DCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICRST` reader - IBUS cache reset"]
pub type ICRST_R = crate::BitReader;
#[doc = "Field `ICRST` writer - IBUS cache reset"]
pub type ICRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCRST` reader - DBUS cache reset"]
pub type DCRST_R = crate::BitReader;
#[doc = "Field `DCRST` writer - DBUS cache reset"]
pub type DCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WSCNT_R {
        WSCNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PFEN_R {
        PFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - IBUS cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBUS cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IBUS cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBUS cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    #[must_use]
    pub fn wscnt(&mut self) -> WSCNT_W<WS_SPEC, 0> {
        WSCNT_W::new(self)
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfen(&mut self) -> PFEN_W<WS_SPEC, 4> {
        PFEN_W::new(self)
    }
    #[doc = "Bit 9 - IBUS cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<WS_SPEC, 9> {
        ICEN_W::new(self)
    }
    #[doc = "Bit 10 - DBUS cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DCEN_W<WS_SPEC, 10> {
        DCEN_W::new(self)
    }
    #[doc = "Bit 11 - IBUS cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<WS_SPEC, 11> {
        ICRST_W::new(self)
    }
    #[doc = "Bit 12 - DBUS cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcrst(&mut self) -> DCRST_W<WS_SPEC, 12> {
        DCRST_W::new(self)
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
#[doc = "wait state counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WS_SPEC;
impl crate::RegisterSpec for WS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ws::R`](R) reader structure"]
impl crate::Readable for WS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ws::W`](W) writer structure"]
impl crate::Writable for WS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WS to value 0x0630"]
impl crate::Resettable for WS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0630;
}
