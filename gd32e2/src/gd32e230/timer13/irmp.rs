#[doc = "Register `IRMP` reader"]
pub type R = crate::R<IRMP_SPEC>;
#[doc = "Register `IRMP` writer"]
pub type W = crate::W<IRMP_SPEC>;
#[doc = "Field `CI0_RMP` reader - Timer input 0 remap"]
pub type CI0_RMP_R = crate::FieldReader<CI0_RMP_A>;
#[doc = "Timer input 0 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CI0_RMP_A {
    #[doc = "0: Channel 0 input is connected to GPIO"]
    GPIO = 0,
    #[doc = "1: Channel 0 input is connected to RTCCLK"]
    RTCCLK = 1,
    #[doc = "2: Channel 0 input is connected to HXTAL / 32"]
    HXTAL_32 = 2,
    #[doc = "3: Channel 0 input is connected to CKOUTSEL"]
    CKOUTSEL = 3,
}
impl From<CI0_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: CI0_RMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CI0_RMP_A {
    type Ux = u8;
}
impl CI0_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CI0_RMP_A {
        match self.bits {
            0 => CI0_RMP_A::GPIO,
            1 => CI0_RMP_A::RTCCLK,
            2 => CI0_RMP_A::HXTAL_32,
            3 => CI0_RMP_A::CKOUTSEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 input is connected to GPIO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == CI0_RMP_A::GPIO
    }
    #[doc = "Channel 0 input is connected to RTCCLK"]
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        *self == CI0_RMP_A::RTCCLK
    }
    #[doc = "Channel 0 input is connected to HXTAL / 32"]
    #[inline(always)]
    pub fn is_hxtal_32(&self) -> bool {
        *self == CI0_RMP_A::HXTAL_32
    }
    #[doc = "Channel 0 input is connected to CKOUTSEL"]
    #[inline(always)]
    pub fn is_ckoutsel(&self) -> bool {
        *self == CI0_RMP_A::CKOUTSEL
    }
}
#[doc = "Field `CI0_RMP` writer - Timer input 0 remap"]
pub type CI0_RMP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CI0_RMP_A>;
impl<'a, REG, const O: u8> CI0_RMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 input is connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(CI0_RMP_A::GPIO)
    }
    #[doc = "Channel 0 input is connected to RTCCLK"]
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CI0_RMP_A::RTCCLK)
    }
    #[doc = "Channel 0 input is connected to HXTAL / 32"]
    #[inline(always)]
    pub fn hxtal_32(self) -> &'a mut crate::W<REG> {
        self.variant(CI0_RMP_A::HXTAL_32)
    }
    #[doc = "Channel 0 input is connected to CKOUTSEL"]
    #[inline(always)]
    pub fn ckoutsel(self) -> &'a mut crate::W<REG> {
        self.variant(CI0_RMP_A::CKOUTSEL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer input 0 remap"]
    #[inline(always)]
    pub fn ci0_rmp(&self) -> CI0_RMP_R {
        CI0_RMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer input 0 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ci0_rmp(&mut self) -> CI0_RMP_W<IRMP_SPEC, 0> {
        CI0_RMP_W::new(self)
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
#[doc = "channel input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRMP_SPEC;
impl crate::RegisterSpec for IRMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irmp::R`](R) reader structure"]
impl crate::Readable for IRMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irmp::W`](W) writer structure"]
impl crate::Writable for IRMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRMP to value 0"]
impl crate::Resettable for IRMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
