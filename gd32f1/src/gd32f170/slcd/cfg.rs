#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `HDEN` reader - High drive enable"]
pub type HDEN_R = crate::BitReader;
#[doc = "Field `HDEN` writer - High drive enable"]
pub type HDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SOFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDIE` reader - SLCD update done interrupt enable"]
pub type UPDIE_R = crate::BitReader;
#[doc = "Field `UPDIE` writer - SLCD update done interrupt enable"]
pub type UPDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PULSE` reader - Pulse on duration"]
pub type PULSE_R = crate::FieldReader;
#[doc = "Field `PULSE` writer - Pulse on duration"]
pub type PULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DTD` reader - Dead time duration"]
pub type DTD_R = crate::FieldReader;
#[doc = "Field `DTD` writer - Dead time duration"]
pub type DTD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CONR` reader - Contrast ratio"]
pub type CONR_R = crate::FieldReader;
#[doc = "Field `CONR` writer - Contrast ratio"]
pub type CONR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BLKDIV` reader - Blink frequency divider"]
pub type BLKDIV_R = crate::FieldReader;
#[doc = "Field `BLKDIV` writer - Blink frequency divider"]
pub type BLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BLKMOD` reader - Blink mode"]
pub type BLKMOD_R = crate::FieldReader;
#[doc = "Field `BLKMOD` writer - Blink mode"]
pub type BLKMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DIV` reader - SLCD clock divider"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - SLCD clock divider"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PSC` reader - SLCD clock prescaler"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - SLCD clock prescaler"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SLCD update done interrupt enable"]
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pulse on duration"]
    #[inline(always)]
    pub fn pulse(&self) -> PULSE_R {
        PULSE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    pub fn dtd(&self) -> DTD_R {
        DTD_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Contrast ratio"]
    #[inline(always)]
    pub fn conr(&self) -> CONR_R {
        CONR_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Blink frequency divider"]
    #[inline(always)]
    pub fn blkdiv(&self) -> BLKDIV_R {
        BLKDIV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Blink mode"]
    #[inline(always)]
    pub fn blkmod(&self) -> BLKMOD_R {
        BLKMOD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - SLCD clock divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - SLCD clock prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HDEN_W<CFG_SPEC, 0> {
        HDEN_W::new(self)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<CFG_SPEC, 1> {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 3 - SLCD update done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn updie(&mut self) -> UPDIE_W<CFG_SPEC, 3> {
        UPDIE_W::new(self)
    }
    #[doc = "Bits 4:6 - Pulse on duration"]
    #[inline(always)]
    #[must_use]
    pub fn pulse(&mut self) -> PULSE_W<CFG_SPEC, 4> {
        PULSE_W::new(self)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DTD_W<CFG_SPEC, 7> {
        DTD_W::new(self)
    }
    #[doc = "Bits 10:12 - Contrast ratio"]
    #[inline(always)]
    #[must_use]
    pub fn conr(&mut self) -> CONR_W<CFG_SPEC, 10> {
        CONR_W::new(self)
    }
    #[doc = "Bits 13:15 - Blink frequency divider"]
    #[inline(always)]
    #[must_use]
    pub fn blkdiv(&mut self) -> BLKDIV_W<CFG_SPEC, 13> {
        BLKDIV_W::new(self)
    }
    #[doc = "Bits 16:17 - Blink mode"]
    #[inline(always)]
    #[must_use]
    pub fn blkmod(&mut self) -> BLKMOD_W<CFG_SPEC, 16> {
        BLKMOD_W::new(self)
    }
    #[doc = "Bits 18:21 - SLCD clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CFG_SPEC, 18> {
        DIV_W::new(self)
    }
    #[doc = "Bits 22:25 - SLCD clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<CFG_SPEC, 22> {
        PSC_W::new(self)
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
#[doc = "SLCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
