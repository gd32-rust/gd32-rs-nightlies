#[doc = "Register `SCUMSK` reader"]
pub struct R(crate::R<SCUMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCUMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCUMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCUMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCUMSK` writer"]
pub struct W(crate::W<SCUMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCUMSK_SPEC>;
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
impl From<crate::W<SCUMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCUMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEM` reader - Frame end code mask bits in embedded synchronous mode"]
pub type FEM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEM` writer - Frame end code mask bits in embedded synchronous mode"]
pub type FEM_W<'a> = crate::FieldWriter<'a, u32, SCUMSK_SPEC, u8, u8, 8, 24>;
#[doc = "Field `LEM` reader - Line end code mask bits in embedded synchronous mode"]
pub type LEM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEM` writer - Line end code mask bits in embedded synchronous mode"]
pub type LEM_W<'a> = crate::FieldWriter<'a, u32, SCUMSK_SPEC, u8, u8, 8, 16>;
#[doc = "Field `LSM` reader - Line start code mask bits in embedded synchronous mode"]
pub type LSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSM` writer - Line start code mask bits in embedded synchronous mode"]
pub type LSM_W<'a> = crate::FieldWriter<'a, u32, SCUMSK_SPEC, u8, u8, 8, 8>;
#[doc = "Field `FSM` reader - Frame start code mask bits in embedded synchronous mode"]
pub type FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSM` writer - Frame start code mask bits in embedded synchronous mode"]
pub type FSM_W<'a> = crate::FieldWriter<'a, u32, SCUMSK_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - Frame end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fem(&self) -> FEM_R {
        FEM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lem(&self) -> LEM_R {
        LEM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Frame start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fsm(&self) -> FSM_R {
        FSM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Frame end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fem(&mut self) -> FEM_W {
        FEM_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lem(&mut self) -> LEM_W {
        LEM_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lsm(&mut self) -> LSM_W {
        LSM_W::new(self)
    }
    #[doc = "Bits 0:7 - Frame start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fsm(&mut self) -> FSM_W {
        FSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCI Synchronization codes unmask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scumsk](index.html) module"]
pub struct SCUMSK_SPEC;
impl crate::RegisterSpec for SCUMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scumsk::R](R) reader structure"]
impl crate::Readable for SCUMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scumsk::W](W) writer structure"]
impl crate::Writable for SCUMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCUMSK to value 0"]
impl crate::Resettable for SCUMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
