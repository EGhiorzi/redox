#![feature(box_syntax)]

extern crate orbital;

extern crate system;

use std::url::Url;
use std::{cmp, mem, ptr, slice};
use std::fs::File;
use std::io::{Result, Read, Write, Seek, SeekFrom};
use std::ops::DerefMut;
use std::to_num::ToNum;
use std::thread;

use system::error::{Error, ENOENT};
use system::scheme::{Packet, Scheme};

use orbital::event::Event;
use orbital::Point;
use orbital::Size;

use self::display::Display;
use self::session::Session;
use self::window::Window;

pub mod display;
pub mod session;
pub mod window;

pub static mut session_ptr: *mut Session = 0 as *mut Session;

/*
/// A window resource
pub struct Resource {
    /// The window
    pub window: Box<Window>,
    /// Seek point
    pub seek: usize,
}

impl Resource {
    pub fn dup(&self) -> Result<Box<Resource>> {
        Ok(box Resource {
            window: Window::new(self.window.point,
                                self.window.size,
                                self.window.title.clone()),
            seek: self.seek,
        })
    }

    /// Return the url of this resource
    pub fn path(&self) -> Result<String> {
        Ok(format!("orbital:///{}/{}/{}/{}/{}",
                   self.window.point.x,
                   self.window.point.y,
                   self.window.size.width,
                   self.window.size.height,
                   self.window.title))
    }

    /// Read data to buffer
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Read events from window
        let mut i = 0;
        while buf.len() - i >= mem::size_of::<Event>() {
            match self.window.poll() {
                Some(event) => {
                    unsafe { ptr::write(buf.as_ptr().offset(i as isize) as *mut Event, event) };
                    i += mem::size_of::<Event>();
                }
                None => break,
            }
        }

        Ok(i)
    }

    /// Write to resource
    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let content = &mut self.window.content;

        let size = cmp::min(content.size - self.seek, buf.len());
        unsafe {
            Display::copy_run(buf.as_ptr() as usize, content.offscreen + self.seek, size);
        }
        self.seek += size;

        Ok(size)
    }

    /// Seek
    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let end = self.window.content.size;

        self.seek = match pos {
            SeekFrom::Start(offset) => cmp::min(end as u64, cmp::max(0, offset)) as usize,
            SeekFrom::Current(offset) => {
                cmp::min(end as i64, cmp::max(0, self.seek as i64 + offset)) as usize
            }
            SeekFrom::End(offset) => {
                cmp::min(end as i64, cmp::max(0, end as i64 + offset)) as usize
            }
        };

        Ok(self.seek as u64)
    }

    /// Sync the resource, should flip
    pub fn sync(&mut self) -> Result<()> {
        self.window.redraw();
        Ok(())
    }
}

/// A window scheme
pub struct Scheme {
    pub session: Option<Box<Session>>,
    pub next_x: isize,
    pub next_y: isize,
}

impl Scheme {
    pub fn new() -> Box<Scheme> {
        let mut ret = box Scheme {
            session: None,
            next_x: 0,
            next_y: 0,
        };
        if let Some(mut session) = Session::new() {
            println!("- Orbital: Found Display {}x{}", session.display.width, session.display.height);
            println!("    Console: Press F1");
            println!("    Desktop: Press F2");

            unsafe { session_ptr = session.deref_mut() };
            ret.session = Some(session);
        } else {
            println!("- Orbital: No Display Found");
        }
        ret
    }

    pub fn open(&mut self, url_str: &str, _: usize) -> Result<Box<Resource>> {
        if let Some(ref session) = self.session {
            // window://host/path/path/path is the path type we're working with.
            let url = Url::from_str(url_str);

            let host = url.host();
            if host.is_empty() {
                let path = url.path_parts();
                let mut pointx = match path.get(0) {
                    Some(x) => x.to_num_signed(),
                    None => 0,
                };
                let mut pointy = match path.get(1) {
                    Some(y) => y.to_num_signed(),
                    None => 0,
                };
                let size_width = match path.get(2) {
                    Some(w) => w.to_num(),
                    None => 100,
                };
                let size_height = match path.get(3) {
                    Some(h) => h.to_num(),
                    None => 100,
                };

                let mut title = match path.get(4) {
                    Some(t) => t.clone(),
                    None => String::new(),
                };
                for i in 5..path.len() {
                    if let Some(t) = path.get(i) {
                        title = title + "/" + t;
                    }
                }

                if pointx <= 0 || pointy <= 0 {
                    if self.next_x > session.display.width as isize - size_width as isize {
                        self.next_x = 0;
                    }
                    self.next_x += 32;
                    pointx = self.next_x as i32;

                    if self.next_y > session.display.height as isize - size_height as isize {
                        self.next_y = 0;
                    }
                    self.next_y += 32;
                    pointy = self.next_y as i32;
                }

                Ok(box Resource {
                    window: Window::new(Point::new(pointx, pointy),
                                        Size::new(size_width, size_height),
                                        title),
                    seek: 0,
                })
            } else {
                Err(Error::new(ENOENT))
            }
        }else{
            Err(Error::new(ENOENT))
        }
    }

    pub fn event(&mut self, event: &Event) {
        if let Some(ref mut session) = self.session {
            session.event(event);

            unsafe { session.redraw() };
        }
    }
}
*/

struct OrbitalScheme;

impl OrbitalScheme {
    fn new() -> OrbitalScheme {
        OrbitalScheme
    }
}

impl Scheme for OrbitalScheme {}

fn main() {
    match File::open("display:") {
        Ok(mut display) => {
            let path = display.path().unwrap().to_string();
            let res = path.split(":").nth(1).unwrap();
            let width = res.split("x").nth(0).unwrap().parse::<u32>().unwrap();
            let height = res.split("x").nth(1).unwrap().parse::<u32>().unwrap();

            println!("- Orbital: Found Display {}x{}", width, height);
            println!("    Console: Press F1");
            println!("    Desktop: Press F2");

            let mut data: Vec<u32> = Vec::new();
            for b in 0..width * height {
                data.push(0xFFFFFFFF);
            }

            display.seek(SeekFrom::Start(0)).unwrap();
            display.write(unsafe { & slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4) }).unwrap();
            display.sync_all().unwrap();

            let mut scheme = OrbitalScheme::new();
            let mut socket = File::create(":orbital").unwrap();
            loop {
                let mut packet = Packet::default();
                if socket.read(&mut packet).unwrap() == 0 {
                    panic!("Unexpected EOF");
                }
                //println!("Recv {:?}", packet);

                scheme.handle(&mut packet);

                socket.write(&packet).unwrap();
                //println!("Sent {:?}", packet);
            }
        },
        Err(err) => println!("- Orbital: No Display Found: {}", err)
    }
}