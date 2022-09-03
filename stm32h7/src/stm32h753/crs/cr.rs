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
#[doc = "Field `SYNCOKIE` reader - SYNC event OK interrupt enable"]
pub type SYNCOKIE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCOKIE` writer - SYNC event OK interrupt enable"]
pub type SYNCOKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SYNCWARNIE` reader - SYNC warning interrupt enable"]
pub type SYNCWARNIE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCWARNIE` writer - SYNC warning interrupt enable"]
pub type SYNCWARNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ESYNCIE` reader - Expected SYNC interrupt enable"]
pub type ESYNCIE_R = crate::BitReader<bool>;
#[doc = "Field `ESYNCIE` writer - Expected SYNC interrupt enable"]
pub type ESYNCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CEN` reader - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AUTOTRIMEN` reader - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
pub type AUTOTRIMEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTRIMEN` writer - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
pub type AUTOTRIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SWSYNC` reader - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SWSYNC_R = crate::BitReader<bool>;
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn syncokie(&self) -> SYNCOKIE_R {
        SYNCOKIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn syncwarnie(&self) -> SYNCWARNIE_R {
        SYNCWARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn esyncie(&self) -> ESYNCIE_R {
        ESYNCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
    #[inline(always)]
    pub fn autotrimen(&self) -> AUTOTRIMEN_R {
        AUTOTRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn syncokie(&mut self) -> SYNCOKIE_W<0> {
        SYNCOKIE_W::new(self)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn syncwarnie(&mut self) -> SYNCWARNIE_W<1> {
        SYNCWARNIE_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<2> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn esyncie(&mut self) -> ESYNCIE_W<3> {
        ESYNCIE_W::new(self)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<5> {
        CEN_W::new(self)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
    #[inline(always)]
    pub fn autotrimen(&mut self) -> AUTOTRIMEN_W<6> {
        AUTOTRIMEN_W::new(self)
    }
    #[doc = "Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<8> {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0x2000"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
