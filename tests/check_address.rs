macro_rules! gen {
    (
        registers = [
            $(
                $( #[$meta:meta] )*
                ($peripheral:ident.$reg:ident, $addr:expr)
            ),* $(,)*
        ];
    ) => {
        #[test]
        #[allow(unused_variables)]
        fn test() {
            let peripherals = unsafe { rza1::Peripherals::steal() };
            $(
                $( #[$meta] )*
                {
                    let addr = (&peripherals.$peripheral.$reg) as *const _;
                    println!("{} = {:p}", stringify!($peripheral.$reg), addr);
                    assert_eq!(addr as usize, $addr);
                }
            )*
        }
    };
}

gen! {
    registers = [
        // 5. LSI Internal Bus
        (INB.rmpr, 0xFCFE1A00),
        (INB.axirerrctl0, 0xFCFE1A30),
        (INB.axirerrclr3, 0xFCFE1A5C),
        // 6. Clock Pulse Generator
        (CPG.frqcr, 0xFCFE0010),
        (CPG.frqcr2, 0xFCFE0014),
        // 7. Interrupt Controller
        (INTC.icr0, 0xFCFEF800),
        (INTC.irqrr, 0xFCFEF804),
        (INTC.icdisr0, 0xE8201080),
        (INTC.ppi_status, 0xE8201D00),
        (INTC.spi_status1, 0xE8201D08),
        (INTC.iccabpr, 0xE820201C),
        (INTC.icciidr, 0xE82020FC),
        // 8. Bus State Controoller
        (BSC.cmncr, 0x3FFFC000),
        (BSC.cs0bcr, 0x3FFFC004),
        (BSC.tostr, 0x3FFFC080),
        (BSC.toenr, 0x3FFFC084),
        // 9. Direct Memory Access Controller
        (DMAC.n0sa_0, 0xE8200000),
        (DMAC.crsa_0, 0xE8200018),
        (DMAC.crla_0, 0xE820003C),
        (DMAC.dctrl_8_15, 0xE8200700),
        (DMAC.dmars0, 0xFCFE1000),
        (DMAC.dmars7, 0xFCFE101C),
        // 10. Multi-Function Timer Pulse Unit 2
        (MTU2.tcr_0, 0xFCFF0300),
        (MTU2.tbtm_0, 0xFCFF0326),
        (MTU2.tolbr, 0xFCFF0236),
        // 11. OS Timer
        (OSTM0.cmp, 0xFCFEC000 + 0x00),
        (OSTM0.ts, 0xFCFEC000 + 0x14),
        (OSTM0.ctl, 0xFCFEC000 + 0x20),
        (OSTM1.cmp, 0xFCFEC400 + 0x00),
        (OSTM1.ts, 0xFCFEC400 + 0x14),
        (OSTM1.ctl, 0xFCFEC400 + 0x20),
        // 12. Watchdog Timer
        (WDT.wtcnt, 0xFCFE0002),
        (WDT.wtcsr, 0xFCFE0000),
        (WDT.wrcsr, 0xFCFE0004),
        // 13. Realtime Clock
        (RTC.r64cnt, 0xFCFF1000),
        (RTC.rfrh, 0xFCFF102A),
        // 14. Serial Communication Interface with FIFO
        (SCIF0.scsmr, 0xE8007000),
        (SCIF0.scfsr, 0xE8007010),
        (SCIF0.scemr, 0xE8007028),
        (SCIF7.scsmr, 0xE800a800),
        (SCIF7.scfsr, 0xE800a810),
        (SCIF7.scemr, 0xE800a828),
        // 15. Serial Communication Interface
        (SCIM0.smr, 0xE800B000),
        (SCIM0.secr, 0xE800B00D),
        (SCIM1.smr, 0xE800B800),
        (SCIM1.secr, 0xE800B80D),
        // 16. Renesas Serial Peripheral Interface
        (RSPI0.spcr, 0xE800C800),
        (RSPI0.spbfdr, 0xE800C822),
        (RSPI4.spcr, 0xE800E800),
        (RSPI4.spbfdr, 0xE800E822),
        // 17. SPI Multi I/O Bus Controller
        (SPIBSC0.cmncr, 0x3FEFA000),
        (SPIBSC0.drdmcr, 0x3FEFA058),
        (SPIBSC0.spodly, 0x3FEFA068),
        (SPIBSC1.cmncr, 0x3FEFB000),
        (SPIBSC1.drdmcr, 0x3FEFB058),
        (SPIBSC1.spodly, 0x3FEFB068),
        // 18. I2C Bus Interface
        (RIIC0.cr1, 0xFCFEE000 + 0x0000),
        (RIIC0.sr2, 0xFCFEE000 + 0x0024),
        (RIIC0.drr, 0xFCFEE000 + 0x0040),
        (RIIC1.cr1, 0xFCFEE400 + 0x0000),
        (RIIC1.sr2, 0xFCFEE400 + 0x0024),
        (RIIC1.drr, 0xFCFEE400 + 0x0040),
        (RIIC2.cr1, 0xFCFEE800 + 0x0000),
        (RIIC2.sr2, 0xFCFEE800 + 0x0024),
        (RIIC2.drr, 0xFCFEE800 + 0x0040),
        (RIIC3.cr1, 0xFCFEEC00 + 0x0000),
        (RIIC3.sr2, 0xFCFEEC00 + 0x0024),
        (RIIC3.drr, 0xFCFEEC00 + 0x0040),
        // 19. Serial Sound Interface
        (SSIF0.ssicr, 0xE820B000),
        (SSIF0.ssifcsr, 0xE820B02C),
        (SSIF5.ssifcsr, 0xE820D82C),
        // 20. Media Local Bus
        (MLB.dccr, 0xE8034000),
        (MLB.lcbcr0, 0xE8034280),
        (MLB.lcbcr30, 0xE8034280 + 4 * 30),
        // 21. CAN Interface
        (RSCAN0.c0cfg, 0xE803A000 + 0x0000),
        (RSCAN0.cfsts10, 0xE803A000 + 0x01A0),
        (RSCAN0.tmid79, 0xE803A000 + 0x14F0),
        // 22. IEBus Controller
        (IEB.b0bcr, 0xFCFEF000 + 0x0000),
        (IEB.b0dr, 0xFCFEF000 + 0x0064),
        // 23. Renesas SPDIF Interface
        // (from Table 58.1 Register Addresses)
        (SPDIF.tlca, 0xE8012000),
        (SPDIF.rdad, 0xE8012034),
        // 24. CD-ROM Decoder
        (ROMDEC.cromen, 0xE8005000),
        (ROMDEC.strmdout0, 0xE8005204),
        // 25. LIN Interface
        (LIN0.rln3n_lwbr, 0xFCFE9000 + 0x01),
        (LIN0.rln3n_ldbr1, 0xFCFE9000 + 0x18),
        (LIN0.rln3n_ldbr8, 0xFCFE9000 + 0x1f),
        (LIN1.rln3n_lwbr, 0xFCFE9800 + 0x01),
        (LIN1.rln3n_ldbr1, 0xFCFE9800 + 0x18),
        (LIN1.rln3n_ldbr8, 0xFCFE9800 + 0x1f),
        // 26. Ethernet Controller
        (ETHER.arstr, 0xE8204800),
        (ETHER.cssmr, 0xE82034EC),
        // 27. A/D Converter
        (ADC.dra, 0xE8005800),
        (ADC.cmpsr, 0xE8005864),
        // 28. NAND Flash Memory Controller
        (FLCTL.flintdmacr, 0xFCFF4018),
        (FLCTL.fltrcr, 0xFCFF402C),
        // 29. USB2.0 Host/Function Module
        (USB200.syscfg0, 0xE8010000),
        (USB200.d1fifob7, 0xE801019C),
        (USB201.d1fifob7, 0xE820719C),
        // 30. Digital Video Decoder
        (DVDEC0.adccr1, 0xFCFFB808),
        (DVDEC0.adccr2, 0xFCFFBA84),
        (DVDEC1.adccr2, 0xFCFFA284),
        // 32. Video Display Controller 5 (2): Input Controller
        (VDC50.inp_update, 0xFCFF7400),
        (VDC50.inp_dly_adj, 0xFCFF7410),
        (VDC50.imgcnt_update, 0xFCFF7480),
        (VDC51.imgcnt_drc_reg, 0xFCFF94C0),
        // 33. Video Display Controller 5 (3): Scaler
        // TODO
        // 34. Video Display Controller 5 (4): Image Quality Improver
        // TODO
        // 35. Video Display Controller 5 (5): Image Synthesizer
        // TODO
        // 36. Video Display Controller 5 (6): Output Image Generator
        // TODO
        // 37. Video Display Controller 5 (7): Output Controller
        // TODO
        // 38. Video Display Controller 5 (8): System Controller
        // TODO
        // 40. LVDS Output Interface
        // TODO
        // 43. Display Out Comparison Unit
        // TODO
        // 45. JPEG Codec Unit
        // TODO
        // 46. Capture Engine Unit
        // TODO
        // 47. Pixel Format Converter
        // TODO
        // 48. SCUX
        // TODO
        // 49. Sound Generator
        // TODO
        // 51. MMC Host Interface
        // TODO
        // 52. Motor Control PWM Timer
        // TODO
        // 54. Ports
        (GPIO.p1, 0xFCFE3000 + 0x0000 + 1 * 4),
        (GPIO.p11, 0xFCFE3000 + 0x0000 + 11 * 4),
        (GPIO.pipc5, 0xFCFE3000 + 0x4200 + 5 * 4),
        (GPIO.jppr0, 0xFCFE7B00 + 0x020),
        (GPIO.jpibc0, 0xFCFE7B00 + 0x400),
        // 55. Power-Down Modes
        (CPG.stbcr1, 0xFCFE0020),
        (CPG.xtalctr, 0xFCFF1810),
    ];
}
