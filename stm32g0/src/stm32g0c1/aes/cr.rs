#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DATATYPE` reader - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATYPE` writer - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHMOD1` reader - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type CHMOD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMOD1` writer - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type CHMOD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CCFC` reader - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
pub type CCFC_R = crate::BitReader<bool>;
#[doc = "Field `CCFC` writer - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
pub type CCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRC` reader - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
pub type ERRC_R = crate::BitReader<bool>;
#[doc = "Field `ERRC` writer - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
pub type ERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CCFIE` reader - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
pub type CCFIE_R = crate::BitReader<bool>;
#[doc = "Field `CCFIE` writer - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
pub type CCFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAINEN` reader - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub type DMAINEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAINEN` writer - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub type DMAINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAOUTEN` reader - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub type DMAOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAOUTEN` writer - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub type DMAOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `GCMPH` reader - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
pub type GCMPH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GCMPH` writer - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
pub type GCMPH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHMOD2` reader - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type CHMOD2_R = crate::BitReader<bool>;
#[doc = "Field `CHMOD2` writer - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type CHMOD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `KEYSIZE` reader - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type KEYSIZE_R = crate::BitReader<bool>;
#[doc = "Field `KEYSIZE` writer - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub type KEYSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NPBLB` reader - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
pub type NPBLB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NPBLB` writer - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
pub type NPBLB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod1(&self) -> CHMOD1_R {
        CHMOD1_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod2(&self) -> CHMOD2_R {
        CHMOD2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<1> {
        DATATYPE_W::new(self)
    }
    #[doc = "Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    #[doc = "Bits 5:6 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod1(&mut self) -> CHMOD1_W<5> {
        CHMOD1_W::new(self)
    }
    #[doc = "Bit 7 - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W<7> {
        CCFC_W::new(self)
    }
    #[doc = "Bit 8 - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<8> {
        ERRC_W::new(self)
    }
    #[doc = "Bit 9 - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<9> {
        CCFIE_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<10> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<11> {
        DMAINEN_W::new(self)
    }
    #[doc = "Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<12> {
        DMAOUTEN_W::new(self)
    }
    #[doc = "Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<13> {
        GCMPH_W::new(self)
    }
    #[doc = "Bit 16 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod2(&mut self) -> CHMOD2_W<16> {
        CHMOD2_W::new(self)
    }
    #[doc = "Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<18> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<20> {
        NPBLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
