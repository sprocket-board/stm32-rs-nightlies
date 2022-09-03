#[doc = "Register `DAC_DHR12LD` reader"]
pub struct R(crate::R<DAC_DHR12LD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DHR12LD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DHR12LD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DHR12LD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DHR12LD` writer"]
pub struct W(crate::W<DAC_DHR12LD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DHR12LD_SPEC>;
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
impl From<crate::W<DAC_DHR12LD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DHR12LD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC1DHR` reader - DACC1DHR"]
pub type DACC1DHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC1DHR` writer - DACC1DHR"]
pub type DACC1DHR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12LD_SPEC, u16, u16, 12, O>;
#[doc = "Field `DACC2DHR` reader - DACC2DHR"]
pub type DACC2DHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC2DHR` writer - DACC2DHR"]
pub type DACC2DHR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12LD_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 4:15 - DACC1DHR"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DACC2DHR"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DACC1DHR"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<4> {
        DACC1DHR_W::new(self)
    }
    #[doc = "Bits 20:31 - DACC2DHR"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<20> {
        DACC2DHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dual DAC 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12ld](index.html) module"]
pub struct DAC_DHR12LD_SPEC;
impl crate::RegisterSpec for DAC_DHR12LD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dhr12ld::R](R) reader structure"]
impl crate::Readable for DAC_DHR12LD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dhr12ld::W](W) writer structure"]
impl crate::Writable for DAC_DHR12LD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_DHR12LD to value 0"]
impl crate::Resettable for DAC_DHR12LD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
