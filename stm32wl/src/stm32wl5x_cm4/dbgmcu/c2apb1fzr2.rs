#[doc = "Register `C2APB1FZR2` reader"]
pub struct R(crate::R<C2APB1FZR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1FZR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1FZR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1FZR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB1FZR2` writer"]
pub struct W(crate::W<C2APB1FZR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1FZR2_SPEC>;
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
impl From<crate::W<C2APB1FZR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1FZR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - DBG_LPTIM2_STOP"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM2_STOP` writer - DBG_LPTIM2_STOP"]
pub type DBG_LPTIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB1FZR2_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM3_STOP` reader - DBG_LPTIM3_STOP"]
pub type DBG_LPTIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM3_STOP` writer - DBG_LPTIM3_STOP"]
pub type DBG_LPTIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB1FZR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<5> {
        DBG_LPTIM2_STOP_W::new(self)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<6> {
        DBG_LPTIM3_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \\[dual core device\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1fzr2](index.html) module"]
pub struct C2APB1FZR2_SPEC;
impl crate::RegisterSpec for C2APB1FZR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb1fzr2::R](R) reader structure"]
impl crate::Readable for C2APB1FZR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb1fzr2::W](W) writer structure"]
impl crate::Writable for C2APB1FZR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB1FZR2 to value 0"]
impl crate::Resettable for C2APB1FZR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
