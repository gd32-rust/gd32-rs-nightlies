#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `SADDRESS` reader - Slave address to be sent"]
pub type SADDRESS_R = crate::FieldReader<u16>;
#[doc = "Field `SADDRESS` writer - Slave address to be sent"]
pub type SADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `TRDIR` reader - Transfer direction in master mode"]
pub type TRDIR_R = crate::BitReader;
#[doc = "Field `TRDIR` writer - Transfer direction in master mode"]
pub type TRDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADD10EN` reader - 10-bit addressing mode enable in master mode"]
pub type ADD10EN_R = crate::BitReader;
#[doc = "Field `ADD10EN` writer - 10-bit addressing mode enable in master mode"]
pub type ADD10EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HEAD10R` reader - 10-bit address header executes read direction only in master receive mode"]
pub type HEAD10R_R = crate::BitReader;
#[doc = "Field `HEAD10R` writer - 10-bit address header executes read direction only in master receive mode"]
pub type HEAD10R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START` reader - Generate a START condition on I2C bus"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Generate a START condition on I2C bus"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` reader - Generate a STOP condition on I2C bus"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Generate a STOP condition on I2C bus"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NACKEN` reader - Generate NACK in slave mode"]
pub type NACKEN_R = crate::BitReader;
#[doc = "Field `NACKEN` writer - Generate NACK in slave mode"]
pub type NACKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BYTENUM` reader - Number of bytes to be transferred"]
pub type BYTENUM_R = crate::FieldReader;
#[doc = "Field `BYTENUM` writer - Number of bytes to be transferred"]
pub type BYTENUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RELOAD` reader - Reload mode"]
pub type RELOAD_R = crate::BitReader;
#[doc = "Field `RELOAD` writer - Reload mode"]
pub type RELOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOEND` reader - Automatic end mode in master mode"]
pub type AUTOEND_R = crate::BitReader;
#[doc = "Field `AUTOEND` writer - Automatic end mode in master mode"]
pub type AUTOEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECTRANS` reader - PEC Transfer"]
pub type PECTRANS_R = crate::BitReader;
#[doc = "Field `PECTRANS` writer - PEC Transfer"]
pub type PECTRANS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - Slave address to be sent"]
    #[inline(always)]
    pub fn saddress(&self) -> SADDRESS_R {
        SADDRESS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction in master mode"]
    #[inline(always)]
    pub fn trdir(&self) -> TRDIR_R {
        TRDIR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode enable in master mode"]
    #[inline(always)]
    pub fn add10en(&self) -> ADD10EN_R {
        ADD10EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header executes read direction only in master receive mode"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generate a START condition on I2C bus"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generate NACK in slave mode"]
    #[inline(always)]
    pub fn nacken(&self) -> NACKEN_R {
        NACKEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes to be transferred"]
    #[inline(always)]
    pub fn bytenum(&self) -> BYTENUM_R {
        BYTENUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reload mode"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode in master mode"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PEC Transfer"]
    #[inline(always)]
    pub fn pectrans(&self) -> PECTRANS_R {
        PECTRANS_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address to be sent"]
    #[inline(always)]
    #[must_use]
    pub fn saddress(&mut self) -> SADDRESS_W<CTL1_SPEC, 0> {
        SADDRESS_W::new(self)
    }
    #[doc = "Bit 10 - Transfer direction in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn trdir(&mut self) -> TRDIR_W<CTL1_SPEC, 10> {
        TRDIR_W::new(self)
    }
    #[doc = "Bit 11 - 10-bit addressing mode enable in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn add10en(&mut self) -> ADD10EN_W<CTL1_SPEC, 11> {
        ADD10EN_W::new(self)
    }
    #[doc = "Bit 12 - 10-bit address header executes read direction only in master receive mode"]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<CTL1_SPEC, 12> {
        HEAD10R_W::new(self)
    }
    #[doc = "Bit 13 - Generate a START condition on I2C bus"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTL1_SPEC, 13> {
        START_W::new(self)
    }
    #[doc = "Bit 14 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CTL1_SPEC, 14> {
        STOP_W::new(self)
    }
    #[doc = "Bit 15 - Generate NACK in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn nacken(&mut self) -> NACKEN_W<CTL1_SPEC, 15> {
        NACKEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of bytes to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn bytenum(&mut self) -> BYTENUM_W<CTL1_SPEC, 16> {
        BYTENUM_W::new(self)
    }
    #[doc = "Bit 24 - Reload mode"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<CTL1_SPEC, 24> {
        RELOAD_W::new(self)
    }
    #[doc = "Bit 25 - Automatic end mode in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<CTL1_SPEC, 25> {
        AUTOEND_W::new(self)
    }
    #[doc = "Bit 26 - PEC Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pectrans(&mut self) -> PECTRANS_W<CTL1_SPEC, 26> {
        PECTRANS_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
