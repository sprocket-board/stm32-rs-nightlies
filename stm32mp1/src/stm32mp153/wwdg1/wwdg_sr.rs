#[doc = "Register `WWDG_SR` reader"]
pub struct R(crate::R<WWDG_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDG_SR` writer"]
pub struct W(crate::W<WWDG_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDG_SR_SPEC>;
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
impl From<crate::W<WWDG_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WWDG_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWIF` reader - EWIF"]
pub type EWIF_R = crate::BitReader<bool>;
#[doc = "Field `EWIF` writer - EWIF"]
pub type EWIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WWDG_SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EWIF"]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EWIF"]
    #[inline(always)]
    pub fn ewif(&mut self) -> EWIF_W<0> {
        EWIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_sr](index.html) module"]
pub struct WWDG_SR_SPEC;
impl crate::RegisterSpec for WWDG_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_sr::R](R) reader structure"]
impl crate::Readable for WWDG_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdg_sr::W](W) writer structure"]
impl crate::Writable for WWDG_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDG_SR to value 0"]
impl crate::Resettable for WWDG_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
