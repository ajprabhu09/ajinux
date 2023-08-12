use core::{char, ops::DerefMut};

use crate::asm::{self, add, iodelay};

/// TODO: add more robust checks for api

/* --- CRTC Register Manipulation --- */
const CRTC_IDX_REG: u16 = 0x3d4;
const CRTC_DATA_REG: u16 = 0x3d5;
const CRTC_CURSOR_LSB_IDX: u8 = 15;
const CRTC_CURSOR_MSB_IDX: u8 = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Text {
    ascii: u8,
    style: u8,
}
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    /// 3:0 - FG 6:4 - BG  7- BLINK
    /// WARNING: This means only 0 - 7 can be used for background
    /// TODO: refactor to be more type safe
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

type PackedColor = u8;

impl Color {
    pub fn pack(left: Color, right: Color) -> PackedColor {
        let left = (left as u8) << 4;
        let right = right as u8;
        return left | right;
    }
    pub fn unpack(packed: PackedColor) -> (Color, Color) {
        let fg_mask = 0xF as u8;
        let bg_mask = fg_mask << 4;
        let fg = packed & fg_mask;
        let bg = (packed & bg_mask) >> 4;
        let fg: Color = unsafe { core::mem::transmute(fg) };
        let bg: Color = unsafe { core::mem::transmute(bg) };

        return (bg, fg);
    }
}

