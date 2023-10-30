#![allow(async_fn_in_trait)]
#![no_std]
#![allow(const_evaluatable_unchecked, incomplete_features)]
#![feature(
    async_fn_in_trait,
    async_iter_from_iter,
    async_iterator,
    array_try_map,
    generic_const_exprs,
    try_blocks,
)]

// TODO improve the organization of the exports/visibility
mod common;
pub mod devices;
mod driver;
mod macros;
pub mod modules;
pub use common::*;
pub use devices::*;
pub use driver::*;

pub mod prelude {
    pub use super::{
        devices::*,
        driver::DriverExt,
        modules::{adc::*, encoder::*, gpio::*, neopixel::*, status::*, timer::*},
        SeesawDevice, SeesawDeviceInit,
    };
}

pub struct Seesaw {}

impl Seesaw {
    pub fn new() -> Self {
        Seesaw {}
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SeesawError<E> {
    /// I2C bus error
    I2c(E),
    /// Occurs when an invalid hardware ID is read
    InvalidHardwareId(u8),
}

pub trait SeesawDevice {
    type Error;
    type Driver: Driver;

    const DEFAULT_ADDR: u8;
    const HARDWARE_ID: HardwareId;
    const PRODUCT_ID: u16;

    fn new(addr: u8, driver: Self::Driver) -> Self;
    fn addr(&self) -> u8;
    fn driver(&mut self) -> &mut Self::Driver;
    fn new_with_default_addr(driver: Self::Driver) -> Self;
}

/// At startup, Seesaw devices typically have a unique set of initialization
/// calls to be made. e.g. for a Neokey1x4, we're need to enable the on-board
/// neopixel and also do some pin mode setting to get everything working.
/// All devices implement `DeviceInit` with a set of sensible defaults. You can
/// override the default initialization function with your own by calling
/// `Seesaw::connect_with` instead of `Seesaw::connect`.
pub trait SeesawDeviceInit<D: Driver>: SeesawDevice<Driver = D>
where
    Self: Sized,
{
    #[allow(async_fn_in_trait)]
    async fn init(self) -> Result<Self, Self::Error>;
}

/// Basic driver implementation, wrapping the provided I2C and delay objects
pub struct SeesawDriver<I2C, DELAY> {
    i2c: I2C,
    delay: DELAY,
}
impl<I2C: embedded_hal_async::i2c::ErrorType, DELAY: embedded_hal_async::delay::DelayUs>
    SeesawDriver<I2C, DELAY>
{
    pub fn new(i2c: I2C, delay: DELAY) -> Self {
        Self { i2c, delay }
    }
}
impl<I2C: embedded_hal_async::i2c::ErrorType, DELAY> embedded_hal_async::i2c::ErrorType
    for SeesawDriver<I2C, DELAY>
{
    type Error = I2C::Error;
}
impl<I2C: embedded_hal_async::i2c::I2c, DELAY> embedded_hal_async::i2c::I2c
    for SeesawDriver<I2C, DELAY>
{
    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.read(address, read).await
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(address, write).await
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c.write_read(address, write, read).await
    }

    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_1::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.i2c.transaction(address, operations).await
    }
}
impl<I2C, DELAY: embedded_hal_async::delay::DelayUs> embedded_hal_async::delay::DelayUs
    for SeesawDriver<I2C, DELAY>
{
    async fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us).await
    }

    async fn delay_ms(&mut self, ms: u32) {
        self.delay.delay_ms(ms).await
    }
}
