#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader<CEN_A>;
#[doc = "Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLED,
            true => CEN_A::ENABLED,
        }
    }
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN_A::DISABLED
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN_A::ENABLED
    }
}
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CEN_A>;
impl<'a, REG, const O: u8> CEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN_A::DISABLED)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN_A::ENABLED)
    }
}
#[doc = "Field `UPDIS` reader - Update disable"]
pub type UPDIS_R = crate::BitReader<UPDIS_A>;
#[doc = "Update disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl UPDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDIS_A {
        match self.bits {
            false => UPDIS_A::ENABLED,
            true => UPDIS_A::DISABLED,
        }
    }
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDIS_A::ENABLED
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDIS_A::DISABLED
    }
}
#[doc = "Field `UPDIS` writer - Update disable"]
pub type UPDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UPDIS_A>;
impl<'a, REG, const O: u8> UPDIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDIS_A::ENABLED)
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDIS_A::DISABLED)
    }
}
#[doc = "Field `UPS` reader - Update source"]
pub type UPS_R = crate::BitReader<UPS_A>;
#[doc = "Update source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPS_A {
    #[doc = "0: Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    ANY_EVENT = 0,
    #[doc = "1: Only counter overflow/underflow generates an update interrupt or DMA request"]
    COUNTER_ONLY = 1,
}
impl From<UPS_A> for bool {
    #[inline(always)]
    fn from(variant: UPS_A) -> Self {
        variant as u8 != 0
    }
}
impl UPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPS_A {
        match self.bits {
            false => UPS_A::ANY_EVENT,
            true => UPS_A::COUNTER_ONLY,
        }
    }
    #[doc = "Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == UPS_A::ANY_EVENT
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == UPS_A::COUNTER_ONLY
    }
}
#[doc = "Field `UPS` writer - Update source"]
pub type UPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UPS_A>;
impl<'a, REG, const O: u8> UPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut crate::W<REG> {
        self.variant(UPS_A::ANY_EVENT)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut crate::W<REG> {
        self.variant(UPS_A::COUNTER_ONLY)
    }
}
#[doc = "Field `SPM` reader - Single pulse mode"]
pub type SPM_R = crate::BitReader<SPM_A>;
#[doc = "Single pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPM_A {
        match self.bits {
            false => SPM_A::DISABLED,
            true => SPM_A::ENABLED,
        }
    }
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPM_A::DISABLED
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPM_A::ENABLED
    }
}
#[doc = "Field `SPM` writer - Single pulse mode"]
pub type SPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPM_A>;
impl<'a, REG, const O: u8> SPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPM_A::DISABLED)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPM_A::ENABLED)
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::UP,
            true => DIR_A::DOWN,
        }
    }
    #[doc = "Counter used as upcounter"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DIR_A::UP
    }
    #[doc = "Counter used as downcounter"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DIR_A::DOWN
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DIR_A>;
impl<'a, REG, const O: u8> DIR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter used as upcounter"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::UP)
    }
    #[doc = "Counter used as downcounter"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::DOWN)
    }
}
#[doc = "Field `CAM` reader - Center-aligned mode selection"]
pub type CAM_R = crate::FieldReader<CAM_A>;
#[doc = "Center-aligned mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAM_A {
    #[doc = "0: The counter counts up or down depending on the direction bit"]
    EDGE_ALIGNED = 0,
    #[doc = "1: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    CENTER_ALIGNED_COUNTING_DOWN = 1,
    #[doc = "2: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    CENTER_ALIGNED_COUNTING_UP = 2,
    #[doc = "3: The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    CENTER_ALIGNED_COUNTING_UP_DOWN = 3,
}
impl From<CAM_A> for u8 {
    #[inline(always)]
    fn from(variant: CAM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAM_A {
    type Ux = u8;
}
impl CAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAM_A {
        match self.bits {
            0 => CAM_A::EDGE_ALIGNED,
            1 => CAM_A::CENTER_ALIGNED_COUNTING_DOWN,
            2 => CAM_A::CENTER_ALIGNED_COUNTING_UP,
            3 => CAM_A::CENTER_ALIGNED_COUNTING_UP_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "The counter counts up or down depending on the direction bit"]
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == CAM_A::EDGE_ALIGNED
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    #[inline(always)]
    pub fn is_center_aligned_counting_down(&self) -> bool {
        *self == CAM_A::CENTER_ALIGNED_COUNTING_DOWN
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    #[inline(always)]
    pub fn is_center_aligned_counting_up(&self) -> bool {
        *self == CAM_A::CENTER_ALIGNED_COUNTING_UP
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    #[inline(always)]
    pub fn is_center_aligned_counting_up_down(&self) -> bool {
        *self == CAM_A::CENTER_ALIGNED_COUNTING_UP_DOWN
    }
}
#[doc = "Field `CAM` writer - Center-aligned mode selection"]
pub type CAM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CAM_A>;
impl<'a, REG, const O: u8> CAM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The counter counts up or down depending on the direction bit"]
    #[inline(always)]
    pub fn edge_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(CAM_A::EDGE_ALIGNED)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    #[inline(always)]
    pub fn center_aligned_counting_down(self) -> &'a mut crate::W<REG> {
        self.variant(CAM_A::CENTER_ALIGNED_COUNTING_DOWN)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    #[inline(always)]
    pub fn center_aligned_counting_up(self) -> &'a mut crate::W<REG> {
        self.variant(CAM_A::CENTER_ALIGNED_COUNTING_UP)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    #[inline(always)]
    pub fn center_aligned_counting_up_down(self) -> &'a mut crate::W<REG> {
        self.variant(CAM_A::CENTER_ALIGNED_COUNTING_UP_DOWN)
    }
}
#[doc = "Field `ARSE` reader - Auto-reload shadow enable"]
pub type ARSE_R = crate::BitReader<ARSE_A>;
#[doc = "Auto-reload shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ARSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARSE_A {
        match self.bits {
            false => ARSE_A::DISABLED,
            true => ARSE_A::ENABLED,
        }
    }
    #[doc = "The shadow register for CAR is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARSE_A::DISABLED
    }
    #[doc = "The shadow register for CAR is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARSE_A::ENABLED
    }
}
#[doc = "Field `ARSE` writer - Auto-reload shadow enable"]
pub type ARSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ARSE_A>;
impl<'a, REG, const O: u8> ARSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The shadow register for CAR is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARSE_A::DISABLED)
    }
    #[doc = "The shadow register for CAR is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARSE_A::ENABLED)
    }
}
#[doc = "Field `CKDIV` reader - Clock division"]
pub type CKDIV_R = crate::FieldReader<CKDIV_A>;
#[doc = "Clock division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CKDIV_A {
    type Ux = u8;
}
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
    #[doc = "t_DTS = t_CK_INT"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKDIV_A::DIV1
    }
    #[doc = "t_DTS = 2 × t_CK_INT"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKDIV_A::DIV2
    }
    #[doc = "t_DTS = 4 × t_CK_INT"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKDIV_A::DIV4
    }
}
#[doc = "Field `CKDIV` writer - Clock division"]
pub type CKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CKDIV_A>;
impl<'a, REG, const O: u8> CKDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "t_DTS = t_CK_INT"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CKDIV_A::DIV1)
    }
    #[doc = "t_DTS = 2 × t_CK_INT"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKDIV_A::DIV2)
    }
    #[doc = "t_DTS = 4 × t_CK_INT"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKDIV_A::DIV4)
    }
}
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UPDIS_R {
        UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&self) -> UPS_R {
        UPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SPM_R {
        SPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cam(&self) -> CAM_R {
        CAM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&self) -> ARSE_R {
        ARSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CTL0_SPEC, 0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    #[must_use]
    pub fn updis(&mut self) -> UPDIS_W<CTL0_SPEC, 1> {
        UPDIS_W::new(self)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    #[must_use]
    pub fn ups(&mut self) -> UPS_W<CTL0_SPEC, 2> {
        UPS_W::new(self)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn spm(&mut self) -> SPM_W<CTL0_SPEC, 3> {
        SPM_W::new(self)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CTL0_SPEC, 4> {
        DIR_W::new(self)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn cam(&mut self) -> CAM_W<CTL0_SPEC, 5> {
        CAM_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn arse(&mut self) -> ARSE_W<CTL0_SPEC, 7> {
        ARSE_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<CTL0_SPEC, 8> {
        CKDIV_W::new(self)
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
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
