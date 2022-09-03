#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA1PD` reader - OPAMP1 power down"]
pub type OPA1PD_R = crate::BitReader<bool>;
#[doc = "Field `OPA1PD` writer - OPAMP1 power down"]
pub type OPA1PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S3SEL1` reader - Switch 3 for OPAMP1 enable"]
pub type S3SEL1_R = crate::BitReader<bool>;
#[doc = "Field `S3SEL1` writer - Switch 3 for OPAMP1 enable"]
pub type S3SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S4SEL1` reader - Switch 4 for OPAMP1 enable"]
pub type S4SEL1_R = crate::BitReader<bool>;
#[doc = "Field `S4SEL1` writer - Switch 4 for OPAMP1 enable"]
pub type S4SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S5SEL1` reader - Switch 5 for OPAMP1 enable"]
pub type S5SEL1_R = crate::BitReader<bool>;
#[doc = "Field `S5SEL1` writer - Switch 5 for OPAMP1 enable"]
pub type S5SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S6SEL1` reader - Switch 6 for OPAMP1 enable"]
pub type S6SEL1_R = crate::BitReader<bool>;
#[doc = "Field `S6SEL1` writer - Switch 6 for OPAMP1 enable"]
pub type S6SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA1CAL_L` reader - OPAMP1 offset calibration for P differential pair"]
pub type OPA1CAL_L_R = crate::BitReader<bool>;
#[doc = "Field `OPA1CAL_L` writer - OPAMP1 offset calibration for P differential pair"]
pub type OPA1CAL_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA1CAL_H` reader - OPAMP1 offset calibration for N differential pair"]
pub type OPA1CAL_H_R = crate::BitReader<bool>;
#[doc = "Field `OPA1CAL_H` writer - OPAMP1 offset calibration for N differential pair"]
pub type OPA1CAL_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA1LPM` reader - OPAMP1 low power mode"]
pub type OPA1LPM_R = crate::BitReader<bool>;
#[doc = "Field `OPA1LPM` writer - OPAMP1 low power mode"]
pub type OPA1LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA2PD` reader - OPAMP2 power down"]
pub type OPA2PD_R = crate::BitReader<bool>;
#[doc = "Field `OPA2PD` writer - OPAMP2 power down"]
pub type OPA2PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S3SEL2` reader - Switch 3 for OPAMP2 enable"]
pub type S3SEL2_R = crate::BitReader<bool>;
#[doc = "Field `S3SEL2` writer - Switch 3 for OPAMP2 enable"]
pub type S3SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S4SEL2` reader - Switch 4 for OPAMP2 enable"]
pub type S4SEL2_R = crate::BitReader<bool>;
#[doc = "Field `S4SEL2` writer - Switch 4 for OPAMP2 enable"]
pub type S4SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S5SEL2` reader - Switch 5 for OPAMP2 enable"]
pub type S5SEL2_R = crate::BitReader<bool>;
#[doc = "Field `S5SEL2` writer - Switch 5 for OPAMP2 enable"]
pub type S5SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S6SEL2` reader - Switch 6 for OPAMP2 enable"]
pub type S6SEL2_R = crate::BitReader<bool>;
#[doc = "Field `S6SEL2` writer - Switch 6 for OPAMP2 enable"]
pub type S6SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA2CAL_L` reader - OPAMP2 offset Calibration for P differential pair"]
pub type OPA2CAL_L_R = crate::BitReader<bool>;
#[doc = "Field `OPA2CAL_L` writer - OPAMP2 offset Calibration for P differential pair"]
pub type OPA2CAL_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA2CAL_H` reader - OPAMP2 offset calibration for N differential pair"]
pub type OPA2CAL_H_R = crate::BitReader<bool>;
#[doc = "Field `OPA2CAL_H` writer - OPAMP2 offset calibration for N differential pair"]
pub type OPA2CAL_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA2LPM` reader - OPAMP2 low power mode"]
pub type OPA2LPM_R = crate::BitReader<bool>;
#[doc = "Field `OPA2LPM` writer - OPAMP2 low power mode"]
pub type OPA2LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA3PD` reader - OPAMP3 power down"]
pub type OPA3PD_R = crate::BitReader<bool>;
#[doc = "Field `OPA3PD` writer - OPAMP3 power down"]
pub type OPA3PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S3SEL3` reader - Switch 3 for OPAMP3 Enable"]
pub type S3SEL3_R = crate::BitReader<bool>;
#[doc = "Field `S3SEL3` writer - Switch 3 for OPAMP3 Enable"]
pub type S3SEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S4SEL3` reader - Switch 4 for OPAMP3 enable"]
pub type S4SEL3_R = crate::BitReader<bool>;
#[doc = "Field `S4SEL3` writer - Switch 4 for OPAMP3 enable"]
pub type S4SEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S5SEL3` reader - Switch 5 for OPAMP3 enable"]
pub type S5SEL3_R = crate::BitReader<bool>;
#[doc = "Field `S5SEL3` writer - Switch 5 for OPAMP3 enable"]
pub type S5SEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S6SEL3` reader - Switch 6 for OPAMP3 enable"]
pub type S6SEL3_R = crate::BitReader<bool>;
#[doc = "Field `S6SEL3` writer - Switch 6 for OPAMP3 enable"]
pub type S6SEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA3CAL_L` reader - OPAMP3 offset Calibration for P differential pair"]
pub type OPA3CAL_L_R = crate::BitReader<bool>;
#[doc = "Field `OPA3CAL_L` writer - OPAMP3 offset Calibration for P differential pair"]
pub type OPA3CAL_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA3CAL_H` reader - OPAMP3 offset calibration for N differential pair"]
pub type OPA3CAL_H_R = crate::BitReader<bool>;
#[doc = "Field `OPA3CAL_H` writer - OPAMP3 offset calibration for N differential pair"]
pub type OPA3CAL_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA3LPM` reader - OPAMP3 low power mode"]
pub type OPA3LPM_R = crate::BitReader<bool>;
#[doc = "Field `OPA3LPM` writer - OPAMP3 low power mode"]
pub type OPA3LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `ANAWSEL1` reader - Switch SanA enable for OPAMP1"]
pub type ANAWSEL1_R = crate::BitReader<bool>;
#[doc = "Field `ANAWSEL1` writer - Switch SanA enable for OPAMP1"]
pub type ANAWSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `ANAWSEL2` reader - Switch SanA enable for OPAMP2"]
pub type ANAWSEL2_R = crate::BitReader<bool>;
#[doc = "Field `ANAWSEL2` writer - Switch SanA enable for OPAMP2"]
pub type ANAWSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `ANAWSEL3` reader - Switch SanA enable for OPAMP3"]
pub type ANAWSEL3_R = crate::BitReader<bool>;
#[doc = "Field `ANAWSEL3` writer - Switch SanA enable for OPAMP3"]
pub type ANAWSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `S7SEL2` reader - Switch 7 for OPAMP2 enable"]
pub type S7SEL2_R = crate::BitReader<bool>;
#[doc = "Field `S7SEL2` writer - Switch 7 for OPAMP2 enable"]
pub type S7SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `AOP_RANGE` reader - Power range selection"]
pub type AOP_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `AOP_RANGE` writer - Power range selection"]
pub type AOP_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA1CALOUT` reader - OPAMP1 calibration output"]
pub type OPA1CALOUT_R = crate::BitReader<bool>;
#[doc = "Field `OPA1CALOUT` writer - OPAMP1 calibration output"]
pub type OPA1CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA2CALOUT` reader - OPAMP2 calibration output"]
pub type OPA2CALOUT_R = crate::BitReader<bool>;
#[doc = "Field `OPA2CALOUT` writer - OPAMP2 calibration output"]
pub type OPA2CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `OPA3CALOUT` reader - OPAMP3 calibration output"]
pub type OPA3CALOUT_R = crate::BitReader<bool>;
#[doc = "Field `OPA3CALOUT` writer - OPAMP3 calibration output"]
pub type OPA3CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OPAMP1 power down"]
    #[inline(always)]
    pub fn opa1pd(&self) -> OPA1PD_R {
        OPA1PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Switch 3 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s3sel1(&self) -> S3SEL1_R {
        S3SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Switch 4 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s4sel1(&self) -> S4SEL1_R {
        S4SEL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Switch 5 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s5sel1(&self) -> S5SEL1_R {
        S5SEL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Switch 6 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s6sel1(&self) -> S6SEL1_R {
        S6SEL1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPAMP1 offset calibration for P differential pair"]
    #[inline(always)]
    pub fn opa1cal_l(&self) -> OPA1CAL_L_R {
        OPA1CAL_L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPAMP1 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa1cal_h(&self) -> OPA1CAL_H_R {
        OPA1CAL_H_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OPAMP1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&self) -> OPA1LPM_R {
        OPA1LPM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAMP2 power down"]
    #[inline(always)]
    pub fn opa2pd(&self) -> OPA2PD_R {
        OPA2PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Switch 3 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s3sel2(&self) -> S3SEL2_R {
        S3SEL2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Switch 4 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s4sel2(&self) -> S4SEL2_R {
        S4SEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Switch 5 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s5sel2(&self) -> S5SEL2_R {
        S5SEL2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Switch 6 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s6sel2(&self) -> S6SEL2_R {
        S6SEL2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP2 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa2cal_l(&self) -> OPA2CAL_L_R {
        OPA2CAL_L_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OPAMP2 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa2cal_h(&self) -> OPA2CAL_H_R {
        OPA2CAL_H_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OPAMP2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&self) -> OPA2LPM_R {
        OPA2LPM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPAMP3 power down"]
    #[inline(always)]
    pub fn opa3pd(&self) -> OPA3PD_R {
        OPA3PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Switch 3 for OPAMP3 Enable"]
    #[inline(always)]
    pub fn s3sel3(&self) -> S3SEL3_R {
        S3SEL3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Switch 4 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s4sel3(&self) -> S4SEL3_R {
        S4SEL3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Switch 5 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s5sel3(&self) -> S5SEL3_R {
        S5SEL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Switch 6 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s6sel3(&self) -> S6SEL3_R {
        S6SEL3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPAMP3 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa3cal_l(&self) -> OPA3CAL_L_R {
        OPA3CAL_L_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPAMP3 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa3cal_h(&self) -> OPA3CAL_H_R {
        OPA3CAL_H_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPAMP3 low power mode"]
    #[inline(always)]
    pub fn opa3lpm(&self) -> OPA3LPM_R {
        OPA3LPM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Switch SanA enable for OPAMP1"]
    #[inline(always)]
    pub fn anawsel1(&self) -> ANAWSEL1_R {
        ANAWSEL1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Switch SanA enable for OPAMP2"]
    #[inline(always)]
    pub fn anawsel2(&self) -> ANAWSEL2_R {
        ANAWSEL2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Switch SanA enable for OPAMP3"]
    #[inline(always)]
    pub fn anawsel3(&self) -> ANAWSEL3_R {
        ANAWSEL3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Switch 7 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s7sel2(&self) -> S7SEL2_R {
        S7SEL2_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power range selection"]
    #[inline(always)]
    pub fn aop_range(&self) -> AOP_RANGE_R {
        AOP_RANGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPAMP1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&self) -> OPA1CALOUT_R {
        OPA1CALOUT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&self) -> OPA2CALOUT_R {
        OPA2CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMP3 calibration output"]
    #[inline(always)]
    pub fn opa3calout(&self) -> OPA3CALOUT_R {
        OPA3CALOUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP1 power down"]
    #[inline(always)]
    pub fn opa1pd(&mut self) -> OPA1PD_W<0> {
        OPA1PD_W::new(self)
    }
    #[doc = "Bit 1 - Switch 3 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s3sel1(&mut self) -> S3SEL1_W<1> {
        S3SEL1_W::new(self)
    }
    #[doc = "Bit 2 - Switch 4 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s4sel1(&mut self) -> S4SEL1_W<2> {
        S4SEL1_W::new(self)
    }
    #[doc = "Bit 3 - Switch 5 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s5sel1(&mut self) -> S5SEL1_W<3> {
        S5SEL1_W::new(self)
    }
    #[doc = "Bit 4 - Switch 6 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s6sel1(&mut self) -> S6SEL1_W<4> {
        S6SEL1_W::new(self)
    }
    #[doc = "Bit 5 - OPAMP1 offset calibration for P differential pair"]
    #[inline(always)]
    pub fn opa1cal_l(&mut self) -> OPA1CAL_L_W<5> {
        OPA1CAL_L_W::new(self)
    }
    #[doc = "Bit 6 - OPAMP1 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa1cal_h(&mut self) -> OPA1CAL_H_W<6> {
        OPA1CAL_H_W::new(self)
    }
    #[doc = "Bit 7 - OPAMP1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&mut self) -> OPA1LPM_W<7> {
        OPA1LPM_W::new(self)
    }
    #[doc = "Bit 8 - OPAMP2 power down"]
    #[inline(always)]
    pub fn opa2pd(&mut self) -> OPA2PD_W<8> {
        OPA2PD_W::new(self)
    }
    #[doc = "Bit 9 - Switch 3 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s3sel2(&mut self) -> S3SEL2_W<9> {
        S3SEL2_W::new(self)
    }
    #[doc = "Bit 10 - Switch 4 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s4sel2(&mut self) -> S4SEL2_W<10> {
        S4SEL2_W::new(self)
    }
    #[doc = "Bit 11 - Switch 5 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s5sel2(&mut self) -> S5SEL2_W<11> {
        S5SEL2_W::new(self)
    }
    #[doc = "Bit 12 - Switch 6 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s6sel2(&mut self) -> S6SEL2_W<12> {
        S6SEL2_W::new(self)
    }
    #[doc = "Bit 13 - OPAMP2 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa2cal_l(&mut self) -> OPA2CAL_L_W<13> {
        OPA2CAL_L_W::new(self)
    }
    #[doc = "Bit 14 - OPAMP2 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa2cal_h(&mut self) -> OPA2CAL_H_W<14> {
        OPA2CAL_H_W::new(self)
    }
    #[doc = "Bit 15 - OPAMP2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&mut self) -> OPA2LPM_W<15> {
        OPA2LPM_W::new(self)
    }
    #[doc = "Bit 16 - OPAMP3 power down"]
    #[inline(always)]
    pub fn opa3pd(&mut self) -> OPA3PD_W<16> {
        OPA3PD_W::new(self)
    }
    #[doc = "Bit 17 - Switch 3 for OPAMP3 Enable"]
    #[inline(always)]
    pub fn s3sel3(&mut self) -> S3SEL3_W<17> {
        S3SEL3_W::new(self)
    }
    #[doc = "Bit 18 - Switch 4 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s4sel3(&mut self) -> S4SEL3_W<18> {
        S4SEL3_W::new(self)
    }
    #[doc = "Bit 19 - Switch 5 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s5sel3(&mut self) -> S5SEL3_W<19> {
        S5SEL3_W::new(self)
    }
    #[doc = "Bit 20 - Switch 6 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s6sel3(&mut self) -> S6SEL3_W<20> {
        S6SEL3_W::new(self)
    }
    #[doc = "Bit 21 - OPAMP3 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa3cal_l(&mut self) -> OPA3CAL_L_W<21> {
        OPA3CAL_L_W::new(self)
    }
    #[doc = "Bit 22 - OPAMP3 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa3cal_h(&mut self) -> OPA3CAL_H_W<22> {
        OPA3CAL_H_W::new(self)
    }
    #[doc = "Bit 23 - OPAMP3 low power mode"]
    #[inline(always)]
    pub fn opa3lpm(&mut self) -> OPA3LPM_W<23> {
        OPA3LPM_W::new(self)
    }
    #[doc = "Bit 24 - Switch SanA enable for OPAMP1"]
    #[inline(always)]
    pub fn anawsel1(&mut self) -> ANAWSEL1_W<24> {
        ANAWSEL1_W::new(self)
    }
    #[doc = "Bit 25 - Switch SanA enable for OPAMP2"]
    #[inline(always)]
    pub fn anawsel2(&mut self) -> ANAWSEL2_W<25> {
        ANAWSEL2_W::new(self)
    }
    #[doc = "Bit 26 - Switch SanA enable for OPAMP3"]
    #[inline(always)]
    pub fn anawsel3(&mut self) -> ANAWSEL3_W<26> {
        ANAWSEL3_W::new(self)
    }
    #[doc = "Bit 27 - Switch 7 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s7sel2(&mut self) -> S7SEL2_W<27> {
        S7SEL2_W::new(self)
    }
    #[doc = "Bit 28 - Power range selection"]
    #[inline(always)]
    pub fn aop_range(&mut self) -> AOP_RANGE_W<28> {
        AOP_RANGE_W::new(self)
    }
    #[doc = "Bit 29 - OPAMP1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&mut self) -> OPA1CALOUT_W<29> {
        OPA1CALOUT_W::new(self)
    }
    #[doc = "Bit 30 - OPAMP2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&mut self) -> OPA2CALOUT_W<30> {
        OPA2CALOUT_W::new(self)
    }
    #[doc = "Bit 31 - OPAMP3 calibration output"]
    #[inline(always)]
    pub fn opa3calout(&mut self) -> OPA3CALOUT_W<31> {
        OPA3CALOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x0001_0101"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0101
    }
}
