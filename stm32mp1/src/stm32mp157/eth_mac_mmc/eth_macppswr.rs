#[doc = "Register `ETH_MACPPSWR` reader"]
pub struct R(crate::R<ETH_MACPPSWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPPSWR` writer"]
pub struct W(crate::W<ETH_MACPPSWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSWR_SPEC>;
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
impl From<crate::W<ETH_MACPPSWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSWIDTH0` reader - PPSWIDTH0"]
pub type PPSWIDTH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PPSWIDTH0` writer - PPSWIDTH0"]
pub type PPSWIDTH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSWR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W<0> {
        PPSWIDTH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppswr](index.html) module"]
pub struct ETH_MACPPSWR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macppswr::R](R) reader structure"]
impl crate::Readable for ETH_MACPPSWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macppswr::W](W) writer structure"]
impl crate::Writable for ETH_MACPPSWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACPPSWR to value 0"]
impl crate::Resettable for ETH_MACPPSWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
