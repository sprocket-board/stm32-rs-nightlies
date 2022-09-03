#[doc = "Register `RCC_APB2DIVR` reader"]
pub struct R(crate::R<RCC_APB2DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2DIVR` writer"]
pub struct W(crate::W<RCC_APB2DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2DIVR_SPEC>;
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
impl From<crate::W<RCC_APB2DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB2DIV` reader - APB2DIV"]
pub type APB2DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB2DIV` writer - APB2DIV"]
pub type APB2DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_APB2DIVR_SPEC, u8, u8, 3, O>;
#[doc = "Field `APB2DIVRDY` reader - APB2DIVRDY"]
pub type APB2DIVRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - APB2DIVRDY"]
    #[inline(always)]
    pub fn apb2divrdy(&self) -> APB2DIVRDY_R {
        APB2DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    pub fn apb2div(&mut self) -> APB2DIV_W<0> {
        APB2DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2divr](index.html) module"]
pub struct RCC_APB2DIVR_SPEC;
impl crate::RegisterSpec for RCC_APB2DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2divr::R](R) reader structure"]
impl crate::Readable for RCC_APB2DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2divr::W](W) writer structure"]
impl crate::Writable for RCC_APB2DIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB2DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB2DIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
