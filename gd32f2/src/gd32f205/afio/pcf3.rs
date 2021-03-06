#[doc = "Register `PCF3` reader"]
pub struct R(crate::R<PCF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCF3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCF3` writer"]
pub struct W(crate::W<PCF3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCF3_SPEC>;
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
impl From<crate::W<PCF3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCF3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLI_B3_PG11_REMAP` reader - TLI_B3_PG11 remapping"]
pub type TLI_B3_PG11_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B3_PG11_REMAP` writer - TLI_B3_PG11 remapping"]
pub type TLI_B3_PG11_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 31>;
#[doc = "Field `TLI_B2_PG10_REMAP` reader - TLI_B2_PG10 remapping"]
pub type TLI_B2_PG10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B2_PG10_REMAP` writer - TLI_B2_PG10 remapping"]
pub type TLI_B2_PG10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 30>;
#[doc = "Field `TLI_G3_PG10_REMAP` reader - TLI_G3_PG10 remapping"]
pub type TLI_G3_PG10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G3_PG10_REMAP` writer - TLI_G3_PG10 remapping"]
pub type TLI_G3_PG10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 29>;
#[doc = "Field `TLI_CLK_PG7_REMAP` reader - TLI_CLK_PG7 remapping"]
pub type TLI_CLK_PG7_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_CLK_PG7_REMAP` writer - TLI_CLK_PG7 remapping"]
pub type TLI_CLK_PG7_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 28>;
#[doc = "Field `TLI_R7_PG6_REMAP` reader - TLI_R7_PG6 remapping"]
pub type TLI_R7_PG6_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R7_PG6_REMAP` writer - TLI_R7_PG6 remapping"]
pub type TLI_R7_PG6_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 27>;
#[doc = "Field `TLI_DE_PF10_REMAP` reader - TLI_DE_PF10 remapping"]
pub type TLI_DE_PF10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_DE_PF10_REMAP` writer - TLI_DE_PF10 remapping"]
pub type TLI_DE_PF10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 26>;
#[doc = "Field `TLI_R7_PE15_REMAP` reader - TLI_R7_PE15 remapping"]
pub type TLI_R7_PE15_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R7_PE15_REMAP` writer - TLI_R7_PE15 remapping"]
pub type TLI_R7_PE15_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 25>;
#[doc = "Field `TLI_CLK_PE14_REMAP` reader - TLI_CLK_PE14 remapping"]
pub type TLI_CLK_PE14_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_CLK_PE14_REMAP` writer - TLI_CLK_PE14 remapping"]
pub type TLI_CLK_PE14_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 24>;
#[doc = "Field `TLI_DE_PE13_REMAP` reader - TLI_DE_PE13 remapping"]
pub type TLI_DE_PE13_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_DE_PE13_REMAP` writer - TLI_DE_PE13 remapping"]
pub type TLI_DE_PE13_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 23>;
#[doc = "Field `TLI_B4_PE12_REMAP` reader - TLI_B4_PE12 remapping"]
pub type TLI_B4_PE12_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B4_PE12_REMAP` writer - TLI_B4_PE12 remapping"]
pub type TLI_B4_PE12_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 22>;
#[doc = "Field `TLI_G3_PE11_REMAP` reader - TLI_G3_PE11 remapping"]
pub type TLI_G3_PE11_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G3_PE11_REMAP` writer - TLI_G3_PE11 remapping"]
pub type TLI_G3_PE11_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 21>;
#[doc = "Field `TLI_G1_PE6_REMAP` reader - TLI_G1_PE6 remapping"]
pub type TLI_G1_PE6_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G1_PE6_REMAP` writer - TLI_G1_PE6 remapping"]
pub type TLI_G1_PE6_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 20>;
#[doc = "Field `TLI_G0_PE5_REMAP` reader - TLI_G0_PE5 remapping"]
pub type TLI_G0_PE5_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G0_PE5_REMAP` writer - TLI_G0_PE5 remapping"]
pub type TLI_G0_PE5_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 19>;
#[doc = "Field `TLI_B0_PE4_REMAP` reader - TLI_B0_PE4 remapping"]
pub type TLI_B0_PE4_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B0_PE4_REMAP` writer - TLI_B0_PE4 remapping"]
pub type TLI_B0_PE4_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 18>;
#[doc = "Field `TLI_B3_PD10_REMAP` reader - TLI_B3_PD10 remapping"]
pub type TLI_B3_PD10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B3_PD10_REMAP` writer - TLI_B3_PD10 remapping"]
pub type TLI_B3_PD10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 17>;
#[doc = "Field `TLI_B2_PD6_REMAP` reader - TLI_B2_PD6 remapping"]
pub type TLI_B2_PD6_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B2_PD6_REMAP` writer - TLI_B2_PD6 remapping"]
pub type TLI_B2_PD6_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 16>;
#[doc = "Field `TLI_G7_PD3_REMAP` reader - TLI_G7_PD3 remapping"]
pub type TLI_G7_PD3_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G7_PD3_REMAP` writer - TLI_G7_PD3 remapping"]
pub type TLI_G7_PD3_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 15>;
#[doc = "Field `TLI_R2_PC10_REMAP` reader - TLI_R2_PC10 remapping"]
pub type TLI_R2_PC10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R2_PC10_REMAP` writer - TLI_R2_PC10 remapping"]
pub type TLI_R2_PC10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 14>;
#[doc = "Field `TLI_G6_PC7_REMAP` reader - TLI_G6_PC7 remapping"]
pub type TLI_G6_PC7_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G6_PC7_REMAP` writer - TLI_G6_PC7 remapping"]
pub type TLI_G6_PC7_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 13>;
#[doc = "Field `TLI_HSYNC_PC6_REMAP` reader - TLI_HSYNC_PC6 remapping"]
pub type TLI_HSYNC_PC6_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_HSYNC_PC6_REMAP` writer - TLI_HSYNC_PC6 remapping"]
pub type TLI_HSYNC_PC6_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 12>;
#[doc = "Field `TLI_G5_PB11_REMAP` reader - TLI_G5_PB11 remapping"]
pub type TLI_G5_PB11_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G5_PB11_REMAP` writer - TLI_G5_PB11 remapping"]
pub type TLI_G5_PB11_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 11>;
#[doc = "Field `TLI_G4_PB10_REMAP` reader - TLI_G4_PB10 remapping"]
pub type TLI_G4_PB10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G4_PB10_REMAP` writer - TLI_G4_PB10 remapping"]
pub type TLI_G4_PB10_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 10>;
#[doc = "Field `TLI_B7_PB9_REMAP` reader - TLI_B7_PB9 remapping"]
pub type TLI_B7_PB9_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B7_PB9_REMAP` writer - TLI_B7_PB9 remapping"]
pub type TLI_B7_PB9_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 9>;
#[doc = "Field `TLI_B6_PB8_REMAP` reader - TLI_B6_PB8 remapping"]
pub type TLI_B6_PB8_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B6_PB8_REMAP` writer - TLI_B6_PB8 remapping"]
pub type TLI_B6_PB8_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 8>;
#[doc = "Field `TLI_R6_PB1_REMAP` reader - TLI_R6_PB1 remapping"]
pub type TLI_R6_PB1_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R6_PB1_REMAP` writer - TLI_R6_PB1 remapping"]
pub type TLI_R6_PB1_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 7>;
#[doc = "Field `TLI_R3_PB0_REMAP` reader - TLI_R3_PB0 remapping"]
pub type TLI_R3_PB0_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R3_PB0_REMAP` writer - TLI_R3_PB0 remapping"]
pub type TLI_R3_PB0_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 6>;
#[doc = "Field `TLI_R5_PA12_REMAP` reader - TLI_R5_PA12 remapping"]
pub type TLI_R5_PA12_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R5_PA12_REMAP` writer - TLI_R5_PA12 remapping"]
pub type TLI_R5_PA12_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 5>;
#[doc = "Field `TLI_R4_PA11_REMAP` reader - TLI_R4_PA11 remapping"]
pub type TLI_R4_PA11_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R4_PA11_REMAP` writer - TLI_R4_PA11 remapping"]
pub type TLI_R4_PA11_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 4>;
#[doc = "Field `TLI_R6_PA8_REMAP` reader - TLI_R6_PA8 remapping"]
pub type TLI_R6_PA8_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_R6_PA8_REMAP` writer - TLI_R6_PA8 remapping"]
pub type TLI_R6_PA8_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 3>;
#[doc = "Field `TLI_G2_PA6_REMAP` reader - TLI_G2_PA6 remapping"]
pub type TLI_G2_PA6_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_G2_PA6_REMAP` writer - TLI_G2_PA6 remapping"]
pub type TLI_G2_PA6_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 2>;
#[doc = "Field `TLI_VSYNC_PA4_REMAP` reader - TLI_VSYNC_PA4 remapping"]
pub type TLI_VSYNC_PA4_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_VSYNC_PA4_REMAP` writer - TLI_VSYNC_PA4 remapping"]
pub type TLI_VSYNC_PA4_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 1>;
#[doc = "Field `TLI_B5_PA3_REMAP` reader - TLI_B5_PA3 remapping"]
pub type TLI_B5_PA3_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TLI_B5_PA3_REMAP` writer - TLI_B5_PA3 remapping"]
pub type TLI_B5_PA3_REMAP_W<'a> = crate::BitWriter<'a, u32, PCF3_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 31 - TLI_B3_PG11 remapping"]
    #[inline(always)]
    pub fn tli_b3_pg11_remap(&self) -> TLI_B3_PG11_REMAP_R {
        TLI_B3_PG11_REMAP_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - TLI_B2_PG10 remapping"]
    #[inline(always)]
    pub fn tli_b2_pg10_remap(&self) -> TLI_B2_PG10_REMAP_R {
        TLI_B2_PG10_REMAP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - TLI_G3_PG10 remapping"]
    #[inline(always)]
    pub fn tli_g3_pg10_remap(&self) -> TLI_G3_PG10_REMAP_R {
        TLI_G3_PG10_REMAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - TLI_CLK_PG7 remapping"]
    #[inline(always)]
    pub fn tli_clk_pg7_remap(&self) -> TLI_CLK_PG7_REMAP_R {
        TLI_CLK_PG7_REMAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - TLI_R7_PG6 remapping"]
    #[inline(always)]
    pub fn tli_r7_pg6_remap(&self) -> TLI_R7_PG6_REMAP_R {
        TLI_R7_PG6_REMAP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI_DE_PF10 remapping"]
    #[inline(always)]
    pub fn tli_de_pf10_remap(&self) -> TLI_DE_PF10_REMAP_R {
        TLI_DE_PF10_REMAP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - TLI_R7_PE15 remapping"]
    #[inline(always)]
    pub fn tli_r7_pe15_remap(&self) -> TLI_R7_PE15_REMAP_R {
        TLI_R7_PE15_REMAP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - TLI_CLK_PE14 remapping"]
    #[inline(always)]
    pub fn tli_clk_pe14_remap(&self) -> TLI_CLK_PE14_REMAP_R {
        TLI_CLK_PE14_REMAP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - TLI_DE_PE13 remapping"]
    #[inline(always)]
    pub fn tli_de_pe13_remap(&self) -> TLI_DE_PE13_REMAP_R {
        TLI_DE_PE13_REMAP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - TLI_B4_PE12 remapping"]
    #[inline(always)]
    pub fn tli_b4_pe12_remap(&self) -> TLI_B4_PE12_REMAP_R {
        TLI_B4_PE12_REMAP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - TLI_G3_PE11 remapping"]
    #[inline(always)]
    pub fn tli_g3_pe11_remap(&self) -> TLI_G3_PE11_REMAP_R {
        TLI_G3_PE11_REMAP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - TLI_G1_PE6 remapping"]
    #[inline(always)]
    pub fn tli_g1_pe6_remap(&self) -> TLI_G1_PE6_REMAP_R {
        TLI_G1_PE6_REMAP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - TLI_G0_PE5 remapping"]
    #[inline(always)]
    pub fn tli_g0_pe5_remap(&self) -> TLI_G0_PE5_REMAP_R {
        TLI_G0_PE5_REMAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - TLI_B0_PE4 remapping"]
    #[inline(always)]
    pub fn tli_b0_pe4_remap(&self) -> TLI_B0_PE4_REMAP_R {
        TLI_B0_PE4_REMAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - TLI_B3_PD10 remapping"]
    #[inline(always)]
    pub fn tli_b3_pd10_remap(&self) -> TLI_B3_PD10_REMAP_R {
        TLI_B3_PD10_REMAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TLI_B2_PD6 remapping"]
    #[inline(always)]
    pub fn tli_b2_pd6_remap(&self) -> TLI_B2_PD6_REMAP_R {
        TLI_B2_PD6_REMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - TLI_G7_PD3 remapping"]
    #[inline(always)]
    pub fn tli_g7_pd3_remap(&self) -> TLI_G7_PD3_REMAP_R {
        TLI_G7_PD3_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - TLI_R2_PC10 remapping"]
    #[inline(always)]
    pub fn tli_r2_pc10_remap(&self) -> TLI_R2_PC10_REMAP_R {
        TLI_R2_PC10_REMAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - TLI_G6_PC7 remapping"]
    #[inline(always)]
    pub fn tli_g6_pc7_remap(&self) -> TLI_G6_PC7_REMAP_R {
        TLI_G6_PC7_REMAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - TLI_HSYNC_PC6 remapping"]
    #[inline(always)]
    pub fn tli_hsync_pc6_remap(&self) -> TLI_HSYNC_PC6_REMAP_R {
        TLI_HSYNC_PC6_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - TLI_G5_PB11 remapping"]
    #[inline(always)]
    pub fn tli_g5_pb11_remap(&self) -> TLI_G5_PB11_REMAP_R {
        TLI_G5_PB11_REMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - TLI_G4_PB10 remapping"]
    #[inline(always)]
    pub fn tli_g4_pb10_remap(&self) -> TLI_G4_PB10_REMAP_R {
        TLI_G4_PB10_REMAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - TLI_B7_PB9 remapping"]
    #[inline(always)]
    pub fn tli_b7_pb9_remap(&self) -> TLI_B7_PB9_REMAP_R {
        TLI_B7_PB9_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - TLI_B6_PB8 remapping"]
    #[inline(always)]
    pub fn tli_b6_pb8_remap(&self) -> TLI_B6_PB8_REMAP_R {
        TLI_B6_PB8_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - TLI_R6_PB1 remapping"]
    #[inline(always)]
    pub fn tli_r6_pb1_remap(&self) -> TLI_R6_PB1_REMAP_R {
        TLI_R6_PB1_REMAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - TLI_R3_PB0 remapping"]
    #[inline(always)]
    pub fn tli_r3_pb0_remap(&self) -> TLI_R3_PB0_REMAP_R {
        TLI_R3_PB0_REMAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - TLI_R5_PA12 remapping"]
    #[inline(always)]
    pub fn tli_r5_pa12_remap(&self) -> TLI_R5_PA12_REMAP_R {
        TLI_R5_PA12_REMAP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - TLI_R4_PA11 remapping"]
    #[inline(always)]
    pub fn tli_r4_pa11_remap(&self) -> TLI_R4_PA11_REMAP_R {
        TLI_R4_PA11_REMAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - TLI_R6_PA8 remapping"]
    #[inline(always)]
    pub fn tli_r6_pa8_remap(&self) -> TLI_R6_PA8_REMAP_R {
        TLI_R6_PA8_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - TLI_G2_PA6 remapping"]
    #[inline(always)]
    pub fn tli_g2_pa6_remap(&self) -> TLI_G2_PA6_REMAP_R {
        TLI_G2_PA6_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - TLI_VSYNC_PA4 remapping"]
    #[inline(always)]
    pub fn tli_vsync_pa4_remap(&self) -> TLI_VSYNC_PA4_REMAP_R {
        TLI_VSYNC_PA4_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TLI_B5_PA3 remapping"]
    #[inline(always)]
    pub fn tli_b5_pa3_remap(&self) -> TLI_B5_PA3_REMAP_R {
        TLI_B5_PA3_REMAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - TLI_B3_PG11 remapping"]
    #[inline(always)]
    pub fn tli_b3_pg11_remap(&mut self) -> TLI_B3_PG11_REMAP_W {
        TLI_B3_PG11_REMAP_W::new(self)
    }
    #[doc = "Bit 30 - TLI_B2_PG10 remapping"]
    #[inline(always)]
    pub fn tli_b2_pg10_remap(&mut self) -> TLI_B2_PG10_REMAP_W {
        TLI_B2_PG10_REMAP_W::new(self)
    }
    #[doc = "Bit 29 - TLI_G3_PG10 remapping"]
    #[inline(always)]
    pub fn tli_g3_pg10_remap(&mut self) -> TLI_G3_PG10_REMAP_W {
        TLI_G3_PG10_REMAP_W::new(self)
    }
    #[doc = "Bit 28 - TLI_CLK_PG7 remapping"]
    #[inline(always)]
    pub fn tli_clk_pg7_remap(&mut self) -> TLI_CLK_PG7_REMAP_W {
        TLI_CLK_PG7_REMAP_W::new(self)
    }
    #[doc = "Bit 27 - TLI_R7_PG6 remapping"]
    #[inline(always)]
    pub fn tli_r7_pg6_remap(&mut self) -> TLI_R7_PG6_REMAP_W {
        TLI_R7_PG6_REMAP_W::new(self)
    }
    #[doc = "Bit 26 - TLI_DE_PF10 remapping"]
    #[inline(always)]
    pub fn tli_de_pf10_remap(&mut self) -> TLI_DE_PF10_REMAP_W {
        TLI_DE_PF10_REMAP_W::new(self)
    }
    #[doc = "Bit 25 - TLI_R7_PE15 remapping"]
    #[inline(always)]
    pub fn tli_r7_pe15_remap(&mut self) -> TLI_R7_PE15_REMAP_W {
        TLI_R7_PE15_REMAP_W::new(self)
    }
    #[doc = "Bit 24 - TLI_CLK_PE14 remapping"]
    #[inline(always)]
    pub fn tli_clk_pe14_remap(&mut self) -> TLI_CLK_PE14_REMAP_W {
        TLI_CLK_PE14_REMAP_W::new(self)
    }
    #[doc = "Bit 23 - TLI_DE_PE13 remapping"]
    #[inline(always)]
    pub fn tli_de_pe13_remap(&mut self) -> TLI_DE_PE13_REMAP_W {
        TLI_DE_PE13_REMAP_W::new(self)
    }
    #[doc = "Bit 22 - TLI_B4_PE12 remapping"]
    #[inline(always)]
    pub fn tli_b4_pe12_remap(&mut self) -> TLI_B4_PE12_REMAP_W {
        TLI_B4_PE12_REMAP_W::new(self)
    }
    #[doc = "Bit 21 - TLI_G3_PE11 remapping"]
    #[inline(always)]
    pub fn tli_g3_pe11_remap(&mut self) -> TLI_G3_PE11_REMAP_W {
        TLI_G3_PE11_REMAP_W::new(self)
    }
    #[doc = "Bit 20 - TLI_G1_PE6 remapping"]
    #[inline(always)]
    pub fn tli_g1_pe6_remap(&mut self) -> TLI_G1_PE6_REMAP_W {
        TLI_G1_PE6_REMAP_W::new(self)
    }
    #[doc = "Bit 19 - TLI_G0_PE5 remapping"]
    #[inline(always)]
    pub fn tli_g0_pe5_remap(&mut self) -> TLI_G0_PE5_REMAP_W {
        TLI_G0_PE5_REMAP_W::new(self)
    }
    #[doc = "Bit 18 - TLI_B0_PE4 remapping"]
    #[inline(always)]
    pub fn tli_b0_pe4_remap(&mut self) -> TLI_B0_PE4_REMAP_W {
        TLI_B0_PE4_REMAP_W::new(self)
    }
    #[doc = "Bit 17 - TLI_B3_PD10 remapping"]
    #[inline(always)]
    pub fn tli_b3_pd10_remap(&mut self) -> TLI_B3_PD10_REMAP_W {
        TLI_B3_PD10_REMAP_W::new(self)
    }
    #[doc = "Bit 16 - TLI_B2_PD6 remapping"]
    #[inline(always)]
    pub fn tli_b2_pd6_remap(&mut self) -> TLI_B2_PD6_REMAP_W {
        TLI_B2_PD6_REMAP_W::new(self)
    }
    #[doc = "Bit 15 - TLI_G7_PD3 remapping"]
    #[inline(always)]
    pub fn tli_g7_pd3_remap(&mut self) -> TLI_G7_PD3_REMAP_W {
        TLI_G7_PD3_REMAP_W::new(self)
    }
    #[doc = "Bit 14 - TLI_R2_PC10 remapping"]
    #[inline(always)]
    pub fn tli_r2_pc10_remap(&mut self) -> TLI_R2_PC10_REMAP_W {
        TLI_R2_PC10_REMAP_W::new(self)
    }
    #[doc = "Bit 13 - TLI_G6_PC7 remapping"]
    #[inline(always)]
    pub fn tli_g6_pc7_remap(&mut self) -> TLI_G6_PC7_REMAP_W {
        TLI_G6_PC7_REMAP_W::new(self)
    }
    #[doc = "Bit 12 - TLI_HSYNC_PC6 remapping"]
    #[inline(always)]
    pub fn tli_hsync_pc6_remap(&mut self) -> TLI_HSYNC_PC6_REMAP_W {
        TLI_HSYNC_PC6_REMAP_W::new(self)
    }
    #[doc = "Bit 11 - TLI_G5_PB11 remapping"]
    #[inline(always)]
    pub fn tli_g5_pb11_remap(&mut self) -> TLI_G5_PB11_REMAP_W {
        TLI_G5_PB11_REMAP_W::new(self)
    }
    #[doc = "Bit 10 - TLI_G4_PB10 remapping"]
    #[inline(always)]
    pub fn tli_g4_pb10_remap(&mut self) -> TLI_G4_PB10_REMAP_W {
        TLI_G4_PB10_REMAP_W::new(self)
    }
    #[doc = "Bit 9 - TLI_B7_PB9 remapping"]
    #[inline(always)]
    pub fn tli_b7_pb9_remap(&mut self) -> TLI_B7_PB9_REMAP_W {
        TLI_B7_PB9_REMAP_W::new(self)
    }
    #[doc = "Bit 8 - TLI_B6_PB8 remapping"]
    #[inline(always)]
    pub fn tli_b6_pb8_remap(&mut self) -> TLI_B6_PB8_REMAP_W {
        TLI_B6_PB8_REMAP_W::new(self)
    }
    #[doc = "Bit 7 - TLI_R6_PB1 remapping"]
    #[inline(always)]
    pub fn tli_r6_pb1_remap(&mut self) -> TLI_R6_PB1_REMAP_W {
        TLI_R6_PB1_REMAP_W::new(self)
    }
    #[doc = "Bit 6 - TLI_R3_PB0 remapping"]
    #[inline(always)]
    pub fn tli_r3_pb0_remap(&mut self) -> TLI_R3_PB0_REMAP_W {
        TLI_R3_PB0_REMAP_W::new(self)
    }
    #[doc = "Bit 5 - TLI_R5_PA12 remapping"]
    #[inline(always)]
    pub fn tli_r5_pa12_remap(&mut self) -> TLI_R5_PA12_REMAP_W {
        TLI_R5_PA12_REMAP_W::new(self)
    }
    #[doc = "Bit 4 - TLI_R4_PA11 remapping"]
    #[inline(always)]
    pub fn tli_r4_pa11_remap(&mut self) -> TLI_R4_PA11_REMAP_W {
        TLI_R4_PA11_REMAP_W::new(self)
    }
    #[doc = "Bit 3 - TLI_R6_PA8 remapping"]
    #[inline(always)]
    pub fn tli_r6_pa8_remap(&mut self) -> TLI_R6_PA8_REMAP_W {
        TLI_R6_PA8_REMAP_W::new(self)
    }
    #[doc = "Bit 2 - TLI_G2_PA6 remapping"]
    #[inline(always)]
    pub fn tli_g2_pa6_remap(&mut self) -> TLI_G2_PA6_REMAP_W {
        TLI_G2_PA6_REMAP_W::new(self)
    }
    #[doc = "Bit 1 - TLI_VSYNC_PA4 remapping"]
    #[inline(always)]
    pub fn tli_vsync_pa4_remap(&mut self) -> TLI_VSYNC_PA4_REMAP_W {
        TLI_VSYNC_PA4_REMAP_W::new(self)
    }
    #[doc = "Bit 0 - TLI_B5_PA3 remapping"]
    #[inline(always)]
    pub fn tli_b5_pa3_remap(&mut self) -> TLI_B5_PA3_REMAP_W {
        TLI_B5_PA3_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO port configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf3](index.html) module"]
pub struct PCF3_SPEC;
impl crate::RegisterSpec for PCF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcf3::R](R) reader structure"]
impl crate::Readable for PCF3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcf3::W](W) writer structure"]
impl crate::Writable for PCF3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCF3 to value 0"]
impl crate::Resettable for PCF3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
