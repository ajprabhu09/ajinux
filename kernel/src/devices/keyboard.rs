#![allow(dead_code)]
#![allow(unused)]
use bitfield_struct::bitfield;

use super::{port::Port, vga::ConsoleDisplay};
use crate::{datastructures::no_alloc::ringbuffer::RingBuf, io::writer::WRITER};
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Char(char),
    Esc,
    Err(&'static str),
    Enter,
    LCtrl,
    LShift,
    LAlt,
    CapsLock,
    RShift,
}
#[derive(Debug, Clone, Copy)]
pub enum KeyAction {
    Press(Key),
    Release(Key),
}

pub trait ConsoleInput {
    fn read_char(&mut self) -> Option<char>;
    fn read_line(&mut self, dest: &mut [char], len: usize);
}

impl ConsoleInput for Keyboard {
    fn read_char(&mut self) -> Option<char> {
        self.process_buf()
    }

    fn read_line(&mut self, dest: &mut [char], len: usize) {
        let mut idx = 0;
        loop {
            if idx == len {
                break;
            }
            let c = self.process_buf_wait();

            dest[idx] = c;
            if c == '\n' {
                break;
            }
            idx += 1;
        }
    }
}

pub const TOP_ROW: &'static str = "qwertyuiop[]";
pub const HOME_ROW: &'static str = "asdfghjkl;'`";
pub const BOTTOM_ROW: &'static str = "zxcvbnm,./";
pub const NUM_SHIFTER: &'static str = "~!@#$%^&*()_+";

fn map_val_to_key_scan_code_1(code: u8) -> KeyAction {
    use Key::*;
    let code2 = code & !(1 << 7);

    let key = match code2 {
        0x01 => Esc,                                          // escape pressed
        0x02..=0x0A => Char((code2 - 1 + '0' as u8) as char), // 1 pressed
        0x0B => Char('0'),                                    // 0 (zero) pressed
        0x0C => Char('-'),                                    // - pressed
        0x0D => Char('='),                                    // = pressed
        0x0E => Char('\x08'),                                 // backspace pressed TODO: check
        0x0F => Char('\t'),                                   // tab pressed
        0x10..=0x1B => Char(TOP_ROW.as_bytes()[(code2 - 0x10) as usize] as char), // Q pressed
        // 0x11 => , // W pressed
        // 0x12 => , // E pressed
        // 0x13 => , // R pressed
        // 0x14 => , // T pressed
        // 0x15 => , // Y pressed
        // 0x16 => , // U pressed
        // 0x17 => , // I pressed
        // 0x18 => , // O pressed
        // 0x19 => , // P pressed
        // 0x1A => , // [ pressed
        // 0x1B => , // ] pressed
        0x1C => Enter, // enter pressed
        0x1D => LCtrl, // left control pressed
        0x1E..=0x29 => Char(HOME_ROW.as_bytes()[(code2 - 0x1E) as usize] as char), // A pressed
        // 0x1F => , // S pressed
        // 0x20 => , // D pressed
        // 0x21 => , // F pressed
        // 0x22 => , // G pressed
        // 0x23 => , // H pressed
        // 0x24 => , // J pressed
        // 0x25 => , // K pressed
        // 0x26 => , // L pressed
        // 0x27 => , // ; pressed
        // 0x28 => , // ' (single quote) pressed
        // 0x29 => , // ` (back tick) pressed
        0x2A => LShift,     // left shift pressed
        0x2B => Char('\\'), // \ pressed
        0x2C..=0x35 => Char(BOTTOM_ROW.as_bytes()[(code2 - 0x2C) as usize] as char), // Z pressed
        // 0x2D => , // X pressed
        // 0x2E => , // C pressed
        // 0x2F => , // V pressed
        // 0x30 => , // B pressed
        // 0x31 => , // N pressed
        // 0x32 => , // M pressed
        // 0x33 => , // , pressed
        // 0x34 => , // . pressed
        // 0x35 => , // / pressed
        0x36 => RShift, // right shift pressed
        // 0x37 => , // (keypad) * pressed
        0x38 => LAlt,      // left alt pressed
        0x39 => Char(' '), // space pressed
        0x3A => CapsLock,  // CapsLock pressed
        // 0x3B => , // F1 pressed
        // 0x3C => , // F2 pressed
        // 0x3D => , // F3 pressed
        // 0x3E => , // F4 pressed
        // 0x3F => , // F5 pressed
        // 0x40 => , // F6 pressed
        // 0x41 => , // F7 pressed
        // 0x42 => , // F8 pressed
        // 0x43 => , // F9 pressed
        // 0x44 => , // F10 pressed
        // 0x45 => , // NumberLock pressed
        // 0x46 => , // ScrollLock pressed
        // 0x47 => , // (keypad) 7 pressed
        // 0x48 => , // (keypad) 8 pressed
        // 0x49 => , // (keypad) 9 pressed
        // 0x4A => , // (keypad) - pressed
        // 0x4B => , // (keypad) 4 pressed
        // 0x4C => , // (keypad) 5 pressed
        // 0x4D => , // (keypad) 6 pressed
        // 0x4E => , // (keypad) + pressed
        // 0x4F => , // (keypad) 1 pressed
        // 0x50 => , // (keypad) 2 pressed
        // 0x51 => , // (keypad) 3 pressed
        // 0x52 => , // (keypad) 0 pressed
        // 0x53 => , // (keypad) . pressed
        // 0x57 => , // F11 pressed
        // 0x58 => , // F12 pressed
        /// Key release are the same as press byt with 8th bit set
        // 0x81 => , // escape released
        // 0x82 => , // 1 released
        // 0x83 => , // 2 released
        // 0x84 => , // 3 released
        // 0x85 => , // 4 released
        // 0x86 => , // 5 released
        // 0x87 => , // 6 released
        // 0x88 => , // 7 released
        // 0x89 => , // 8 released
        // 0x8A => , // 9 released
        // 0x8B => , // 0 (zero) released
        // 0x8C => , // - released
        // 0x8D => , // = released
        // 0x8E => , // backspace released
        // 0x8F => , // tab released
        // 0x90 => , // Q released
        // 0x91 => , // W released
        // 0x92 => , // E released
        // 0x93 => , // R released
        // 0x94 => , // T released
        // 0x95 => , // Y released
        // 0x96 => , // U released
        // 0x97 => , // I released
        // 0x98 => , // O released
        // 0x99 => , // P released
        // 0x9A => , // [ released
        // 0x9B => , // ] released
        // 0x9C => , // enter released
        // 0x9D => , // left control released
        // 0x9E => , // A released
        // 0x9F => , // S released
        // 0xA0 => , // D released
        // 0xA1 => , // F released
        // 0xA2 => , // G released
        // 0xA3 => , // H released
        // 0xA4 => , // J released
        // 0xA5 => , // K released
        // 0xA6 => , // L released
        // 0xA7 => , // ; released
        // 0xA8 => , // ' (single quote) released
        // 0xA9 => , // ` (back tick) released
        // 0xAA => , // left shift released
        // 0xAB => , // \ released
        // 0xAC => , // Z released
        // 0xAD => , // X released
        // 0xAE => , // C released
        // 0xAF => , // V released
        // 0xB0 => , // B released
        // 0xB1 => , // N released
        // 0xB2 => , // M released
        // 0xB3 => , // , released
        // 0xB4 => , // . released
        // 0xB5 => , // / released
        // 0xB6 => , // right shift released
        // 0xB7 => , // (keypad) * released
        // 0xB8 => , // left alt released
        // 0xB9 => , // space released
        // 0xBA => , // CapsLock released
        // 0xBB => , // F1 released
        // 0xBC => , // F2 released
        // 0xBD => , // F3 released
        // 0xBE => , // F4 released
        // 0xBF => , // F5 released
        // 0xC0 => , // F6 released
        // 0xC1 => , // F7 released
        // 0xC2 => , // F8 released
        // 0xC3 => , // F9 released
        // 0xC4 => , // F10 released
        // 0xC5 => , // NumberLock released
        // 0xC6 => , // ScrollLock released
        // 0xC7 => , // (keypad) 7 released
        // 0xC8 => , // (keypad) 8 released
        // 0xC9 => , // (keypad) 9 released
        // 0xCA => , // (keypad) - released
        // 0xCB => , // (keypad) 4 released
        // 0xCC => , // (keypad) 5 released
        // 0xCD => , // (keypad) 6 released
        // 0xCE => , // (keypad) + released
        // 0xCF => , // (keypad) 1 released
        // 0xD0 => , // (keypad) 2 released
        // 0xD1 => , // (keypad) 3 released
        // 0xD2 => , // (keypad) 0 released
        // 0xD3 => , // (keypad) . released
        // 0xD7 => , // F11 released
        // 0xD8 => , // F12 released
        // 0xE0 => , // ,
        // 0x10 => , // (multimedia) previous track pressed
        // 0xE0 => , // ,
        // 0x19 => , // (multimedia) next track pressed
        // 0xE0 => , // ,
        // 0x1C => , // (keypad) enter pressed
        // 0xE0 => , // ,
        // 0x1D => , // right control pressed
        // 0xE0 => , // ,
        // 0x20 => , // (multimedia) mute pressed
        // 0xE0 => , // ,
        // 0x21 => , // (multimedia) calculator pressed
        // 0xE0 => , // ,
        // 0x22 => , // (multimedia) play pressed
        // 0xE0 => , // ,
        // 0x24 => , // (multimedia) stop pressed
        // 0xE0 => , // ,
        // 0x2E => , // (multimedia) volume down pressed
        // 0xE0 => , // ,
        // 0x30 => , // (multimedia) volume up pressed
        // 0xE0 => , // ,
        // 0x32 => , // (multimedia) WWW home pressed
        // 0xE0 => , // ,
        // 0x35 => , // (keypad) / pressed
        // 0xE0 => , // ,
        // 0x38 => , // right alt (or altGr) pressed
        // 0xE0 => , // ,
        // 0x47 => , // home pressed
        // 0xE0 => , // ,
        // 0x48 => , // cursor up pressed
        // 0xE0 => , // ,
        // 0x49 => , // page up pressed
        // 0xE0 => , // ,
        // 0x4B => , // cursor left pressed
        // 0xE0 => , // ,
        // 0x4D => , // cursor right pressed
        // 0xE0 => , // ,
        // 0x4F => , // end pressed
        // 0xE0 => , // ,
        // 0x50 => , // cursor down pressed
        // 0xE0 => , // ,
        // 0x51 => , // page down pressed
        // 0xE0 => , // ,
        // 0x52 => , // insert pressed
        // 0xE0 => , // ,
        // 0x53 => , // delete pressed
        // 0xE0 => , // ,
        // 0x5B => , // left GUI pressed
        // 0xE0 => , // ,
        // 0x5C => , // right GUI pressed
        // 0xE0 => , // ,
        // 0x5D => , // "apps" pressed
        // 0xE0 => , // ,
        // 0x5E => , // (ACPI) power pressed
        // 0xE0 => , // ,
        // 0x5F => , // (ACPI) sleep pressed
        // 0xE0 => , // ,
        // 0x63 => , // (ACPI) wake pressed
        // 0xE0 => , // ,
        // 0x65 => , // (multimedia) WWW search pressed
        // 0xE0 => , // ,
        // 0x66 => , // (multimedia) WWW favorites pressed
        // 0xE0 => , // ,
        // 0x67 => , // (multimedia) WWW refresh pressed
        // 0xE0 => , // ,
        // 0x68 => , // (multimedia) WWW stop pressed
        // 0xE0 => , // ,
        // 0x69 => , // (multimedia) WWW forward pressed
        // 0xE0 => , // ,
        // 0x6A => , // (multimedia) WWW back pressed
        // 0xE0 => , // ,
        // 0x6B => , // (multimedia) my computer pressed
        // 0xE0 => , // ,
        // 0x6C => , // (multimedia) email pressed
        // 0xE0 => , // ,
        // 0x6D => , // (multimedia) media select pressed
        // 0xE0 => , // ,
        // 0x90 => , // (multimedia) previous track released
        // 0xE0 => , // ,
        // 0x99 => , // (multimedia) next track released
        // 0xE0 => , // ,
        // 0x9C => , // (keypad) enter released
        // 0xE0 => , // ,
        // 0x9D => , // right control released
        // 0xE0 => , // ,
        // 0xA0 => , // (multimedia) mute released
        // 0xE0 => , // ,
        // 0xA1 => , // (multimedia) calculator released
        // 0xE0 => , // ,
        // 0xA2 => , // (multimedia) play released
        // 0xE0 => , // ,
        // 0xA4 => , // (multimedia) stop released
        // 0xE0 => , // ,
        // 0xAE => , // (multimedia) volume down released
        // 0xE0 => , // ,
        // 0xB0 => , // (multimedia) volume up released
        // 0xE0 => , // ,
        // 0xB2 => , // (multimedia) WWW home released
        // 0xE0 => , // ,
        // 0xB5 => , // (keypad) / released
        // 0xE0 => , // ,
        // 0xB8 => , // right alt (or altGr) released
        // 0xE0 => , // ,
        // 0xC7 => , // home released
        // 0xE0 => , // ,
        // 0xC8 => , // cursor up released
        // 0xE0 => , // ,
        // 0xC9 => , // page up released
        // 0xE0 => , // ,
        // 0xCB => , // cursor left released
        // 0xE0 => , // ,
        // 0xCD => , // cursor right released
        // 0xE0 => , // ,
        // 0xCF => , // end released
        // 0xE0 => , // ,
        // 0xD0 => , // cursor down released
        // 0xE0 => , // ,
        // 0xD1 => , // page down released
        // 0xE0 => , // ,
        // 0xD2 => , // insert released
        // 0xE0 => , // ,
        // 0xD3 => , // delete released
        // 0xE0 => , // ,
        // 0xDB => , // left GUI released
        // 0xE0 => , // ,
        // 0xDC => , // right GUI released
        // 0xE0 => , // ,
        // 0xDD => , // "apps" released
        // 0xE0 => , // ,
        // 0xDE => , // (ACPI) power released
        // 0xE0 => , // ,
        // 0xDF => , // (ACPI) sleep released
        // 0xE0 => , // ,
        // 0xE3 => , // (ACPI) wake released
        // 0xE0 => , // ,
        // 0xE5 => , // (multimedia) WWW search released
        // 0xE0 => , // ,
        // 0xE6 => , // (multimedia) WWW favorites released
        // 0xE0 => , // ,
        // 0xE7 => , // (multimedia) WWW refresh released
        // 0xE0 => , // ,
        // 0xE8 => , // (multimedia) WWW stop released
        // 0xE0 => , // ,
        // 0xE9 => , // (multimedia) WWW forward released
        // 0xE0 => , // ,
        // 0xEA => , // (multimedia) WWW back released
        // 0xE0 => , // ,
        // 0xEB => , // (multimedia) my computer released
        // 0xE0 => , // ,
        // 0xEC => , // (multimedia) email released
        // 0xE0 => , // ,
        // 0xED => , // (multimedia) media select released
        // 0xE0 => , // ,
        // 0x2A => , // ,
        // 0xE0 => , // ,
        // 0x37 => , // print screen pressed
        // 0xE0 => , // ,
        // 0xB7 => , // ,
        // 0xE0 => , // ,
        // 0xAA => , // print screen released
        // 0xE1 => , // ,
        // 0x1D => , // ,
        // 0x45 => , // ,
        // 0xE1 => , // ,
        // 0x9D => , // ,
        // 0xC5 => , // pause pressed
        _ => Err("invalid scan code"),
    };
    use KeyAction::*;

    if (code & (1 << 7)) != 0 {
        Release(key)
    } else {
        Press(key)
    }

    // Err("()")
}

