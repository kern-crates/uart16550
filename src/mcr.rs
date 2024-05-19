use crate::{Register, Uart16550IO, MCR};

impl<R: Register> MCR<R> {
    /// 写入调制解调器控制设置。
    #[inline]
    pub fn write(&self, io_region: &dyn Uart16550IO<R>, val: ModemControl) {
        io_region.write_at(self.offset, R::from(val.0));
    }

    /// 读取调制解调器控制设置。
    #[inline]
    pub fn read(&self, io_region: &dyn Uart16550IO<R>) -> ModemControl {
        let val = io_region.read_at(self.offset).val();
        ModemControl(val)
    }
}

/// 调制解调器控制设置。
///
/// TODO: 一般用不到，未实现方法，可以取出值操作。
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct ModemControl(pub u8);
