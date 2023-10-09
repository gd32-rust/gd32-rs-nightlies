#[doc = "Register `CMP3_CS` reader"]
pub type R = crate::R<CMP3_CS_SPEC>;
#[doc = "Register `CMP3_CS` writer"]
pub type W = crate::W<CMP3_CS_SPEC>;
#[doc = "Field `CMP3EN` reader - Comparator 3 enable"]
pub type CMP3EN_R = crate::BitReader;
#[doc = "Field `CMP3EN` writer - Comparator 3 enable"]
pub type CMP3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3MSEL` reader - Comparator 3 input selection"]
pub type CMP3MSEL_R = crate::FieldReader;
#[doc = "Field `CMP3MSEL` writer - Comparator 3 input selection"]
pub type CMP3MSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP3OSEL` reader - Comparator 3 output selection"]
pub type CMP3OSEL_R = crate::FieldReader;
#[doc = "Field `CMP3OSEL` writer - Comparator 3 output selection"]
pub type CMP3OSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CMP3PL` reader - Polarity of comparator 3 output"]
pub type CMP3PL_R = crate::BitReader;
#[doc = "Field `CMP3PL` writer - Polarity of comparator 3 output"]
pub type CMP3PL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3BLK` reader - CMP3 output blanking source"]
pub type CMP3BLK_R = crate::FieldReader;
#[doc = "Field `CMP3BLK` writer - CMP3 output blanking source"]
pub type CMP3BLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP3MSEL_3` reader - CMP3_IM input selection"]
pub type CMP3MSEL_3_R = crate::BitReader;
#[doc = "Field `CMP3MSEL_3` writer - CMP3_IM input selection"]
pub type CMP3MSEL_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3O` reader - CMP3 output"]
pub type CMP3O_R = crate::BitReader;
#[doc = "Field `CMP3LK` reader - Comparator 3 lock"]
pub type CMP3LK_R = crate::BitReader;
#[doc = "Field `CMP3LK` writer - Comparator 3 lock"]
pub type CMP3LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    pub fn cmp3en(&self) -> CMP3EN_R {
        CMP3EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 3 input selection"]
    #[inline(always)]
    pub fn cmp3msel(&self) -> CMP3MSEL_R {
        CMP3MSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    pub fn cmp3osel(&self) -> CMP3OSEL_R {
        CMP3OSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Polarity of comparator 3 output"]
    #[inline(always)]
    pub fn cmp3pl(&self) -> CMP3PL_R {
        CMP3PL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - CMP3 output blanking source"]
    #[inline(always)]
    pub fn cmp3blk(&self) -> CMP3BLK_R {
        CMP3BLK_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - CMP3_IM input selection"]
    #[inline(always)]
    pub fn cmp3msel_3(&self) -> CMP3MSEL_3_R {
        CMP3MSEL_3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - CMP3 output"]
    #[inline(always)]
    pub fn cmp3o(&self) -> CMP3O_R {
        CMP3O_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    pub fn cmp3lk(&self) -> CMP3LK_R {
        CMP3LK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3en(&mut self) -> CMP3EN_W<CMP3_CS_SPEC, 0> {
        CMP3EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 3 input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3msel(&mut self) -> CMP3MSEL_W<CMP3_CS_SPEC, 4> {
        CMP3MSEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3osel(&mut self) -> CMP3OSEL_W<CMP3_CS_SPEC, 10> {
        CMP3OSEL_W::new(self)
    }
    #[doc = "Bit 15 - Polarity of comparator 3 output"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3pl(&mut self) -> CMP3PL_W<CMP3_CS_SPEC, 15> {
        CMP3PL_W::new(self)
    }
    #[doc = "Bits 18:20 - CMP3 output blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3blk(&mut self) -> CMP3BLK_W<CMP3_CS_SPEC, 18> {
        CMP3BLK_W::new(self)
    }
    #[doc = "Bit 22 - CMP3_IM input selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3msel_3(&mut self) -> CMP3MSEL_3_W<CMP3_CS_SPEC, 22> {
        CMP3MSEL_3_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3lk(&mut self) -> CMP3LK_W<CMP3_CS_SPEC, 31> {
        CMP3LK_W::new(self)
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
#[doc = "CMP3 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP3_CS_SPEC;
impl crate::RegisterSpec for CMP3_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp3_cs::R`](R) reader structure"]
impl crate::Readable for CMP3_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp3_cs::W`](W) writer structure"]
impl crate::Writable for CMP3_CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP3_CS to value 0"]
impl crate::Resettable for CMP3_CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
