use super::gpio::{GpioModule, PinMode};
use crate::{
    common::{Modules, Reg},
    DriverExt,
};

#[allow(dead_code)]
const STATUS: &Reg = &[Modules::Encoder.into_u8(), 0x00];
const INT_SET: &Reg = &[Modules::Encoder.into_u8(), 0x10];
const INT_CLR: &Reg = &[Modules::Encoder.into_u8(), 0x20];
const POSITION: &Reg = &[Modules::Encoder.into_u8(), 0x30];
const DELTA: &Reg = &[Modules::Encoder.into_u8(), 0x40];

pub trait EncoderModule<D: crate::Driver>: GpioModule<D> {
    const ENCODER_BTN_PIN: u8;

    async fn enable_button(&mut self) -> Result<(), crate::SeesawError<D::Error>> {
        self.set_pin_mode(Self::ENCODER_BTN_PIN, PinMode::InputPullup)
            .await?;
        self.driver().delay_us(125).await;
        Ok(())
    }

    async fn button(&mut self) -> Result<bool, crate::SeesawError<D::Error>> {
        self.digital_read(Self::ENCODER_BTN_PIN).await
    }

    async fn delta(&mut self) -> Result<i32, crate::SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .read_i32(addr, DELTA)
            .await
            .map_err(crate::SeesawError::I2c)
    }

    async fn disable_interrupt(&mut self) -> Result<(), crate::SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_CLR, 1)
            .await
            .map_err(crate::SeesawError::I2c)
    }

    async fn enable_interrupt(&mut self) -> Result<(), crate::SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_SET, 1)
            .await
            .map_err(crate::SeesawError::I2c)
    }

    async fn position(&mut self) -> Result<i32, crate::SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .read_i32(addr, POSITION)
            .await
            .map_err(crate::SeesawError::I2c)
    }

    async fn set_position(&mut self, pos: i32) -> Result<(), crate::SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_i32(addr, POSITION, pos)
            .await
            .map_err(crate::SeesawError::I2c)
    }
}
