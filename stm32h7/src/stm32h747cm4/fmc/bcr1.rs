#[doc = "Register `BCR1` reader"]
pub struct R(crate::R<BCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCR1` writer"]
pub struct W(crate::W<BCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR1_SPEC>;
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
impl From<crate::W<BCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MBKEN` reader - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus."]
pub type MBKEN_R = crate::BitReader<bool>;
#[doc = "Field `MBKEN` writer - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus."]
pub type MBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `MUXEN` reader - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:"]
pub type MUXEN_R = crate::BitReader<bool>;
#[doc = "Field `MUXEN` writer - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:"]
pub type MUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `MTYP` reader - Memory type These bits define the type of external memory attached to the corresponding memory bank:"]
pub type MTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTYP` writer - Memory type These bits define the type of external memory attached to the corresponding memory bank:"]
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `MWID` reader - Memory data bus width Defines the external memory device width, valid for all type of memories."]
pub type MWID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWID` writer - Memory data bus width Defines the external memory device width, valid for all type of memories."]
pub type MWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FACCEN` reader - Flash access enable This bit enables NOR Flash memory access operations."]
pub type FACCEN_R = crate::BitReader<bool>;
#[doc = "Field `FACCEN` writer - Flash access enable This bit enables NOR Flash memory access operations."]
pub type FACCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `BURSTEN` reader - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:"]
pub type BURSTEN_R = crate::BitReader<bool>;
#[doc = "Field `BURSTEN` writer - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:"]
pub type BURSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `WAITPOL` reader - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:"]
pub type WAITPOL_R = crate::BitReader<bool>;
#[doc = "Field `WAITPOL` writer - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:"]
pub type WAITPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `WAITCFG` reader - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:"]
pub type WAITCFG_R = crate::BitReader<bool>;
#[doc = "Field `WAITCFG` writer - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:"]
pub type WAITCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `WREN` reader - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `WAITEN` reader - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode."]
pub type WAITEN_R = crate::BitReader<bool>;
#[doc = "Field `WAITEN` writer - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode."]
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `EXTMOD` reader - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
pub type EXTMOD_R = crate::BitReader<bool>;
#[doc = "Field `EXTMOD` writer - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
pub type EXTMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
pub type ASYNCWAIT_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
pub type ASYNCWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `CPSIZE` reader - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved."]
pub type CPSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPSIZE` writer - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved."]
pub type CPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CBURSTRW` reader - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
pub type CBURSTRW_R = crate::BitReader<bool>;
#[doc = "Field `CBURSTRW` writer - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
pub type CBURSTRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `CCLKEN` reader - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)"]
pub type CCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CCLKEN` writer - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)"]
pub type CCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `WFDIS` reader - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
pub type WFDIS_R = crate::BitReader<bool>;
#[doc = "Field `WFDIS` writer - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
pub type WFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
#[doc = "Field `BMAP` reader - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register."]
pub type BMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BMAP` writer - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register."]
pub type BMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FMCEN` reader - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
pub type FMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FMCEN` writer - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus."]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Memory type These bits define the type of external memory attached to the corresponding memory bank:"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width Defines the external memory device width, valid for all type of memories."]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Flash access enable This bit enables NOR Flash memory access operations."]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode."]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved."]
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)"]
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
    #[inline(always)]
    pub fn wfdis(&self) -> WFDIS_R {
        WFDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register."]
    #[inline(always)]
    pub fn bmap(&self) -> BMAP_R {
        BMAP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus."]
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<0> {
        MBKEN_W::new(self)
    }
    #[doc = "Bit 1 - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:"]
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<1> {
        MUXEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Memory type These bits define the type of external memory attached to the corresponding memory bank:"]
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<2> {
        MTYP_W::new(self)
    }
    #[doc = "Bits 4:5 - Memory data bus width Defines the external memory device width, valid for all type of memories."]
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<4> {
        MWID_W::new(self)
    }
    #[doc = "Bit 6 - Flash access enable This bit enables NOR Flash memory access operations."]
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<6> {
        FACCEN_W::new(self)
    }
    #[doc = "Bit 8 - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:"]
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<8> {
        BURSTEN_W::new(self)
    }
    #[doc = "Bit 9 - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:"]
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<9> {
        WAITPOL_W::new(self)
    }
    #[doc = "Bit 11 - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:"]
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<11> {
        WAITCFG_W::new(self)
    }
    #[doc = "Bit 12 - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<12> {
        WREN_W::new(self)
    }
    #[doc = "Bit 13 - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode."]
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<13> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 14 - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10)."]
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<14> {
        EXTMOD_W::new(self)
    }
    #[doc = "Bit 15 - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol."]
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<15> {
        ASYNCWAIT_W::new(self)
    }
    #[doc = "Bits 16:18 - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved."]
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<16> {
        CPSIZE_W::new(self)
    }
    #[doc = "Bit 19 - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register."]
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<19> {
        CBURSTRW_W::new(self)
    }
    #[doc = "Bit 20 - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)"]
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W<20> {
        CCLKEN_W::new(self)
    }
    #[doc = "Bit 21 - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
    #[inline(always)]
    pub fn wfdis(&mut self) -> WFDIS_W<21> {
        WFDIS_W::new(self)
    }
    #[doc = "Bits 24:25 - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register."]
    #[inline(always)]
    pub fn bmap(&mut self) -> BMAP_W<24> {
        BMAP_W::new(self)
    }
    #[doc = "Bit 31 - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register."]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<31> {
        FMCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr1](index.html) module"]
pub struct BCR1_SPEC;
impl crate::RegisterSpec for BCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcr1::R](R) reader structure"]
impl crate::Readable for BCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcr1::W](W) writer structure"]
impl crate::Writable for BCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCR1 to value 0x30db"]
impl crate::Resettable for BCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30db
    }
}
