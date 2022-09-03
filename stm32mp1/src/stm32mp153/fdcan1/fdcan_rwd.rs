#[doc = "Register `FDCAN_RWD` reader"]
pub struct R(crate::R<FDCAN_RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RWD` writer"]
pub struct W(crate::W<FDCAN_RWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RWD_SPEC>;
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
impl From<crate::W<FDCAN_RWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDC` reader - WDC"]
pub type WDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDC` writer - WDC"]
pub type WDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RWD_SPEC, u8, u8, 8, O>;
#[doc = "Field `WDV` reader - WDV"]
pub type WDV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDC"]
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W<0> {
        WDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rwd](index.html) module"]
pub struct FDCAN_RWD_SPEC;
impl crate::RegisterSpec for FDCAN_RWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rwd::R](R) reader structure"]
impl crate::Readable for FDCAN_RWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rwd::W](W) writer structure"]
impl crate::Writable for FDCAN_RWD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_RWD to value 0"]
impl crate::Resettable for FDCAN_RWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}