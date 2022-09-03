#[doc = "Register `HDP_CTRL` reader"]
pub struct R(crate::R<HDP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HDP_CTRL` writer"]
pub struct W(crate::W<HDP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDP_CTRL_SPEC>;
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
impl From<crate::W<HDP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HDP_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HDP Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_ctrl](index.html) module"]
pub struct HDP_CTRL_SPEC;
impl crate::RegisterSpec for HDP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdp_ctrl::R](R) reader structure"]
impl crate::Readable for HDP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hdp_ctrl::W](W) writer structure"]
impl crate::Writable for HDP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HDP_CTRL to value 0"]
impl crate::Resettable for HDP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
