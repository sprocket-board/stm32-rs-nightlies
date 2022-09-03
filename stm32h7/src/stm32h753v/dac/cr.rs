#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type EN1_R = crate::BitReader<bool>;
#[doc = "Field `EN1` writer - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type TEN1_R = crate::BitReader<bool>;
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type TEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type TSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type TSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type WAVE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type WAVE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type MAMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type MAMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type DMAEN1_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type DMAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type DMAUDRIE1_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type DMAUDRIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CEN1` reader - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type CEN1_R = crate::BitReader<bool>;
#[doc = "Field `CEN1` writer - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type CEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EN2` reader - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
pub type EN2_R = crate::BitReader<bool>;
#[doc = "Field `EN2` writer - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TEN2` reader - DAC channel2 trigger enable"]
pub type TEN2_R = crate::BitReader<bool>;
#[doc = "Field `TEN2` writer - DAC channel2 trigger enable"]
pub type TEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSEL2` reader - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
pub type TSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEL2` writer - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
pub type TSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
pub type WAVE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
pub type WAVE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MAMP2` reader - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type MAMP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAMP2` writer - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type MAMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMAEN2` reader - DAC channel2 DMA enable This bit is set and cleared by software."]
pub type DMAEN2_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN2` writer - DAC channel2 DMA enable This bit is set and cleared by software."]
pub type DMAEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
pub type DMAUDRIE2_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
pub type DMAUDRIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CEN2` reader - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type CEN2_R = crate::BitReader<bool>;
#[doc = "Field `CEN2` writer - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type CEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W<1> {
        TEN1_W::new(self)
    }
    #[doc = "Bits 2:4 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W<2> {
        TSEL1_W::new(self)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W<6> {
        WAVE1_W::new(self)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W<8> {
        MAMP1_W::new(self)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W<12> {
        DMAEN1_W::new(self)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<13> {
        DMAUDRIE1_W::new(self)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN1_W<14> {
        CEN1_W::new(self)
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<16> {
        EN2_W::new(self)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN2_W<17> {
        TEN2_W::new(self)
    }
    #[doc = "Bits 18:20 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
    #[inline(always)]
    pub fn tsel2(&mut self) -> TSEL2_W<18> {
        TSEL2_W::new(self)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE2_W<22> {
        WAVE2_W::new(self)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP2_W<24> {
        MAMP2_W::new(self)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W<28> {
        DMAEN2_W::new(self)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W<29> {
        DMAUDRIE2_W::new(self)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen2(&mut self) -> CEN2_W<30> {
        CEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}