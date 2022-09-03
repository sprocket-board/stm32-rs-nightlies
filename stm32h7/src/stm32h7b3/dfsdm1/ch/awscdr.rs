#[doc = "Register `AWSCDR` reader"]
pub struct R(crate::R<AWSCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWSCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWSCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWSCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWSCDR` writer"]
pub struct W(crate::W<AWSCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWSCDR_SPEC>;
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
impl From<crate::W<AWSCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWSCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDT` reader - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
pub type SCDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCDT` writer - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
pub type SCDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWSCDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BKSCD` reader - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
pub type BKSCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKSCD` writer - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
pub type BKSCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWSCDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWFOSR` reader - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
pub type AWFOSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWFOSR` writer - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
pub type AWFOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWSCDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `AWFORD` reader - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type AWFORD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWFORD` writer - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type AWFORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWSCDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W<0> {
        SCDT_W::new(self)
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y"]
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W<12> {
        BKSCD_W::new(self)
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is"]
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W<16> {
        AWFOSR_W::new(self)
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W<22> {
        AWFORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel 0 analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awscdr](index.html) module"]
pub struct AWSCDR_SPEC;
impl crate::RegisterSpec for AWSCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awscdr::R](R) reader structure"]
impl crate::Readable for AWSCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awscdr::W](W) writer structure"]
impl crate::Writable for AWSCDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWSCDR to value 0"]
impl crate::Resettable for AWSCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
