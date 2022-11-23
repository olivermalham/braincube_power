use cortex_m::asm::delay;

// Command codes:
const COL_ADDR_SET: u8 = 0x2A;
const ROW_ADDR_SET: u8 = 0x2B;
const MEM_WR: u8 = 0x2C;
const COLOR_MODE: u8 = 0x3A;
const COLOR_MODE__12_BIT: u8 = 0x03;
const COLOR_MODE__16_BIT: u8 = 0x05;
const COLOR_MODE__18_BIT: u8 = 0x06;
const MEM_WR_CONT: u8 = 0x3C;


pub fn gc9a01_delay(ms: u32) {
    delay(ms);
}

pub fn gc9a01_set_reset(val: u8) {
    // digitalWrite(RES, val);
}

pub fn gc9a01_set_data_command(val: u8) {
    // digitalWrite(DC, val);
}

pub fn gc9a01_set_chip_select(val: u8) {
    // digitalWrite(CS, val);
}

pub fn gc9a01_spi_tx(data: &[u8]) {
    // data needs to be a vector
    for i in 0..data.len(){
        // SPI.transfer(data[i]);
    }
}

pub fn gc9a01_write_command(cmd: u8) {
    gc9a01_set_data_command(0);
    gc9a01_set_chip_select(0);
    gc9a01_spi_tx(&[cmd]);
    gc9a01_set_chip_select(1);
}

pub fn gc9a01_write_data(data: &[u8]) {
    gc9a01_set_data_command(1);
    gc9a01_set_chip_select(0);
    gc9a01_spi_tx(data);
    gc9a01_set_chip_select(1);
}

pub fn gc9a01_write_byte(val: u8) {
    gc9a01_write_data(&[val]);
}

