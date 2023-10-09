#[doc = "Register `CMP1_CS` reader"]
pub type R = crate::R<CMP1_CS_SPEC>;
#[doc = "Register `CMP1_CS` writer"]
pub type W = crate::W<CMP1_CS_SPEC>;
#[doc = "Field `CMP1EN` reader - Comparator 1 enable"]
pub type CMP1EN_R = crate::BitReader;
#[doc = "Field `CMP1EN` writer - Comparator 1 enable"]
pub type CMP1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1MSEL` reader - Comparator 1 input selection"]
pub type CMP1MSEL_R = crate::FieldReader;
#[doc = "Field `CMP1MSEL` writer - Comparator 1 input selection"]
pub type CMP1MSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP1OSEL` reader - Comparator 1 output selection"]
pub type CMP1OSEL_R = crate::FieldReader;
#[doc = "Field `CMP1OSEL` writer - Comparator 1 output selection"]
pub type CMP1OSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CMP1PL` reader - Polarity of comparator 1 output"]
pub type CMP1PL_R = crate::BitReader;
#[doc = "Field `CMP1PL` writer - Polarity of comparator 1 output"]
pub type CMP1PL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1BLK` reader - CMP1 output blanking source"]
pub type CMP1BLK_R = crate::FieldReader;
#[doc = "Field `CMP1BLK` writer - CMP1 output blanking source"]
pub type CMP1BLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP1MSEL_3` reader - CMP1_IM input selection"]
pub type CMP1MSEL_3_R = crate::BitReader;
#[doc = "Field `CMP1MSEL_3` writer - CMP1_IM input selection"]
pub type CMP1MSEL_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1O` reader - CMP1 output"]
pub type CMP1O_R = crate::BitReader;
#[doc = "Field `CMP1LK` reader - Comparator 1 lock"]
pub type CMP1LK_R = crate::BitReader;
#[doc = "Field `CMP1LK` writer - Comparator 1 lock"]
pub type CMP1LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 1 input selection"]
    #[inline(always)]
    pub fn cmp1msel(&self) -> CMP1MSEL_R {
        CMP1MSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn cmp1osel(&self) -> CMP1OSEL_R {
        CMP1OSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Polarity of comparator 1 output"]
    #[inline(always)]
    pub fn cmp1pl(&self) -> CMP1PL_R {
        CMP1PL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - CMP1 output blanking source"]
    #[inline(always)]
    pub fn cmp1blk(&self) -> CMP1BLK_R {
        CMP1BLK_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - CMP1_IM input selection"]
    #[inline(always)]
    pub fn cmp1msel_3(&self) -> CMP1MSEL_3_R {
        CMP1MSEL_3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - CMP1 output"]
    #[inline(always)]
    pub fn cmp1o(&self) -> CMP1O_R {
        CMP1O_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    pub fn cmp1lk(&self) -> CMP1LK_R {
        CMP1LK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> CMP1EN_W<CMP1_CS_SPEC, 0> {
        CMP1EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 1 input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1msel(&mut self) -> CMP1MSEL_W<CMP1_CS_SPEC, 4> {
        CMP1MSEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1osel(&mut self) -> CMP1OSEL_W<CMP1_CS_SPEC, 10> {
        CMP1OSEL_W::new(self)
    }
    #[doc = "Bit 15 - Polarity of comparator 1 output"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1pl(&mut self) -> CMP1PL_W<CMP1_CS_SPEC, 15> {
        CMP1PL_W::new(self)
    }
    #[doc = "Bits 18:20 - CMP1 output blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1blk(&mut self) -> CMP1BLK_W<CMP1_CS_SPEC, 18> {
        CMP1BLK_W::new(self)
    }
    #[doc = "Bit 22 - CMP1_IM input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1msel_3(&mut self) -> CMP1MSEL_3_W<CMP1_CS_SPEC, 22> {
        CMP1MSEL_3_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1lk(&mut self) -> CMP1LK_W<CMP1_CS_SPEC, 31> {
        CMP1LK_W::new(self)
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
#[doc = "CMP1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP1_CS_SPEC;
impl crate::RegisterSpec for CMP1_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1_cs::R`](R) reader structure"]
impl crate::Readable for CMP1_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp1_cs::W`](W) writer structure"]
impl crate::Writable for CMP1_CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP1_CS to value 0"]
impl crate::Resettable for CMP1_CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