#[bitfield(u8)]
pub struct StatusReg {
    #[bits(1)]
    buffer_status: bool,
    #[bits(1)]
    input_buffer_status: bool,
    #[bits(1)]
    system_flag: bool,
    #[bits(1)]
    command_data: bool,
    #[bits(1)]
    unk_key_lock: bool,
    #[bits(1)]
    recv_timeout: bool,
    #[bits(1)]
    timeout_error: bool,
    #[bits(1)]
    parity_error: bool,
}

pub struct Keyboard {
    pub data_port: Port,
    pub status_cmd_port: Port, // status reg: R Cmd reg: W
    pub shifter: bool,
    pub caps_lock: bool,
    pub caps_lock_pressed: bool,
    buffer: RingBuf<KeyAction, 100>,
}

impl Keyboard {
    pub const fn default() -> Self {
        Self {
            data_port: Port(0x60),
            status_cmd_port: Port(0x64),
            shifter: false,
            caps_lock: false,
            caps_lock_pressed: false,
            buffer: RingBuf::new(),
        }
    }

    pub fn status_reg(&self) -> StatusReg {
        StatusReg::from(self.status_cmd_port.read_byte())
    }

    pub fn cmd_reg(&self, byt: u8) {
        self.status_cmd_port.send_byte(byt);
    }

