#[doc = "Register `IWDG_RLR` reader"]
pub struct R(crate::R<IWDG_RLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_RLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_RLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_RLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWDG_RLR` writer"]
pub struct W(crate::W<IWDG_RLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_RLR_SPEC>;
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
impl From<crate::W<IWDG_RLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_RLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RL` reader - RL"]
pub type RL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RL` writer - RL"]
pub type RL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IWDG_RLR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - RL"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RL"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W<0> {
        RL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_rlr](index.html) module"]
pub struct IWDG_RLR_SPEC;
impl crate::RegisterSpec for IWDG_RLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwdg_rlr::R](R) reader structure"]
impl crate::Readable for IWDG_RLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwdg_rlr::W](W) writer structure"]
impl crate::Writable for IWDG_RLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IWDG_RLR to value 0x0fff"]
impl crate::Resettable for IWDG_RLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
