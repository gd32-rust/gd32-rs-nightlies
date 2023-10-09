#[doc = "Register `SCUMSK` reader"]
pub type R = crate::R<SCUMSK_SPEC>;
#[doc = "Register `SCUMSK` writer"]
pub type W = crate::W<SCUMSK_SPEC>;
#[doc = "Field `FSM` reader - Frame start code mask bits in embedded synchronous mode"]
pub type FSM_R = crate::FieldReader;
#[doc = "Field `FSM` writer - Frame start code mask bits in embedded synchronous mode"]
pub type FSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LSM` reader - Line start code mask bits in embedded synchronous mode"]
pub type LSM_R = crate::FieldReader;
#[doc = "Field `LSM` writer - Line start code mask bits in embedded synchronous mode"]
pub type LSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LEM` reader - Line end code mask bits in embedded synchronous mode"]
pub type LEM_R = crate::FieldReader;
#[doc = "Field `LEM` writer - Line end code mask bits in embedded synchronous mode"]
pub type LEM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FEM` reader - Frame end code mask bits in embedded synchronous mode"]
pub type FEM_R = crate::FieldReader;
#[doc = "Field `FEM` writer - Frame end code mask bits in embedded synchronous mode"]
pub type FEM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fsm(&self) -> FSM_R {
        FSM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lem(&self) -> LEM_R {
        LEM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fem(&self) -> FEM_R {
        FEM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn fsm(&mut self) -> FSM_W<SCUMSK_SPEC, 0> {
        FSM_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<SCUMSK_SPEC, 8> {
        LSM_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn lem(&mut self) -> LEM_W<SCUMSK_SPEC, 16> {
        LEM_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn fem(&mut self) -> FEM_W<SCUMSK_SPEC, 24> {
        FEM_W::new(self)
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
#[doc = "DCI Synchronization codes unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scumsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scumsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCUMSK_SPEC;
impl crate::RegisterSpec for SCUMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scumsk::R`](R) reader structure"]
impl crate::Readable for SCUMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scumsk::W`](W) writer structure"]
impl crate::Writable for SCUMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCUMSK to value 0"]
impl crate::Resettable for SCUMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