    pub fn scan_code(&self) -> u8 {
        self.data_port.read_byte()
    }

    fn set_modifier(&mut self, action: KeyAction) -> Option<char> {
        use Key::*;
        use KeyAction::*;
        match action {
            Press(key) => match key {
                LShift | RShift => {
                    self.shifter = true;
                    None
                }
                CapsLock => {
                    if !self.caps_lock_pressed {
                        self.caps_lock_pressed = true;
                        self.caps_lock = !self.caps_lock;
                    }
                    None
                }
                LAlt | Esc | LCtrl => None,
                Char(c) => {
                    if self.caps_lock && self.shifter {
                        Some(c)
                    } else if self.caps_lock && !self.shifter || !self.caps_lock && self.shifter {
                        match c {
                            'a'..='z' => {
                                return Some(c.to_ascii_uppercase());
                            }
                            '0'..='9' => {
                                if c == '0' {
                                    return Some(')');
                                }
                                let idx = (c as u8) - ('0' as u8);
                                return Some(NUM_SHIFTER.as_bytes()[idx as usize] as char);
                            }
                            '`' => Some('~'),
                            '[' => Some('{'),
                            ']' => Some('}'),
                            '-' => Some('_'),
                            '=' => Some('+'),
                            ';' => Some(':'),
                            _ => Some(c),
                        }
                    } else {
                        Some(c)
                    }
                }
                Enter => Some('\n'),
                _ => None,
            },
            Release(key) => {
                match key {
                    LShift | RShift => {
                        self.shifter = false;
                        None
                    }
                    CapsLock => {
                        // Press handles it
                        self.caps_lock_pressed = false;
                        None
                    }
                    LAlt => None,
                    LCtrl => None,
                    Esc => None,
                    _ => None,
                }
            }
        }
    }
    pub fn read_key(&mut self) -> KeyAction {
        let scan = map_val_to_key_scan_code_1(self.scan_code());
        self.set_modifier(scan);
        scan
    }

    pub fn read_into_buf(&mut self) -> Result<(), &'static str> {
        let action = self.read_raw();
        self.buffer.push(action)
    }

    pub fn process_buf(&mut self) -> Option<char> {
        let action = self.buffer.take();
        let modifier = self.set_modifier(action?);
        return modifier;
    }

    pub fn process_buf_wait(&mut self) -> char {
        loop {
            let val = self.process_buf();
            if let Some(c) = val {
                WRITER.take().display.put_byte(c as u8);
                return c;
            }
        }
    }

    pub fn read_raw(&self) -> KeyAction {
        map_val_to_key_scan_code_1(self.scan_code())
    }
}
