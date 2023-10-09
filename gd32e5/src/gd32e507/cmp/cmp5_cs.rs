#[doc = "Register `CMP5_CS` reader"]
pub type R = crate::R<CMP5_CS_SPEC>;
#[doc = "Register `CMP5_CS` writer"]
pub type W = crate::W<CMP5_CS_SPEC>;
#[doc = "Field `CMP5EN` reader - Comparator 5 enable"]
pub type CMP5EN_R = crate::BitReader;
#[doc = "Field `CMP5EN` writer - Comparator 5 enable"]
pub type CMP5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP5MSEL` reader - Comparator 5 input selection"]
pub type CMP5MSEL_R = crate::FieldReader;
#[doc = "Field `CMP5MSEL` writer - Comparator 5 input selection"]
pub type CMP5MSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP5OSEL` reader - Comparator 5 output selection"]
pub type CMP5OSEL_R = crate::FieldReader;
#[doc = "Field `CMP5OSEL` writer - Comparator 5 output selection"]
pub type CMP5OSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CMP5PL` reader - Polarity of comparator 5 output"]
pub type CMP5PL_R = crate::BitReader;
#[doc = "Field `CMP5PL` writer - Polarity of comparator 5 output"]
pub type CMP5PL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP5BLK` reader - CMP5 output blanking source"]
pub type CMP5BLK_R = crate::FieldReader;
#[doc = "Field `CMP5BLK` writer - CMP5 output blanking source"]
pub type CMP5BLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP5MSEL_3` reader - CMP5_IM input selection"]
pub type CMP5MSEL_3_R = crate::BitReader;
#[doc = "Field `CMP5MSEL_3` writer - CMP5_IM input selection"]
pub type CMP5MSEL_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP5O` reader - CMP5 output"]
pub type CMP5O_R = crate::BitReader;
#[doc = "Field `CMP5LK` reader - Comparator 5 lock"]
pub type CMP5LK_R = crate::BitReader;
#[doc = "Field `CMP5LK` writer - Comparator 5 lock"]
pub type CMP5LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    pub fn cmp5en(&self) -> CMP5EN_R {
        CMP5EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 5 input selection"]
    #[inline(always)]
    pub fn cmp5msel(&self) -> CMP5MSEL_R {
        CMP5MSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    pub fn cmp5osel(&self) -> CMP5OSEL_R {
        CMP5OSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Polarity of comparator 5 output"]
    #[inline(always)]
    pub fn cmp5pl(&self) -> CMP5PL_R {
        CMP5PL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - CMP5 output blanking source"]
    #[inline(always)]
    pub fn cmp5blk(&self) -> CMP5BLK_R {
        CMP5BLK_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - CMP5_IM input selection"]
    #[inline(always)]
    pub fn cmp5msel_3(&self) -> CMP5MSEL_3_R {
        CMP5MSEL_3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - CMP5 output"]
    #[inline(always)]
    pub fn cmp5o(&self) -> CMP5O_R {
        CMP5O_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    pub fn cmp5lk(&self) -> CMP5LK_R {
        CMP5LK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5en(&mut self) -> CMP5EN_W<CMP5_CS_SPEC, 0> {
        CMP5EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 5 input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5msel(&mut self) -> CMP5MSEL_W<CMP5_CS_SPEC, 4> {
        CMP5MSEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5osel(&mut self) -> CMP5OSEL_W<CMP5_CS_SPEC, 10> {
        CMP5OSEL_W::new(self)
    }
    #[doc = "Bit 15 - Polarity of comparator 5 output"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5pl(&mut self) -> CMP5PL_W<CMP5_CS_SPEC, 15> {
        CMP5PL_W::new(self)
    }
    #[doc = "Bits 18:20 - CMP5 output blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5blk(&mut self) -> CMP5BLK_W<CMP5_CS_SPEC, 18> {
        CMP5BLK_W::new(self)
    }
    #[doc = "Bit 22 - CMP5_IM input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5msel_3(&mut self) -> CMP5MSEL_3_W<CMP5_CS_SPEC, 22> {
        CMP5MSEL_3_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5lk(&mut self) -> CMP5LK_W<CMP5_CS_SPEC, 31> {
        CMP5LK_W::new(self)
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
#[doc = "CMP5 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp5_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp5_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP5_CS_SPEC;
impl crate::RegisterSpec for CMP5_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp5_cs::R`](R) reader structure"]
impl crate::Readable for CMP5_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp5_cs::W`](W) writer structure"]
impl crate::Writable for CMP5_CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP5_CS to value 0"]
impl crate::Resettable for CMP5_CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
