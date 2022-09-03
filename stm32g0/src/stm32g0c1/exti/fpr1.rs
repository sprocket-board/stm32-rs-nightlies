#[doc = "Register `FPR1` reader"]
pub struct R(crate::R<FPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPR1` writer"]
pub struct W(crate::W<FPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR1_SPEC>;
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
impl From<crate::W<FPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIF0` reader - Falling edge event pending for configurable line"]
pub type FPIF0_R = crate::BitReader<FPIF0R_A>;
#[doc = "Falling edge event pending for configurable line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPIF0R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<FPIF0R_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF0R_A) -> Self {
        variant as u8 != 0
    }
}
impl FPIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPIF0R_A {
        match self.bits {
            false => FPIF0R_A::NotPending,
            true => FPIF0R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF0R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF0R_A::Pending
    }
}
#[doc = "Falling edge event pending for configurable line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPIF0W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<FPIF0W_AW> for bool {
    #[inline(always)]
    fn from(variant: FPIF0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF0` writer - Falling edge event pending for configurable line"]
pub type FPIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, FPIF0W_AW, O>;
impl<'a, const O: u8> FPIF0_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF0W_AW::Clear)
    }
}
#[doc = "Field `FPIF1` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF1_R;
#[doc = "Field `FPIF2` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF2_R;
#[doc = "Field `FPIF3` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF3_R;
#[doc = "Field `FPIF4` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF4_R;
#[doc = "Field `FPIF5` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF5_R;
#[doc = "Field `FPIF6` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF6_R;
#[doc = "Field `FPIF7` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF7_R;
#[doc = "Field `FPIF8` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF8_R;
#[doc = "Field `FPIF9` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF9_R;
#[doc = "Field `FPIF10` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF10_R;
#[doc = "Field `FPIF11` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF11_R;
#[doc = "Field `FPIF12` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF12_R;
#[doc = "Field `FPIF13` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF13_R;
#[doc = "Field `FPIF14` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF14_R;
#[doc = "Field `FPIF15` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF15_R;
#[doc = "Field `FPIF16` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF16_R;
#[doc = "Field `FPIF17` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF17_R;
#[doc = "Field `FPIF18` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF18_R;
#[doc = "Field `FPIF20` reader - Falling edge event pending for configurable line"]
pub use FPIF0_R as FPIF20_R;
#[doc = "Field `FPIF1` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF1_W;
#[doc = "Field `FPIF2` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF2_W;
#[doc = "Field `FPIF3` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF3_W;
#[doc = "Field `FPIF4` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF4_W;
#[doc = "Field `FPIF5` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF5_W;
#[doc = "Field `FPIF6` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF6_W;
#[doc = "Field `FPIF7` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF7_W;
#[doc = "Field `FPIF8` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF8_W;
#[doc = "Field `FPIF9` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF9_W;
#[doc = "Field `FPIF10` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF10_W;
#[doc = "Field `FPIF11` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF11_W;
#[doc = "Field `FPIF12` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF12_W;
#[doc = "Field `FPIF13` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF13_W;
#[doc = "Field `FPIF14` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF14_W;
#[doc = "Field `FPIF15` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF15_W;
#[doc = "Field `FPIF16` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF16_W;
#[doc = "Field `FPIF17` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF17_W;
#[doc = "Field `FPIF18` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF18_W;
#[doc = "Field `FPIF20` writer - Falling edge event pending for configurable line"]
pub use FPIF0_W as FPIF20_W;
impl R {
    #[doc = "Bit 0 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif17(&self) -> FPIF17_R {
        FPIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif18(&self) -> FPIF18_R {
        FPIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif20(&self) -> FPIF20_R {
        FPIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif0(&mut self) -> FPIF0_W<0> {
        FPIF0_W::new(self)
    }
    #[doc = "Bit 1 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif1(&mut self) -> FPIF1_W<1> {
        FPIF1_W::new(self)
    }
    #[doc = "Bit 2 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W<2> {
        FPIF2_W::new(self)
    }
    #[doc = "Bit 3 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif3(&mut self) -> FPIF3_W<3> {
        FPIF3_W::new(self)
    }
    #[doc = "Bit 4 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif4(&mut self) -> FPIF4_W<4> {
        FPIF4_W::new(self)
    }
    #[doc = "Bit 5 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif5(&mut self) -> FPIF5_W<5> {
        FPIF5_W::new(self)
    }
    #[doc = "Bit 6 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif6(&mut self) -> FPIF6_W<6> {
        FPIF6_W::new(self)
    }
    #[doc = "Bit 7 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif7(&mut self) -> FPIF7_W<7> {
        FPIF7_W::new(self)
    }
    #[doc = "Bit 8 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif8(&mut self) -> FPIF8_W<8> {
        FPIF8_W::new(self)
    }
    #[doc = "Bit 9 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif9(&mut self) -> FPIF9_W<9> {
        FPIF9_W::new(self)
    }
    #[doc = "Bit 10 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif10(&mut self) -> FPIF10_W<10> {
        FPIF10_W::new(self)
    }
    #[doc = "Bit 11 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif11(&mut self) -> FPIF11_W<11> {
        FPIF11_W::new(self)
    }
    #[doc = "Bit 12 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif12(&mut self) -> FPIF12_W<12> {
        FPIF12_W::new(self)
    }
    #[doc = "Bit 13 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif13(&mut self) -> FPIF13_W<13> {
        FPIF13_W::new(self)
    }
    #[doc = "Bit 14 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif14(&mut self) -> FPIF14_W<14> {
        FPIF14_W::new(self)
    }
    #[doc = "Bit 15 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif15(&mut self) -> FPIF15_W<15> {
        FPIF15_W::new(self)
    }
    #[doc = "Bit 16 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif16(&mut self) -> FPIF16_W<16> {
        FPIF16_W::new(self)
    }
    #[doc = "Bit 17 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif17(&mut self) -> FPIF17_W<17> {
        FPIF17_W::new(self)
    }
    #[doc = "Bit 18 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif18(&mut self) -> FPIF18_W<18> {
        FPIF18_W::new(self)
    }
    #[doc = "Bit 20 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn fpif20(&mut self) -> FPIF20_W<20> {
        FPIF20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr1](index.html) module"]
pub struct FPR1_SPEC;
impl crate::RegisterSpec for FPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpr1::R](R) reader structure"]
impl crate::Readable for FPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpr1::W](W) writer structure"]
impl crate::Writable for FPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPR1 to value 0"]
impl crate::Resettable for FPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
