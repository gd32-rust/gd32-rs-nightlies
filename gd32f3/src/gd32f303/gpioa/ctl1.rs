#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `MD8` reader - Port x mode bits (x = 8)"]
pub type MD8_R = crate::FieldReader<MD8_A>;
#[doc = "Port x mode bits (x = 8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD8_A {
    #[doc = "0: Input mode (reset state)"]
    INPUT = 0,
    #[doc = "1: Output mode ,max speed 10MHz"]
    SPEED10M = 1,
    #[doc = "2: Output mode ,max speed 2MHz"]
    SPEED2M = 2,
    #[doc = "3: Output mode ,max speed 50MHz"]
    SPEED50M = 3,
}
impl From<MD8_A> for u8 {
    #[inline(always)]
    fn from(variant: MD8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MD8_A {
    type Ux = u8;
}
impl MD8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD8_A {
        match self.bits {
            0 => MD8_A::INPUT,
            1 => MD8_A::SPEED10M,
            2 => MD8_A::SPEED2M,
            3 => MD8_A::SPEED50M,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MD8_A::INPUT
    }
    #[doc = "Output mode ,max speed 10MHz"]
    #[inline(always)]
    pub fn is_speed10m(&self) -> bool {
        *self == MD8_A::SPEED10M
    }
    #[doc = "Output mode ,max speed 2MHz"]
    #[inline(always)]
    pub fn is_speed2m(&self) -> bool {
        *self == MD8_A::SPEED2M
    }
    #[doc = "Output mode ,max speed 50MHz"]
    #[inline(always)]
    pub fn is_speed50m(&self) -> bool {
        *self == MD8_A::SPEED50M
    }
}
#[doc = "Field `MD8` writer - Port x mode bits (x = 8)"]
pub type MD8_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MD8_A>;
impl<'a, REG, const O: u8> MD8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MD8_A::INPUT)
    }
    #[doc = "Output mode ,max speed 10MHz"]
    #[inline(always)]
    pub fn speed10m(self) -> &'a mut crate::W<REG> {
        self.variant(MD8_A::SPEED10M)
    }
    #[doc = "Output mode ,max speed 2MHz"]
    #[inline(always)]
    pub fn speed2m(self) -> &'a mut crate::W<REG> {
        self.variant(MD8_A::SPEED2M)
    }
    #[doc = "Output mode ,max speed 50MHz"]
    #[inline(always)]
    pub fn speed50m(self) -> &'a mut crate::W<REG> {
        self.variant(MD8_A::SPEED50M)
    }
}
#[doc = "Field `CTL8` reader - Port x configuration bits (x = 8)"]
pub type CTL8_R = crate::FieldReader<CTL8_A>;
#[doc = "Port x configuration bits (x = 8)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTL8_A {
    #[doc = "0: Analog mode/GPIO output with push-pull"]
    ANALOG_OR_PUSH_PULL = 0,
    #[doc = "1: Floating input/GPIO output with open-drain"]
    FLOATING_OR_OPEN_DRAIN = 1,
    #[doc = "2: Input with pull-up pull-down/AFIO output with push-pull"]
    INPUT_OR_AFIO_PP = 2,
    #[doc = "3: Reserved/AFIO output with open-drain"]
    RSVDOR_AFIO_OD = 3,
}
impl From<CTL8_A> for u8 {
    #[inline(always)]
    fn from(variant: CTL8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTL8_A {
    type Ux = u8;
}
impl CTL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL8_A {
        match self.bits {
            0 => CTL8_A::ANALOG_OR_PUSH_PULL,
            1 => CTL8_A::FLOATING_OR_OPEN_DRAIN,
            2 => CTL8_A::INPUT_OR_AFIO_PP,
            3 => CTL8_A::RSVDOR_AFIO_OD,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog mode/GPIO output with push-pull"]
    #[inline(always)]
    pub fn is_analog_or_push_pull(&self) -> bool {
        *self == CTL8_A::ANALOG_OR_PUSH_PULL
    }
    #[doc = "Floating input/GPIO output with open-drain"]
    #[inline(always)]
    pub fn is_floating_or_open_drain(&self) -> bool {
        *self == CTL8_A::FLOATING_OR_OPEN_DRAIN
    }
    #[doc = "Input with pull-up pull-down/AFIO output with push-pull"]
    #[inline(always)]
    pub fn is_input_or_afio_pp(&self) -> bool {
        *self == CTL8_A::INPUT_OR_AFIO_PP
    }
    #[doc = "Reserved/AFIO output with open-drain"]
    #[inline(always)]
    pub fn is_rsvdor_afio_od(&self) -> bool {
        *self == CTL8_A::RSVDOR_AFIO_OD
    }
}
#[doc = "Field `CTL8` writer - Port x configuration bits (x = 8)"]
pub type CTL8_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CTL8_A>;
impl<'a, REG, const O: u8> CTL8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog mode/GPIO output with push-pull"]
    #[inline(always)]
    pub fn analog_or_push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(CTL8_A::ANALOG_OR_PUSH_PULL)
    }
    #[doc = "Floating input/GPIO output with open-drain"]
    #[inline(always)]
    pub fn floating_or_open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(CTL8_A::FLOATING_OR_OPEN_DRAIN)
    }
    #[doc = "Input with pull-up pull-down/AFIO output with push-pull"]
    #[inline(always)]
    pub fn input_or_afio_pp(self) -> &'a mut crate::W<REG> {
        self.variant(CTL8_A::INPUT_OR_AFIO_PP)
    }
    #[doc = "Reserved/AFIO output with open-drain"]
    #[inline(always)]
    pub fn rsvdor_afio_od(self) -> &'a mut crate::W<REG> {
        self.variant(CTL8_A::RSVDOR_AFIO_OD)
    }
}
#[doc = "Field `CTL9` reader - Port x configuration bits (x = 9)"]
pub use CTL8_R as CTL9_R;
#[doc = "Field `CTL10` reader - Port x configuration bits (x = 10)"]
pub use CTL8_R as CTL10_R;
#[doc = "Field `CTL11` reader - Port x configuration bits (x = 11)"]
pub use CTL8_R as CTL11_R;
#[doc = "Field `CTL12` reader - Port x configuration bits (x = 12)"]
pub use CTL8_R as CTL12_R;
#[doc = "Field `CTL13` reader - Port x configuration bits (x = 13)"]
pub use CTL8_R as CTL13_R;
#[doc = "Field `CTL14` reader - Port x configuration bits (x = 14)"]
pub use CTL8_R as CTL14_R;
#[doc = "Field `CTL15` reader - Port x configuration bits (x = 15)"]
pub use CTL8_R as CTL15_R;
#[doc = "Field `CTL9` writer - Port x configuration bits (x = 9)"]
pub use CTL8_W as CTL9_W;
#[doc = "Field `CTL10` writer - Port x configuration bits (x = 10)"]
pub use CTL8_W as CTL10_W;
#[doc = "Field `CTL11` writer - Port x configuration bits (x = 11)"]
pub use CTL8_W as CTL11_W;
#[doc = "Field `CTL12` writer - Port x configuration bits (x = 12)"]
pub use CTL8_W as CTL12_W;
#[doc = "Field `CTL13` writer - Port x configuration bits (x = 13)"]
pub use CTL8_W as CTL13_W;
#[doc = "Field `CTL14` writer - Port x configuration bits (x = 14)"]
pub use CTL8_W as CTL14_W;
#[doc = "Field `CTL15` writer - Port x configuration bits (x = 15)"]
pub use CTL8_W as CTL15_W;
#[doc = "Field `MD9` reader - Port x mode bits (x = 9)"]
pub use MD8_R as MD9_R;
#[doc = "Field `MD10` reader - Port x mode bits (x = 10 )"]
pub use MD8_R as MD10_R;
#[doc = "Field `MD11` reader - Port x mode bits (x = 11 )"]
pub use MD8_R as MD11_R;
#[doc = "Field `MD12` reader - Port x mode bits (x = 12)"]
pub use MD8_R as MD12_R;
#[doc = "Field `MD13` reader - Port x mode bits (x = 13)"]
pub use MD8_R as MD13_R;
#[doc = "Field `MD14` reader - Port x mode bits (x = 14)"]
pub use MD8_R as MD14_R;
#[doc = "Field `MD15` reader - Port x mode bits (x = 15)"]
pub use MD8_R as MD15_R;
#[doc = "Field `MD9` writer - Port x mode bits (x = 9)"]
pub use MD8_W as MD9_W;
#[doc = "Field `MD10` writer - Port x mode bits (x = 10 )"]
pub use MD8_W as MD10_W;
#[doc = "Field `MD11` writer - Port x mode bits (x = 11 )"]
pub use MD8_W as MD11_W;
#[doc = "Field `MD12` writer - Port x mode bits (x = 12)"]
pub use MD8_W as MD12_W;
#[doc = "Field `MD13` writer - Port x mode bits (x = 13)"]
pub use MD8_W as MD13_W;
#[doc = "Field `MD14` writer - Port x mode bits (x = 14)"]
pub use MD8_W as MD14_W;
#[doc = "Field `MD15` writer - Port x mode bits (x = 15)"]
pub use MD8_W as MD15_W;
impl R {
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    pub fn md8(&self) -> MD8_R {
        MD8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&self) -> CTL8_R {
        CTL8_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    pub fn md9(&self) -> MD9_R {
        MD9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&self) -> CTL9_R {
        CTL9_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    pub fn md10(&self) -> MD10_R {
        MD10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&self) -> CTL10_R {
        CTL10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    pub fn md11(&self) -> MD11_R {
        MD11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&self) -> CTL11_R {
        CTL11_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    pub fn md12(&self) -> MD12_R {
        MD12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&self) -> CTL12_R {
        CTL12_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    pub fn md13(&self) -> MD13_R {
        MD13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&self) -> CTL13_R {
        CTL13_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    pub fn md14(&self) -> MD14_R {
        MD14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&self) -> CTL14_R {
        CTL14_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    pub fn md15(&self) -> MD15_R {
        MD15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&self) -> CTL15_R {
        CTL15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn md8(&mut self) -> MD8_W<CTL1_SPEC, 0> {
        MD8_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl8(&mut self) -> CTL8_W<CTL1_SPEC, 2> {
        CTL8_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn md9(&mut self) -> MD9_W<CTL1_SPEC, 4> {
        MD9_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl9(&mut self) -> CTL9_W<CTL1_SPEC, 6> {
        CTL9_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    #[must_use]
    pub fn md10(&mut self) -> MD10_W<CTL1_SPEC, 8> {
        MD10_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl10(&mut self) -> CTL10_W<CTL1_SPEC, 10> {
        CTL10_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    #[must_use]
    pub fn md11(&mut self) -> MD11_W<CTL1_SPEC, 12> {
        MD11_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl11(&mut self) -> CTL11_W<CTL1_SPEC, 14> {
        CTL11_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn md12(&mut self) -> MD12_W<CTL1_SPEC, 16> {
        MD12_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl12(&mut self) -> CTL12_W<CTL1_SPEC, 18> {
        CTL12_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn md13(&mut self) -> MD13_W<CTL1_SPEC, 20> {
        MD13_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl13(&mut self) -> CTL13_W<CTL1_SPEC, 22> {
        CTL13_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn md14(&mut self) -> MD14_W<CTL1_SPEC, 24> {
        MD14_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl14(&mut self) -> CTL14_W<CTL1_SPEC, 26> {
        CTL14_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn md15(&mut self) -> MD15_W<CTL1_SPEC, 28> {
        MD15_W::new(self)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl15(&mut self) -> CTL15_W<CTL1_SPEC, 30> {
        CTL15_W::new(self)
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
#[doc = "port control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x4444_4444"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
