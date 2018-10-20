//! Exercise for practicing assertions and backtraces.
//!
//! This exercise panics when run, but it involves asynchronous operations
//! so the backtrace does not tell us anything that is immediately useful.
//!
//! Work backwards from the immediate cause of the panic (out of bounds
//! array access) by:
//! 1. associating a backtrace with each operation
//! 2. printing a culprit backtrace before the panic occurs
//! 3. adding assertions to catch the responsible code before it
//!    becomes an asynchronous operation

extern crate backtrace;

use backtrace::Backtrace;
use std::mem;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

/// A solid rectangle.
struct Rect {
    /// The leftmost point of the rectangle.
    x: u32,
    /// The topmost point of the rectangle.
    y: u32,
    /// The width of the rectangle.
    width: u32,
    /// The height of the rectangle.
    height: u32,
    /// The fill color of the rectangle.
    color: u32,
}

/// The commands that the graphics backend understands.
enum Command {
    /// Draw a rectangle.
    Rect(Rect, Backtrace),
    /// Request a pixel buffer containing the rendered rectangles.
    Render(Sender<Vec<u32>>, Backtrace),
}

/// Run a rendering loop indefinitely.
fn graphics_loop(receiver: Receiver<Command>, width: u32, height: u32) {
    // The rectangles that have been recorded so far.
    let mut rects = vec![];
    // If there is no way to communicate with this graphics backend,
    // we can shut it down.
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Rect(rect, trace) => {
                // Add a rectangle to the list of known rectangles.
                rects.push((rect, trace));
            }

            Command::Render(sender, _trace) => {
                // Create a pixel buffer with the provided size.
                let mut buffer = vec![0; (width * height) as usize];
                // Retrieve the list of known rectangles and clear the original.
                let rects = mem::replace(&mut rects, vec![]);
                for (rect, trace) in rects {
                    // For each known rectangle, plot all of its pixels in the pixel buffer.
                    for y_offset in 0..rect.height {
                        for x_offset in 0..rect.width {
                            let index = (rect.y + y_offset) * width + (rect.x + x_offset);
                            if index as usize >= buffer.len() {
                                eprintln!("This rectangle is too big: {:?}", trace);
                            }
                            buffer[index as usize] = rect.color;
                        }
                    }
                }
                // The rectangles are rendered; send the pixel buffer to its destination.
                let _ = sender.send(buffer);
            }
        }
    }
}

/// A renderer encapsulating access to a graphics backend.
struct Renderer {
    /// The sending half of a channel to the graphics backend.
    sender: Sender<Command>,
    /// The current color for any graphics operations.
    color: u32,
    /// The current x origin for any grapics operations.
    x: u32,
    /// The current y origin for any graphics operations.
    y: u32,
    width: u32,
    height: u32,
}

impl Renderer {
    /// Set the current color.
    fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    /// Set the current x and y origins.
    fn set_origin(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    /// Draw a square with the provided length.
    fn square(&self, length: u32) {
        self.sender.send(Command::Rect(Rect {
            x: self.x,
            y: self.y,
            width: length,
            height: length,
            color: self.color,
        }, Backtrace::new())).expect("Missing graphics thread")
    }

    /// Draw a rectangle with the provided length.
    fn rect(&self, width: u32, height: u32) {
        self.sender.send(Command::Rect(Rect {
            x: self.x,
            y: self.y,
            width: width,
            height: height,
            color: self.color,
        }, Backtrace::new())).expect("Missing graphics thread")
    }

    /// Draw a horizontal line with the provided length.
    fn horizontal_line(&self, length: u32) {
        if self.x + length >= self.width {
            panic!("no");
        }
        self.sender.send(Command::Rect(Rect {
            x: self.x,
            y: self.y,
            width: length,
            height: 1,
            color: self.color,
        }, Backtrace::new())).expect("Missing graphics thread")
    }

    /// Draw a vertical line with the provided length.
    fn vertical_line(&self, length: u32) {
        if self.y + length >= self.height {
            panic!("no");
        }
        self.sender.send(Command::Rect(Rect {
            x: self.x,
            y: self.y,
            width: 1,
            height: length,
            color: self.color,
        }, Backtrace::new())).expect("Missing graphics thread")
    }

    /// Retrieve the pixels that match all previous drawing operations
    /// since the previous get_pixels call, or since this renderer's
    /// creation.
    fn get_pixels(&self) -> Vec<u32> {
        let (pixels_sender, pixels_receiver) = channel();
        self.sender.send(Command::Render(pixels_sender, Backtrace::new())).expect("Missing graphics thread");
        pixels_receiver.recv().expect("Couldn't get pixels")
    }
}

/// Create a new renderer object corresponding to a
/// graphics buffer of width x height pixels.
fn create_renderer(width: u32, height: u32) -> Renderer {
    let (sender, receiver) = channel();
    // Create a new thread for the graphics backend to run independently.
    thread::spawn(move || {
        graphics_loop(receiver, width, height)
    });

    // Return a handle to the new graphics backend.
    Renderer {
        sender: sender,
        color: 0,
        x: 0,
        y: 0,
        width: width,
        height: height,
    }
}

fn main() {
    let mut r = create_renderer(10, 10);
    r.set_color(0xFFFFFFFF);
    r.square(5);
    r.set_origin(5, 0);
    r.set_color(0x777777FF);
    r.rect(5, 10);
    r.set_color(0x44AAFFFF);
    for x in 0..=5 {
        r.set_origin(x, 5);
        r.vertical_line(5);
    }
    r.set_color(0x9944CCFF);
    for y in 0..=5 {
        r.set_origin(0, 5 + y);
        r.horizontal_line(5);
    }
    println!("{:?}", r.get_pixels());
}
