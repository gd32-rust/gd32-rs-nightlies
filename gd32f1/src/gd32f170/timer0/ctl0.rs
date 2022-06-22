#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKDIV_A {
    #[doc = "0: t_DTS = t_CK_INT"]
    DIV1 = 0,
    #[doc = "1: t_DTS = 2 × t_CK_INT"]
    DIV2 = 1,
    #[doc = "2: t_DTS = 4 × t_CK_INT"]
    DIV4 = 2,
}
impl From<CKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKDIV` reader - Clock division"]
pub type CKDIV_R = crate::FieldReader<u8, CKDIV_A>;
impl CKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKDIV_A> {
        match self.bits {
            0 => Some(CKDIV_A::DIV1),
            1 => Some(CKDIV_A::DIV2),
            2 => Some(CKDIV_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKDIV_A::DIV4
    }
}
#[doc = "Field `CKDIV` writer - Clock division"]
pub type CKDIV_W<'a> = crate::FieldWriter<'a, u16, CTL0_SPEC, u8, CKDIV_A, 2, 8>;
impl<'a> CKDIV_W<'a> {
    #[doc = "t_DTS = t_CK_INT"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CKDIV_A::DIV1)
    }
    #[doc = "t_DTS = 2 × t_CK_INT"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CKDIV_A::DIV2)
    }
    #[doc = "t_DTS = 4 × t_CK_INT"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CKDIV_A::DIV4)
    }
}
#[doc = "Auto-reload shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARSE_A {
    #[doc = "0: The shadow register for CAR is disabled"]
    DISABLED = 0,
    #[doc = "1: The shadow register for CAR is enabled"]
    ENABLED = 1,
}
impl From<ARSE_A> for bool {
    #[inline(always)]
    fn from(variant: ARSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARSE` reader - Auto-reload shadow enable"]
