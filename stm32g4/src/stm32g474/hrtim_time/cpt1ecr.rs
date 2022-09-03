#[doc = "Register `CPT1ECR` reader"]
pub struct R(crate::R<CPT1ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT1ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT1ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT1ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPT1ECR` writer"]
pub struct W(crate::W<CPT1ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPT1ECR_SPEC>;
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
impl From<crate::W<CPT1ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPT1ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWCPT` reader - Software Capture"]
pub type SWCPT_R = crate::BitReader<bool>;
#[doc = "Field `SWCPT` writer - Software Capture"]
pub type SWCPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `UDPCPT` reader - Update Capture"]
pub type UDPCPT_R = crate::BitReader<bool>;
#[doc = "Field `UDPCPT` writer - Update Capture"]
pub type UDPCPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV1CPT` reader - External Event 1 Capture"]
pub type EXEV1CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV1CPT` writer - External Event 1 Capture"]
pub type EXEV1CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV2CPT` reader - External Event 2 Capture"]
pub type EXEV2CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV2CPT` writer - External Event 2 Capture"]
pub type EXEV2CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV3CPT` reader - External Event 3 Capture"]
pub type EXEV3CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV3CPT` writer - External Event 3 Capture"]
pub type EXEV3CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV4CPT` reader - External Event 4 Capture"]
pub type EXEV4CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV4CPT` writer - External Event 4 Capture"]
pub type EXEV4CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV5CPT` reader - External Event 5 Capture"]
pub type EXEV5CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV5CPT` writer - External Event 5 Capture"]
pub type EXEV5CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV6CPT` reader - External Event 6 Capture"]
pub type EXEV6CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV6CPT` writer - External Event 6 Capture"]
pub type EXEV6CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV7CPT` reader - External Event 7 Capture"]
pub type EXEV7CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV7CPT` writer - External Event 7 Capture"]
pub type EXEV7CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV8CPT` reader - External Event 8 Capture"]
pub type EXEV8CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV8CPT` writer - External Event 8 Capture"]
pub type EXEV8CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV9CPT` reader - External Event 9 Capture"]
pub type EXEV9CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV9CPT` writer - External Event 9 Capture"]
pub type EXEV9CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `EXEV10CPT` reader - External Event 10 Capture"]
pub type EXEV10CPT_R = crate::BitReader<bool>;
#[doc = "Field `EXEV10CPT` writer - External Event 10 Capture"]
pub type EXEV10CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TA1SET` reader - Timer A output 1 Set"]
pub type TA1SET_R = crate::BitReader<bool>;
#[doc = "Field `TA1SET` writer - Timer A output 1 Set"]
pub type TA1SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TA1RST` reader - Timer A output 1 Reset"]
pub type TA1RST_R = crate::BitReader<bool>;
#[doc = "Field `TA1RST` writer - Timer A output 1 Reset"]
pub type TA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TACMP1` reader - Timer A Compare 1"]
pub type TACMP1_R = crate::BitReader<bool>;
#[doc = "Field `TACMP1` writer - Timer A Compare 1"]
pub type TACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TACMP2` reader - Timer A Compare 2"]
pub type TACMP2_R = crate::BitReader<bool>;
#[doc = "Field `TACMP2` writer - Timer A Compare 2"]
pub type TACMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TB1SET` reader - Timer B output 1 Set"]
pub type TB1SET_R = crate::BitReader<bool>;
#[doc = "Field `TB1SET` writer - Timer B output 1 Set"]
pub type TB1SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TB1RST` reader - Timer B output 1 Reset"]
pub type TB1RST_R = crate::BitReader<bool>;
#[doc = "Field `TB1RST` writer - Timer B output 1 Reset"]
pub type TB1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TBCMP1` reader - Timer B Compare 1"]
pub type TBCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TBCMP1` writer - Timer B Compare 1"]
pub type TBCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TBCMP2` reader - Timer B Compare 2"]
pub type TBCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TBCMP2` writer - Timer B Compare 2"]
pub type TBCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TC1SET` reader - Timer C output 1 Set"]
pub type TC1SET_R = crate::BitReader<bool>;
#[doc = "Field `TC1SET` writer - Timer C output 1 Set"]
pub type TC1SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TC1RST` reader - Timer C output 1 Reset"]
pub type TC1RST_R = crate::BitReader<bool>;
#[doc = "Field `TC1RST` writer - Timer C output 1 Reset"]
pub type TC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TCCMP1` reader - Timer C Compare 1"]
pub type TCCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TCCMP1` writer - Timer C Compare 1"]
pub type TCCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TCCMP2` reader - Timer C Compare 2"]
pub type TCCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TCCMP2` writer - Timer C Compare 2"]
pub type TCCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TD1SET` reader - Timer D output 1 Set"]
pub type TD1SET_R = crate::BitReader<bool>;
#[doc = "Field `TD1SET` writer - Timer D output 1 Set"]
pub type TD1SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TD1RST` reader - Timer D output 1 Reset"]
pub type TD1RST_R = crate::BitReader<bool>;
#[doc = "Field `TD1RST` writer - Timer D output 1 Reset"]
pub type TD1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TDCMP1` reader - Timer D Compare 1"]
pub type TDCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TDCMP1` writer - Timer D Compare 1"]
pub type TDCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TDCMP2` reader - Timer D Compare 2"]
pub type TDCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TDCMP2` writer - Timer D Compare 2"]
pub type TDCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TF1SET` reader - TF1SET"]
pub type TF1SET_R = crate::BitReader<bool>;
#[doc = "Field `TF1SET` writer - TF1SET"]
pub type TF1SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TF1RST` reader - TF1RST"]
pub type TF1RST_R = crate::BitReader<bool>;
#[doc = "Field `TF1RST` writer - TF1RST"]
pub type TF1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TFCMP1` reader - TFCMP1"]
pub type TFCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TFCMP1` writer - TFCMP1"]
pub type TFCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
#[doc = "Field `TFCMP2` reader - TFCMP2"]
pub type TFCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TFCMP2` writer - TFCMP2"]
pub type TFCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT1ECR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    pub fn swcpt(&self) -> SWCPT_R {
        SWCPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    pub fn udpcpt(&self) -> UDPCPT_R {
        UDPCPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    pub fn exev1cpt(&self) -> EXEV1CPT_R {
        EXEV1CPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    pub fn exev2cpt(&self) -> EXEV2CPT_R {
        EXEV2CPT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    pub fn exev3cpt(&self) -> EXEV3CPT_R {
        EXEV3CPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    pub fn exev4cpt(&self) -> EXEV4CPT_R {
        EXEV4CPT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    pub fn exev5cpt(&self) -> EXEV5CPT_R {
        EXEV5CPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    pub fn exev6cpt(&self) -> EXEV6CPT_R {
        EXEV6CPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    pub fn exev7cpt(&self) -> EXEV7CPT_R {
        EXEV7CPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    pub fn exev8cpt(&self) -> EXEV8CPT_R {
        EXEV8CPT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    pub fn exev9cpt(&self) -> EXEV9CPT_R {
        EXEV9CPT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    pub fn exev10cpt(&self) -> EXEV10CPT_R {
        EXEV10CPT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    pub fn ta1set(&self) -> TA1SET_R {
        TA1SET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    pub fn ta1rst(&self) -> TA1RST_R {
        TA1RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    pub fn tb1set(&self) -> TB1SET_R {
        TB1SET_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    pub fn tb1rst(&self) -> TB1RST_R {
        TB1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    pub fn tc1set(&self) -> TC1SET_R {
        TC1SET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    pub fn tc1rst(&self) -> TC1RST_R {
        TC1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    pub fn td1set(&self) -> TD1SET_R {
        TD1SET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    pub fn td1rst(&self) -> TD1RST_R {
        TD1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TF1SET"]
    #[inline(always)]
    pub fn tf1set(&self) -> TF1SET_R {
        TF1SET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TF1RST"]
    #[inline(always)]
    pub fn tf1rst(&self) -> TF1RST_R {
        TF1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TFCMP1"]
    #[inline(always)]
    pub fn tfcmp1(&self) -> TFCMP1_R {
        TFCMP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TFCMP2"]
    #[inline(always)]
    pub fn tfcmp2(&self) -> TFCMP2_R {
        TFCMP2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    pub fn swcpt(&mut self) -> SWCPT_W<0> {
        SWCPT_W::new(self)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    pub fn udpcpt(&mut self) -> UDPCPT_W<1> {
        UDPCPT_W::new(self)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    pub fn exev1cpt(&mut self) -> EXEV1CPT_W<2> {
        EXEV1CPT_W::new(self)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    pub fn exev2cpt(&mut self) -> EXEV2CPT_W<3> {
        EXEV2CPT_W::new(self)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    pub fn exev3cpt(&mut self) -> EXEV3CPT_W<4> {
        EXEV3CPT_W::new(self)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    pub fn exev4cpt(&mut self) -> EXEV4CPT_W<5> {
        EXEV4CPT_W::new(self)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    pub fn exev5cpt(&mut self) -> EXEV5CPT_W<6> {
        EXEV5CPT_W::new(self)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    pub fn exev6cpt(&mut self) -> EXEV6CPT_W<7> {
        EXEV6CPT_W::new(self)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    pub fn exev7cpt(&mut self) -> EXEV7CPT_W<8> {
        EXEV7CPT_W::new(self)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    pub fn exev8cpt(&mut self) -> EXEV8CPT_W<9> {
        EXEV8CPT_W::new(self)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    pub fn exev9cpt(&mut self) -> EXEV9CPT_W<10> {
        EXEV9CPT_W::new(self)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    pub fn exev10cpt(&mut self) -> EXEV10CPT_W<11> {
        EXEV10CPT_W::new(self)
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    pub fn ta1set(&mut self) -> TA1SET_W<12> {
        TA1SET_W::new(self)
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    pub fn ta1rst(&mut self) -> TA1RST_W<13> {
        TA1RST_W::new(self)
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    pub fn tacmp1(&mut self) -> TACMP1_W<14> {
        TACMP1_W::new(self)
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    pub fn tacmp2(&mut self) -> TACMP2_W<15> {
        TACMP2_W::new(self)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    pub fn tb1set(&mut self) -> TB1SET_W<16> {
        TB1SET_W::new(self)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    pub fn tb1rst(&mut self) -> TB1RST_W<17> {
        TB1RST_W::new(self)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<18> {
        TBCMP1_W::new(self)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<19> {
        TBCMP2_W::new(self)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    pub fn tc1set(&mut self) -> TC1SET_W<20> {
        TC1SET_W::new(self)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    pub fn tc1rst(&mut self) -> TC1RST_W<21> {
        TC1RST_W::new(self)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn tccmp1(&mut self) -> TCCMP1_W<22> {
        TCCMP1_W::new(self)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn tccmp2(&mut self) -> TCCMP2_W<23> {
        TCCMP2_W::new(self)
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    pub fn td1set(&mut self) -> TD1SET_W<24> {
        TD1SET_W::new(self)
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    pub fn td1rst(&mut self) -> TD1RST_W<25> {
        TD1RST_W::new(self)
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<26> {
        TDCMP1_W::new(self)
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<27> {
        TDCMP2_W::new(self)
    }
    #[doc = "Bit 28 - TF1SET"]
    #[inline(always)]
    pub fn tf1set(&mut self) -> TF1SET_W<28> {
        TF1SET_W::new(self)
    }
    #[doc = "Bit 29 - TF1RST"]
    #[inline(always)]
    pub fn tf1rst(&mut self) -> TF1RST_W<29> {
        TF1RST_W::new(self)
    }
    #[doc = "Bit 30 - TFCMP1"]
    #[inline(always)]
    pub fn tfcmp1(&mut self) -> TFCMP1_W<30> {
        TFCMP1_W::new(self)
    }
    #[doc = "Bit 31 - TFCMP2"]
    #[inline(always)]
    pub fn tfcmp2(&mut self) -> TFCMP2_W<31> {
        TFCMP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Capture 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1ecr](index.html) module"]
pub struct CPT1ECR_SPEC;
impl crate::RegisterSpec for CPT1ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpt1ecr::R](R) reader structure"]
impl crate::Readable for CPT1ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpt1ecr::W](W) writer structure"]
impl crate::Writable for CPT1ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPT1ECR to value 0"]
impl crate::Resettable for CPT1ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
