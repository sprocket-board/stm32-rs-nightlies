#[doc = "Register `BRR` reader"]
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRR_0_3` reader - BRR_0_3"]
pub type BRR_0_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRR_0_3` writer - BRR_0_3"]
pub type BRR_0_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BRR_4_15` reader - BRR_4_15"]
pub type BRR_4_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRR_4_15` writer - BRR_4_15"]
pub type BRR_4_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:3 - BRR_0_3"]
    #[inline(always)]
    pub fn brr_0_3(&self) -> BRR_0_3_R {
        BRR_0_3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - BRR_4_15"]
    #[inline(always)]
    pub fn brr_4_15(&self) -> BRR_4_15_R {
        BRR_4_15_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - BRR_0_3"]
    #[inline(always)]
    pub fn brr_0_3(&mut self) -> BRR_0_3_W<0> {
        BRR_0_3_W::new(self)
    }
    #[doc = "Bits 4:15 - BRR_4_15"]
    #[inline(always)]
    pub fn brr_4_15(&mut self) -> BRR_4_15_W<4> {
        BRR_4_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brr::R](R) reader structure"]
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
