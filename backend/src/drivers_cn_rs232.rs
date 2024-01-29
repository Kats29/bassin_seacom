use std::fs::{File, OpenOptions};
use i2c_linux::I2c;
#[derive(Debug)]
pub enum I2cAddr {
    AddrXE,
    AddrYE,
    AddrZE,
    AddrTE,
    AddrXR,
    AddrYR,
    AddrZR,
    AddrTR,
}

pub fn get_i2c_addr_value(i2c_addr: I2cAddr) -> u8 {
    return match i2c_addr {
        I2cAddr::AddrXE => ADDR_X_E,
        I2cAddr::AddrYE => ADDR_Y_E,
        I2cAddr::AddrZE => ADDR_Z_E,
        I2cAddr::AddrTE => ADDR_T_E,

        I2cAddr::AddrXR => ADDR_X_R,
        I2cAddr::AddrYR => ADDR_Y_R,
        I2cAddr::AddrZR => ADDR_Z_R,
        I2cAddr::AddrTR => ADDR_T_R
    }
}

const ADDR_X_E: u8 = 0x39;
const ADDR_Y_E: u8= 0x39;
const ADDR_Z_E: u8 = 0x39;
const ADDR_T_E: u8 = 0x39;

const ADDR_X_R: u8 = 0x39;
const ADDR_Y_R: u8= 0x39;
const ADDR_Z_R: u8 = 0x39;
const ADDR_T_R: u8 = 0x39;

#[derive(Clone,Copy)]
pub struct DriversCnRs232{
    i2c_handler: Option<I2c<File>>
}

impl Default for DriversCnRs232{
    fn default() -> Self{
        Self{
            i2c_handler: None
        }
    }
}


impl DriversCnRs232{
    pub fn new() -> std::io::Result<Self>{
        let mut driver = Self::default();
        let i2c_file = OpenOptions::new().read(true).write(true).open("/dev/i2c-1")?;
        let i2c= I2c::new(i2c_file);
        driver.i2c_handler = Some(i2c);
        Ok(driver)
    }

    pub fn write_i2c(self, data: &[u8],addr_cn : I2cAddr) -> std::io::Result<()>{
        let i2c_addr = get_i2c_addr_value(addr_cn);

        println!("set addr slave to :{}",i2c_addr);
        //self.i2c_handler?.smbus_set_slave_address(i2c_addr as u16, false).expect(&*format!("Probleme lors du set slave addresse pour {:?}", addr_cn));


        for n in data.into_iter(){
            println!("command the slave to write to RS232 :{}",i2c_addr);
            //self.i2c_handler?.smbus_write_byte(0x74)?;
            println!("write the data {}",n);
            //self.i2c_handler?.smbus_write_byte(*n)?;

            println!("Loop while the slave don't have 1 byte of new data");

            /*let mut result = true;
            while result {
                self.i2c_handler?.smbus_write_byte(0x63)?;
                self.i2c_handler?.smbus_read_byte()?;
                let count = self.i2c_handler?.smbus_read_byte()?;
                let new = self.i2c_handler?.smbus_read_byte()?;
                if count == 1 && new&(1)==1 {
                    result = false;
                }
            }*/
        }
        Ok(())
    }
}