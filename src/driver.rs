#![allow(async_fn_in_trait)]
use crate::common::Reg;
use embedded_hal_async::delay;

const DELAY_US: u32 = 1400;

pub trait Driver: embedded_hal_async::i2c::I2c + delay::DelayUs {}
impl<T> Driver for T where T: embedded_hal_async::i2c::I2c + delay::DelayUs {}

macro_rules! impl_integer_write {
    ($fn:ident $nty:tt) => {
        async fn $fn(
            &mut self,
            addr: embedded_hal_1::i2c::SevenBitAddress,
            reg: &Reg,
            value: $nty,
        ) -> Result<(), Self::Error> {
            self.register_write(addr, reg, &<$nty>::to_be_bytes(value))
                .await
        }
    };
}

macro_rules! impl_integer_read {
    ($fn:ident $nty:tt) => {
        async fn $fn(
            &mut self,
            addr: embedded_hal_1::i2c::SevenBitAddress,
            reg: &Reg,
        ) -> Result<$nty, Self::Error> {
            self.register_read::<{ ($nty::BITS / 8) as usize }>(addr, reg)
                .await
                .map($nty::from_be_bytes)
        }
    };
}

pub trait DriverExt {
    type Error;

    async fn register_read<const N: usize>(
        &mut self,
        addr: embedded_hal_1::i2c::SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], Self::Error>;

    async fn register_write<const N: usize>(
        &mut self,
        addr: embedded_hal_1::i2c::SevenBitAddress,
        reg: &Reg,
        bytes: &[u8; N],
    ) -> Result<(), Self::Error>
    where
        [(); N + 2]: Sized;

    impl_integer_read! { read_u8 u8 }
    impl_integer_read! { read_u16 u16 }
    impl_integer_read! { read_u32 u32 }
    impl_integer_read! { read_u64 u64 }
    impl_integer_read! { read_i8 i8 }
    impl_integer_read! { read_i16 i16 }
    impl_integer_read! { read_i32 i32 }
    impl_integer_read! { read_i64 i64 }
    impl_integer_write! { write_u8 u8 }
    impl_integer_write! { write_u16 u16 }
    impl_integer_write! { write_u32 u32 }
    impl_integer_write! { write_u64 u64 }
    impl_integer_write! { write_i8 i8 }
    impl_integer_write! { write_i16 i16 }
    impl_integer_write! { write_i32 i32 }
    impl_integer_write! { write_i64 i64 }
}

impl<T: Driver> DriverExt for T {
    type Error = T::Error;

    async fn register_read<const N: usize>(
        &mut self,
        addr: embedded_hal_1::i2c::SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], Self::Error> {
        let mut buffer = [0u8; N];
        defmt::trace!(
            "[0x{:x}] Reading register 0x{:x} of length {}",
            addr,
            u16::from_be_bytes(reg.clone()),
            N
        );
        self.write(addr, reg).await?;
        self.delay_us(DELAY_US).await;
        self.read(addr, &mut buffer).await?;

        Ok(buffer)
    }

    async fn register_write<const N: usize>(
        &mut self,
        addr: embedded_hal_1::i2c::SevenBitAddress,
        reg: &Reg,
        bytes: &[u8; N],
    ) -> Result<(), Self::Error>
    where
        [(); N + 2]: Sized,
    {
        let mut buffer = [0u8; N + 2];
        buffer[0..2].copy_from_slice(reg);
        buffer[2..].copy_from_slice(bytes);

        defmt::trace!(
            "[0x{:x}] Writing register 0x{:x}, value={=[u8]:b}",
            addr,
            u16::from_be_bytes(reg.clone()),
            *bytes
        );

        self.write(addr, &buffer).await?;
        self.delay_us(DELAY_US).await;

        Ok(())
    }
}
