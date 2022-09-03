#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRDY` reader - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1."]
pub type DRDY_R = crate::BitReader<bool>;
#[doc = "Field `CECS` reader - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
pub type CECS_R = crate::BitReader<bool>;
#[doc = "Field `SECS` reader - Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 32 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d}) Both noise sources have delivered more than 32 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 16 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d})"]
pub type SECS_R = crate::BitReader<bool>;
#[doc = "Field `CEIS` reader - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CEIS_R = crate::BitReader<bool>;
#[doc = "Field `CEIS` writer - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CEIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `SEIS` reader - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SEIS_R = crate::BitReader<bool>;
#[doc = "Field `SEIS` writer - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SEIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1."]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 32 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d}) Both noise sources have delivered more than 32 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 16 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d})"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W<5> {
        CEIS_W::new(self)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W<6> {
        SEIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