pub type ARSE_R = crate::BitReader<ARSE_A>;
impl ARSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARSE_A {
        match self.bits {
            false => ARSE_A::DISABLED,
            true => ARSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARSE_A::ENABLED
    }
}
#[doc = "Field `ARSE` writer - Auto-reload shadow enable"]
pub type ARSE_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, ARSE_A, 7>;
impl<'a> ARSE_W<'a> {
    #[doc = "The shadow register for CAR is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARSE_A::DISABLED)
    }
    #[doc = "The shadow register for CAR is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARSE_A::ENABLED)
    }
}
#[doc = "Counter aligns mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAM_A {
    #[doc = "0: The counter counts up or down depending on the direction bit"]
    EDGEALIGNED = 0,
    #[doc = "1: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    CENTERALIGNEDCOUNTINGDOWN = 1,
    #[doc = "2: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    CENTERALIGNEDCOUNTINGUP = 2,
    #[doc = "3: The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    CENTERALIGNEDCOUNTINGUPDOWN = 3,
}
impl From<CAM_A> for u8 {
    #[inline(always)]
    fn from(variant: CAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAM` reader - Counter aligns mode selection"]
pub type CAM_R = crate::FieldReader<u8, CAM_A>;
impl CAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAM_A {
        match self.bits {
            0 => CAM_A::EDGEALIGNED,
            1 => CAM_A::CENTERALIGNEDCOUNTINGDOWN,
            2 => CAM_A::CENTERALIGNEDCOUNTINGUP,
            3 => CAM_A::CENTERALIGNEDCOUNTINGUPDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGEALIGNED`"]
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == CAM_A::EDGEALIGNED
    }
    #[doc = "Checks if the value of the field is `CENTERALIGNEDCOUNTINGDOWN`"]
    #[inline(always)]
    pub fn is_center_aligned_counting_down(&self) -> bool {
        *self == CAM_A::CENTERALIGNEDCOUNTINGDOWN
    }
    #[doc = "Checks if the value of the field is `CENTERALIGNEDCOUNTINGUP`"]
    #[inline(always)]
    pub fn is_center_aligned_counting_up(&self) -> bool {
        *self == CAM_A::CENTERALIGNEDCOUNTINGUP
    }
    #[doc = "Checks if the value of the field is `CENTERALIGNEDCOUNTINGUPDOWN`"]
    #[inline(always)]
    pub fn is_center_aligned_counting_up_down(&self) -> bool {
        *self == CAM_A::CENTERALIGNEDCOUNTINGUPDOWN
    }
}
#[doc = "Field `CAM` writer - Counter aligns mode selection"]
pub type CAM_W<'a> = crate::FieldWriterSafe<'a, u16, CTL0_SPEC, u8, CAM_A, 2, 5>;
impl<'a> CAM_W<'a> {
    #[doc = "The counter counts up or down depending on the direction bit"]
    #[inline(always)]
    pub fn edge_aligned(self) -> &'a mut W {
        self.variant(CAM_A::EDGEALIGNED)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    #[inline(always)]
    pub fn center_aligned_counting_down(self) -> &'a mut W {
        self.variant(CAM_A::CENTERALIGNEDCOUNTINGDOWN)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    #[inline(always)]
    pub fn center_aligned_counting_up(self) -> &'a mut W {
        self.variant(CAM_A::CENTERALIGNEDCOUNTINGUP)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    #[inline(always)]
    pub fn center_aligned_counting_up_down(self) -> &'a mut W {
        self.variant(CAM_A::CENTERALIGNEDCOUNTINGUPDOWN)
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Counter used as upcounter"]
    UP = 0,
    #[doc = "1: Counter used as downcounter"]
    DOWN = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::UP,
            true => DIR_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DIR_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DIR_A::DOWN
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, DIR_A, 4>;
impl<'a> DIR_W<'a> {
    #[doc = "Counter used as upcounter"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(DIR_A::UP)
    }
    #[doc = "Counter used as downcounter"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(DIR_A::DOWN)
    }
}
#[doc = "Single pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPM_A {
    #[doc = "0: Counter is not stopped at update event"]
    DISABLED = 0,
    #[doc = "1: Counter stops counting at the next update event (clearing the CEN bit)"]
    ENABLED = 1,
}
impl From<SPM_A> for bool {
    #[inline(always)]
    fn from(variant: SPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPM` reader - Single pulse mode"]
pub type SPM_R = crate::BitReader<SPM_A>;
impl SPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPM_A {
        match self.bits {
            false => SPM_A::DISABLED,
            true => SPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPM_A::ENABLED
    }
}
#[doc = "Field `SPM` writer - Single pulse mode"]
pub type SPM_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, SPM_A, 3>;
impl<'a> SPM_W<'a> {
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPM_A::DISABLED)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPM_A::ENABLED)
    }
}
#[doc = "Update source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPS_A {
    #[doc = "0: Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    ANYEVENT = 0,
    #[doc = "1: Only counter overflow/underflow generates an update interrupt or DMA request"]
    COUNTERONLY = 1,
}
impl From<UPS_A> for bool {
    #[inline(always)]
    fn from(variant: UPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPS` reader - Update source"]
pub type UPS_R = crate::BitReader<UPS_A>;
impl UPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPS_A {
        match self.bits {
            false => UPS_A::ANYEVENT,
            true => UPS_A::COUNTERONLY,
        }
    }
    #[doc = "Checks if the value of the field is `ANYEVENT`"]
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == UPS_A::ANYEVENT
    }
    #[doc = "Checks if the value of the field is `COUNTERONLY`"]
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == UPS_A::COUNTERONLY
    }
}
#[doc = "Field `UPS` writer - Update source"]
pub type UPS_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, UPS_A, 2>;
impl<'a> UPS_W<'a> {
    #[doc = "Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(UPS_A::ANYEVENT)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(UPS_A::COUNTERONLY)
    }
}
#[doc = "Update disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDIS_A {
    #[doc = "0: Update event enabled"]
    ENABLED = 0,
    #[doc = "1: Update event disabled"]
    DISABLED = 1,
}
impl From<UPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: UPDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDIS` reader - Update disable"]
pub type UPDIS_R = crate::BitReader<UPDIS_A>;
impl UPDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDIS_A {
        match self.bits {
            false => UPDIS_A::ENABLED,
            true => UPDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDIS_A::DISABLED
    }
}
#[doc = "Field `UPDIS` writer - Update disable"]
pub type UPDIS_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, UPDIS_A, 1>;
impl<'a> UPDIS_W<'a> {
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPDIS_A::ENABLED)
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPDIS_A::DISABLED)
    }
}
#[doc = "Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    #[doc = "0: Counter disabled"]
    DISABLED = 0,
    #[doc = "1: Counter enabled"]
    ENABLED = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader<CEN_A>;
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLED,
            true => CEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN_A::ENABLED
    }
}
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, CEN_A, 0>;
impl<'a> CEN_W<'a> {
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::DISABLED)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&self) -> ARSE_R {
        ARSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&self) -> CAM_R {
        CAM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SPM_R {
        SPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&self) -> UPS_R {
        UPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UPDIS_R {
        UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&mut self) -> ARSE_W {
        ARSE_W::new(self)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&mut self) -> CAM_W {
        CAM_W::new(self)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&mut self) -> SPM_W {
        SPM_W::new(self)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&mut self) -> UPS_W {
        UPS_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&mut self) -> UPDIS_W {
        UPDIS_W::new(self)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