impl Text {
    pub fn colored(v: u8, fg: Color, bg: Color) -> Self {
        Self {
            ascii: v,
            style: Color::pack(bg, fg),
        }
    }
    pub fn from(v: u8) -> Self {
        Self::colored(v, DEFAULT_BG_COLOR, DEFAULT_FG_COLOR)
    }
    pub fn raw(v: u8, color: u8) -> Self {
        Self {
            ascii: v,
            style: color,
        }
    }
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct VGABuffer {
    pub buffer: [[Text; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl VGABuffer {
    pub fn mmio<'b>(addr: *mut u8) -> &'b mut Self {
        unsafe { &mut *(addr as *mut Self) }
    }
    pub fn set_at(&mut self, loc: (i32, i32), val: Text) {
        let addr = core::ptr::addr_of_mut!(self.buffer[loc.0 as usize][loc.1 as usize]);
        unsafe{addr.write_volatile(val);}
    }
    pub fn get_at(&self, loc: (i32, i32)) -> Text {
        let addr = core::ptr::addr_of!(self.buffer[loc.0 as usize][loc.1 as usize]);
        return unsafe {
            addr.read_volatile()
        };
    }
}

pub struct VGADisplay {
    cursor_saved: (i32, i32),
    buffer: &'static mut VGABuffer,
    curr_fg_color: Color,
    curr_bg_color: Color,
}

pub const DEFAULT_BG_COLOR: Color = Color::Black;
pub const DEFAULT_FG_COLOR: Color = Color::Cyan;

type ConsoleErrType = &'static str;
pub trait ConsoleDisplay {
    fn put_byte(&mut self, ch: u8) -> Result<(), ConsoleErrType>;
    fn put_bytes(&mut self, ch: &[u8]) -> Result<(), ConsoleErrType>;
    fn draw_char(&mut self, loc: (i32, i32), ch: u8, color: PackedColor);
    fn get_char(&mut self, loc: (i32, i32)) -> Result<u8, ConsoleErrType>;
    fn set_term_color(&mut self, color: PackedColor);
    fn get_term_color(&mut self) -> Result<PackedColor, ConsoleErrType>;
    fn set_cursor(&mut self, loc: (i32, i32)) -> Result<(), ConsoleErrType>;
    fn get_cursor(&mut self) -> (i32, i32);
    fn hide_cursor(&mut self);
}
impl VGADisplay {
    /// Non buffered scrolling so all the previos data is lost
    pub fn scroll_down(&mut self, rows: usize) {
        if rows == 0 {
            return;
        }
        for i in 0..BUFFER_HEIGHT - rows {
            for j in 0..BUFFER_WIDTH {
                let loc0 = (i as i32, j as i32);
                let loc1 = ((i + rows) as i32, j as i32);
                self.buffer.set_at(loc0, self.buffer.get_at(loc1));
            }
        }
        for i in (BUFFER_HEIGHT - rows)..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                let loc0 = (i as i32, j as i32);
                self.buffer.set_at(loc0, Text::colored(b' ', self.curr_fg_color, self.curr_bg_color));
            }
        }
    }

    pub fn default() -> Self {
        Self {
            cursor_saved: (0, 0),
            buffer: VGABuffer::mmio(0xb8000 as *mut u8),
            curr_bg_color: DEFAULT_BG_COLOR,
            curr_fg_color: DEFAULT_FG_COLOR,
        }
    }
    pub fn restore_cursor(&mut self) {
        self.set_cursor(self.cursor_saved).unwrap();
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Component {
    lsb: u8,
    msb: u8,
}
#[repr(C)]
union TopLevel {
    word: u16,
    component: Component,
}

pub fn delay(n: usize) {
    for i in 0..n {
        unsafe { asm::iodelay() };
    }
}

pub fn bounds_check(loc: (i32, i32)) -> bool {
    return (0..BUFFER_HEIGHT).contains(&(loc.0 as usize))
        && (0..BUFFER_WIDTH).contains(&(loc.1 as usize));
}

impl ConsoleDisplay for VGADisplay {
    fn put_byte(&mut self, ch: u8) -> Result<(), ConsoleErrType> {
            let cursor = self.get_cursor();

            delay(100000);

            match ch {
                b'\n' => {

                    if cursor.0 == BUFFER_HEIGHT as i32 {
                        self.scroll_down(1);
                        self.set_cursor((cursor.0, 0))?;
                    } else {
                        self.set_cursor((cursor.0 + 1, 0))?;
                    }
                    
                }
                b'\x08' => {
                    // \b
                    if cursor.1 == 0 && cursor.0 == 0 {
                        // Nothing should happen here
                    } else if cursor.0 > 0 && cursor.1 == 0 {
                        // TODO: this is complicated to handle
                        // Either you double buffer the vga buffer and handle it correctly or write a very 
                        // complicated logic here
                        // YOU DECIDE.
                        // for now moving to end of the previous line
                        self.set_cursor((cursor.0 - 1, (BUFFER_WIDTH - 1) as i32))?;
                    } else {
                        self.set_cursor((cursor.0, cursor.1 - 1))?;
                        self.buffer.set_at(
                            (cursor.0, cursor.1 - 1),
                            Text::colored(b' ', self.curr_fg_color, self.curr_bg_color),
                        );
                    }
                    
                }
                b'\r' => {
                    self.set_cursor((cursor.0, 0))?;
                }
                _ => {
                    self.buffer.set_at(
                        cursor,
                        Text::colored(ch, self.curr_fg_color, self.curr_bg_color),
                    );
                    if cursor.1 == (BUFFER_WIDTH as i32) - 1 {
                        self.set_cursor((cursor.0 + 1, 0))?;
                        
                    } else {
                        self.set_cursor((cursor.0, cursor.1 + 1))?;

                    }
                }
            };

        Ok(())
    }
    fn put_bytes(&mut self, ch: &[u8]) -> Result<(), ConsoleErrType> {
        ch.iter().for_each(|ch| {
            self.put_byte(*ch);
            // delay(500000);
        });
        Ok(())
    }

    fn draw_char(&mut self, loc: (i32, i32), ch: u8, color: u8) {
        if !bounds_check(loc) {
            return;
        }
        unsafe { self.buffer.set_at(loc, Text::raw(ch, color)) }
    }

    fn get_char(&mut self, loc: (i32, i32)) -> Result<u8, ConsoleErrType> {
        if !bounds_check(loc) {
            return Err("outside display");
        }
        unsafe { Ok(self.buffer.get_at(loc).ascii) }
    }

    fn set_term_color(&mut self, color: PackedColor) {
        let (bg, fg) = Color::unpack(color);
        self.curr_bg_color = bg;
        self.curr_fg_color = fg;
    }

    fn get_term_color(&mut self) -> Result<PackedColor, ConsoleErrType> {
        Ok(Color::pack(self.curr_bg_color, self.curr_fg_color))
    }

    fn set_cursor(&mut self, loc: (i32, i32)) -> Result<(), ConsoleErrType> {
        let offset = (loc.0 as u16) * (BUFFER_WIDTH as u16) + loc.1 as u16;
        unsafe {
            asm::outb(CRTC_IDX_REG, CRTC_CURSOR_LSB_IDX);
            let toplevel = TopLevel { word: offset };
            asm::outb(CRTC_DATA_REG, toplevel.component.lsb);
            asm::iodelay();
            asm::outb(CRTC_IDX_REG, CRTC_CURSOR_MSB_IDX);
            asm::outb(CRTC_DATA_REG, toplevel.component.msb);
        }
        Ok(())
    }

    fn get_cursor(&mut self) -> (i32, i32) {
        unsafe {
            asm::outb(CRTC_IDX_REG, CRTC_CURSOR_LSB_IDX);
            let lsb = asm::inb(CRTC_DATA_REG);
            asm::iodelay();
            asm::outb(CRTC_IDX_REG, CRTC_CURSOR_MSB_IDX);
            let msb = asm::inb(CRTC_DATA_REG);
            let toplevl = TopLevel {
                component: Component { lsb, msb },
            };
            let merged = toplevl.word;
            let c = merged % (BUFFER_WIDTH as u16);
            let r = (merged - c) / (BUFFER_WIDTH as u16);
            return (r as i32, c as i32);
        }
    }

    fn hide_cursor(&mut self) {
        self.cursor_saved = self.get_cursor();
        self.set_cursor(((BUFFER_HEIGHT + 1) as i32, (BUFFER_WIDTH + 1) as i32))
            .unwrap();
    }
}
