#[doc = "Register `TIM6_CCR5` reader"]
pub struct R(crate::R<TIM6_CCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_CCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_CCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_CCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM6_CCR5` writer"]
pub struct W(crate::W<TIM6_CCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_CCR5_SPEC>;
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
impl From<crate::W<TIM6_CCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_CCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR5` reader - CCR5"]
pub type CCR5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR5` writer - CCR5"]
pub type CCR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM6_CCR5_SPEC, u16, u16, 16, O>;
#[doc = "Field `GC5C1` reader - GC5C1"]
pub type GC5C1_R = crate::BitReader<bool>;
#[doc = "Field `GC5C1` writer - GC5C1"]
pub type GC5C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CCR5_SPEC, bool, O>;
#[doc = "Field `GC5C2` reader - GC5C2"]
pub type GC5C2_R = crate::BitReader<bool>;
#[doc = "Field `GC5C2` writer - GC5C2"]
pub type GC5C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CCR5_SPEC, bool, O>;
#[doc = "Field `GC5C3` reader - GC5C3"]
pub type GC5C3_R = crate::BitReader<bool>;
#[doc = "Field `GC5C3` writer - GC5C3"]
pub type GC5C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM6_CCR5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - CCR5"]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - GC5C1"]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GC5C2"]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GC5C3"]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR5"]
    #[inline(always)]
    pub fn ccr5(&mut self) -> CCR5_W<0> {
        CCR5_W::new(self)
    }
    #[doc = "Bit 29 - GC5C1"]
    #[inline(always)]
    pub fn gc5c1(&mut self) -> GC5C1_W<29> {
        GC5C1_W::new(self)
    }
    #[doc = "Bit 30 - GC5C2"]
    #[inline(always)]
    pub fn gc5c2(&mut self) -> GC5C2_W<30> {
        GC5C2_W::new(self)
    }
    #[doc = "Bit 31 - GC5C3"]
    #[inline(always)]
    pub fn gc5c3(&mut self) -> GC5C3_W<31> {
        GC5C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM6 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr5](index.html) module"]
pub struct TIM6_CCR5_SPEC;
impl crate::RegisterSpec for TIM6_CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim6_ccr5::R](R) reader structure"]
impl crate::Readable for TIM6_CCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim6_ccr5::W](W) writer structure"]
impl crate::Writable for TIM6_CCR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM6_CCR5 to value 0"]
impl crate::Resettable for TIM6_CCR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
