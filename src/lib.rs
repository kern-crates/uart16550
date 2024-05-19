//! Provide definition of 16550 uart registers.

#![no_std]
#![deny(warnings, missing_docs)]
#![forbid(unsafe_code)]
extern crate alloc;

mod fcr;
mod ier;
mod iir;
mod lcr;
mod lsr;
mod mcr;
mod msr;
mod rbr_thr;

use alloc::boxed::Box;
use core::fmt::Debug;
use core::marker::PhantomData;

pub use fcr::{FifoControl, TriggerLevel};
pub use ier::InterruptTypes;
pub use iir::{InterruptIdentification, PendingInterrupt};
pub use lcr::{CharLen, LineControl, PARITY};
pub use lsr::LineStatus;
pub use mcr::ModemControl;
pub use msr::ModemStatus;

/// 寄存器特质。
///
/// 由于 16550 设计历史悠久，它的实现有 8 位寄存器和 32 位寄存器两种模式，
/// 在驱动中用这个特质来描述。但无论哪种模式，只要是兼容 16550 定义，就只有 8 位是有效的。
pub trait Register: From<u8> {
    /// 取出寄存器中的有效位。
    fn val(self) -> u8;
}

/// 寄存器的 8 位模式。
impl Register for u8 {
    #[inline]
    fn val(self) -> u8 {
        self
    }
}

/// 寄存器的 32 位模式。
impl Register for u32 {
    #[inline]
    fn val(self) -> u8 {
        self as _
    }
}

macro_rules! gen_reg {
    ($name:ident) => {
        /// Register for $name.
        #[allow(non_camel_case_types)]
        pub struct $name<R: Register> {
            /// Offset of the register.
            offset: usize,
            /// Phantom data.
            phantom_data: PhantomData<R>,
        }
    };
}

// 接收缓冲寄存器和发送保持寄存器。
gen_reg!(RBR_THR);
// 中断使能寄存器。
gen_reg!(IER);

// 中断识别寄存器和队列控制寄存器。
gen_reg!(IIR_FCR);

// 线路控制寄存器。
gen_reg!(LCR);
// 调制解调器控制寄存器。
gen_reg!(MCR);
// 线路状态寄存器。
gen_reg!(LSR);
// 调制解调器状态寄存器。
gen_reg!(MSR);

/// 工作状态的 uart16550 数据结构。
#[repr(C)]
pub struct Uart16550<R: Register> {
    rbr_thr: RBR_THR<R>, // offset = 0(0x00)
    ier: IER<R>,         // offset = 1(0x04)
    iir_fcr: IIR_FCR<R>, // offset = 2(0x08)
    lcr: LCR<R>,         // offset = 3(0x0c)
    mcr: MCR<R>,         // offset = 4(0x10)
    lsr: LSR<R>,         // offset = 5(0x14)
    msr: MSR<R>,         // offset = 6(0x18)
    io_region: Box<dyn Uart16550IO<R>>,
}
/// The trait for uart16550 io.
/// User should implement this trait to use uart16550.
pub trait Uart16550IO<R: Register>: Debug + Send + Sync {
    /// Read from the register at the offset.
    fn read_at(&self, offset: usize) -> R;
    /// Write to the register at the offset.
    fn write_at(&self, offset: usize, value: R);
}

impl<R: Register> Uart16550IO<R> for Box<dyn Uart16550IO<R>> {
    fn read_at(&self, offset: usize) -> R {
        self.as_ref().read_at(offset)
    }

    fn write_at(&self, offset: usize, value: R) {
        self.as_ref().write_at(offset, value)
    }
}

impl<R: Register> Uart16550<R> {
    /// Create a new Uart16550 instance.
    pub fn new(io_region: Box<dyn Uart16550IO<R>>) -> Self {
        Self {
            rbr_thr: RBR_THR {
                offset: 0,
                phantom_data: PhantomData,
            },
            ier: IER {
                offset: core::mem::size_of::<R>(),
                phantom_data: PhantomData,
            },
            iir_fcr: IIR_FCR {
                offset: 2 * core::mem::size_of::<R>(),
                phantom_data: PhantomData,
            },
            lcr: LCR {
                offset: 3 * core::mem::size_of::<R>(),
                phantom_data: PhantomData,
            },
            mcr: MCR {
                offset: 4 * core::mem::size_of::<R>(),
                phantom_data: PhantomData,
            },
            lsr: LSR {
                offset: 5 * core::mem::size_of::<R>(),
                phantom_data: PhantomData,
            },
            msr: MSR {
                offset: 6 * core::mem::size_of::<R>(),
                phantom_data: PhantomData,
            },
            io_region,
        }
    }

    /// 取出 IO 区域。
    pub fn io_region(&self) -> &dyn Uart16550IO<R> {
        self.io_region.as_ref()
    }

    /// 取出接收缓冲和发送保持寄存器。
    #[inline]
    pub fn rbr_thr(&self) -> &RBR_THR<R> {
        &self.rbr_thr
    }

    /// 取出中断使能寄存器。
    #[inline]
    pub fn ier(&self) -> &IER<R> {
        &self.ier
    }

    /// 取出中断识别和队列控制寄存器。
    #[inline]
    pub fn iir_fcr(&self) -> &IIR_FCR<R> {
        &self.iir_fcr
    }

    /// 取出线路控制寄存器。
    #[inline]
    pub fn lcr(&self) -> &LCR<R> {
        &self.lcr
    }

    /// 取出调制解调器控制寄存器。
    #[inline]
    pub fn mcr(&self) -> &MCR<R> {
        &self.mcr
    }

    /// 取出线路状态寄存器。
    #[inline]
    pub fn lsr(&self) -> &LSR<R> {
        &self.lsr
    }

    /// 取出调制解调器状态寄存器。
    #[inline]
    pub fn msr(&self) -> &MSR<R> {
        &self.msr
    }

    /// 将分频系数写入锁存器。
    pub fn write_divisor(&self, divisor: u16) {
        let io_region = self.io_region();
        let lcr = self.lcr().read(io_region);
        self.lcr().write(io_region, lcr.enable_dlr_access());

        self.rbr_thr().write(io_region, R::from(divisor as _));
        self.ier()
            .write_divisor(io_region, R::from((divisor >> 8) as _));

        self.lcr().write(io_region, lcr);
    }

    /// 从接收队列读取字符到 `buf`，返回读取的字符数。
    pub fn read(&self, buf: &mut [u8]) -> usize {
        let io_region = self.io_region();
        let mut count = 0usize;
        for c in buf {
            if self.lsr().read(io_region).is_data_ready() {
                *c = self.rbr_thr().rx_data(io_region);
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    /// 从 `buf` 写入字符到发送队列，返回写入的字符数。
    pub fn write(&self, buf: &[u8]) -> usize {
        let io_region = self.io_region();
        let mut count = 0usize;
        for c in buf {
            if self.lsr().read(io_region).is_transmitter_fifo_empty() {
                self.rbr_thr().tx_data(io_region, *c);
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