pub fn gc9a01_init() {

    gc9a01_set_chip_select(1);
    gc9a01_delay(5);
    gc9a01_set_reset(0);
    gc9a01_delay(10);
    gc9a01_set_reset(1);
    gc9a01_delay(120);

    /* Initial Sequence */

    gc9a01_write_command(0xEF);

    gc9a01_write_command(0xEB);
    gc9a01_write_byte(0x14);

    gc9a01_write_command(0xFE);
    gc9a01_write_command(0xEF);

    gc9a01_write_command(0xEB);
    gc9a01_write_byte(0x14);

    gc9a01_write_command(0x84);
    gc9a01_write_byte(0x40);

    gc9a01_write_command(0x85);
    gc9a01_write_byte(0xFF);

    gc9a01_write_command(0x86);
    gc9a01_write_byte(0xFF);

    gc9a01_write_command(0x87);
    gc9a01_write_byte(0xFF);

    gc9a01_write_command(0x88);
    gc9a01_write_byte(0x0A);

    gc9a01_write_command(0x89);
    gc9a01_write_byte(0x21);

    gc9a01_write_command(0x8A);
    gc9a01_write_byte(0x00);

    gc9a01_write_command(0x8B);
    gc9a01_write_byte(0x80);

    gc9a01_write_command(0x8C);
    gc9a01_write_byte(0x01);

    gc9a01_write_command(0x8D);
    gc9a01_write_byte(0x01);

    gc9a01_write_command(0x8E);
    gc9a01_write_byte(0xFF);

    gc9a01_write_command(0x8F);
    gc9a01_write_byte(0xFF);


    gc9a01_write_command(0xB6);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x00);

    gc9a01_write_command(0x36);

    // ORIENTATION == 2
    gc9a01_write_byte(0x48);

    gc9a01_write_command(COLOR_MODE);
    gc9a01_write_byte(COLOR_MODE__18_BIT);

    gc9a01_write_command(0x90);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x08);

    gc9a01_write_command(0xBD);
    gc9a01_write_byte(0x06);

    gc9a01_write_command(0xBC);
    gc9a01_write_byte(0x00);

    gc9a01_write_command(0xFF);
    gc9a01_write_byte(0x60);
    gc9a01_write_byte(0x01);
    gc9a01_write_byte(0x04);

    gc9a01_write_command(0xC3);
    gc9a01_write_byte(0x13);
    gc9a01_write_command(0xC4);
    gc9a01_write_byte(0x13);

    gc9a01_write_command(0xC9);
    gc9a01_write_byte(0x22);

    gc9a01_write_command(0xBE);
    gc9a01_write_byte(0x11);

    gc9a01_write_command(0xE1);
    gc9a01_write_byte(0x10);
    gc9a01_write_byte(0x0E);

    gc9a01_write_command(0xDF);
    gc9a01_write_byte(0x21);
    gc9a01_write_byte(0x0c);
    gc9a01_write_byte(0x02);

    gc9a01_write_command(0xF0);
    gc9a01_write_byte(0x45);
    gc9a01_write_byte(0x09);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x26);
    gc9a01_write_byte(0x2A);

    gc9a01_write_command(0xF1);
    gc9a01_write_byte(0x43);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x72);
    gc9a01_write_byte(0x36);
    gc9a01_write_byte(0x37);
    gc9a01_write_byte(0x6F);

    gc9a01_write_command(0xF2);
    gc9a01_write_byte(0x45);
    gc9a01_write_byte(0x09);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x26);
    gc9a01_write_byte(0x2A);

    gc9a01_write_command(0xF3);
    gc9a01_write_byte(0x43);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x72);
    gc9a01_write_byte(0x36);
    gc9a01_write_byte(0x37);
    gc9a01_write_byte(0x6F);

    gc9a01_write_command(0xED);
    gc9a01_write_byte(0x1B);
    gc9a01_write_byte(0x0B);

    gc9a01_write_command(0xAE);
    gc9a01_write_byte(0x77);

    gc9a01_write_command(0xCD);
    gc9a01_write_byte(0x63);

    gc9a01_write_command(0x70);
    gc9a01_write_byte(0x07);
    gc9a01_write_byte(0x07);
    gc9a01_write_byte(0x04);
    gc9a01_write_byte(0x0E);
    gc9a01_write_byte(0x0F);
    gc9a01_write_byte(0x09);
    gc9a01_write_byte(0x07);
    gc9a01_write_byte(0x08);
    gc9a01_write_byte(0x03);

    gc9a01_write_command(0xE8);
    gc9a01_write_byte(0x34);

    gc9a01_write_command(0x62);
    gc9a01_write_byte(0x18);
    gc9a01_write_byte(0x0D);
    gc9a01_write_byte(0x71);
    gc9a01_write_byte(0xED);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x18);
    gc9a01_write_byte(0x0F);
    gc9a01_write_byte(0x71);
    gc9a01_write_byte(0xEF);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x70);

    gc9a01_write_command(0x63);
    gc9a01_write_byte(0x18);
    gc9a01_write_byte(0x11);
    gc9a01_write_byte(0x71);
    gc9a01_write_byte(0xF1);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x18);
    gc9a01_write_byte(0x13);
    gc9a01_write_byte(0x71);
    gc9a01_write_byte(0xF3);
    gc9a01_write_byte(0x70);
    gc9a01_write_byte(0x70);

    gc9a01_write_command(0x64);
    gc9a01_write_byte(0x28);
    gc9a01_write_byte(0x29);
    gc9a01_write_byte(0xF1);
    gc9a01_write_byte(0x01);
    gc9a01_write_byte(0xF1);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x07);

    gc9a01_write_command(0x66);
    gc9a01_write_byte(0x3C);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0xCD);
    gc9a01_write_byte(0x67);
    gc9a01_write_byte(0x45);
    gc9a01_write_byte(0x45);
    gc9a01_write_byte(0x10);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x00);

    gc9a01_write_command(0x67);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x3C);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x01);
    gc9a01_write_byte(0x54);
    gc9a01_write_byte(0x10);
    gc9a01_write_byte(0x32);
    gc9a01_write_byte(0x98);

    gc9a01_write_command(0x74);
    gc9a01_write_byte(0x10);
    gc9a01_write_byte(0x85);
    gc9a01_write_byte(0x80);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x00);
    gc9a01_write_byte(0x4E);
    gc9a01_write_byte(0x00);

    gc9a01_write_command(0x98);
    gc9a01_write_byte(0x3e);
    gc9a01_write_byte(0x07);

    gc9a01_write_command(0x35);
    gc9a01_write_command(0x21);

    gc9a01_write_command(0x11);
    gc9a01_delay(120);
    gc9a01_write_command(0x29);
    gc9a01_delay(20);
}

pub fn gc9a01_set_frame(start_x: i16, end_x: i16, start_y: i16, end_y: i16) {

    let mut data: [u8; 4] = [0; 4];

    gc9a01_write_command(COL_ADDR_SET);
    data[0] = (start_x >> 8) as u8 & 0xFF;
    data[1] = start_x as u8 & 0xFF;
    data[2] = (end_x >> 8) as u8 & 0xFF;
    data[3] = end_x as u8 & 0xFF;
    gc9a01_write_data(&data);

    gc9a01_write_command(ROW_ADDR_SET);
    data[0] = (start_y >> 8) as u8 & 0xFF;
    data[1] = start_y as u8 & 0xFF;
    data[2] = (end_y >> 8) as u8 & 0xFF;
    data[3] = end_y as u8 & 0xFF;
    gc9a01_write_data(&data);
}

pub fn gc9a01_write(data: &[u8]) {
    // data needs to be a vector
    gc9a01_write_command(MEM_WR);
    gc9a01_write_data(data);
}

pub fn gc9a01_write_continue(data: &[u8]) {
    // data needs to be a vector
    gc9a01_write_command(MEM_WR_CONT);
    gc9a01_write_data(data);
}