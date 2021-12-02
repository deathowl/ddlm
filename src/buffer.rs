use thiserror::Error;

use crate::color::Color;

pub type Vect = (u32, u32);
pub type Rect = (u32, u32, u32, u32);

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BufferError {
    #[error("cannot create subdimensions larger than buffer: {subdimensions:?} > {bounds:?}")]
    SubdimensionsTooLarge { subdimensions: Rect, bounds: Rect },
    #[error("cannot create offset outside buffer: {offset:?} > {bounds:?}")]
    OffsetOutOfBounds { offset: Vect, bounds: Rect },
    #[error("put({pos:?}) is not within subdimensions of buffer ({subdim:?})")]
    PixelOutOfSubdimBounds { pos: Vect, subdim: Rect },
    #[error("put({pos:?}) is not within dimensions of buffer ({dim:?})")]
    PixelOutOfBounds { pos: Vect, dim: Vect },
}

pub struct Buffer<'a> {
    buf: &'a mut [u8],
    dimensions: Vect,
    subdimensions: Option<Rect>,
}

impl<'a> Buffer<'a> {
    pub fn new(buf: &'a mut [u8], dimensions: Vect) -> Self {
        Self {
            buf,
            dimensions,
            subdimensions: None,
        }
    }

    pub fn get_bounds(&self) -> Rect {
        if let Some(subdim) = self.subdimensions {
            subdim
        } else {
            (0, 0, self.dimensions.0, self.dimensions.1)
        }
    }

    pub fn subdimensions(&mut self, subdimensions: Rect) -> Result<Buffer<'_>, BufferError> {
        let bounds = self.get_bounds();
        if subdimensions.0 + subdimensions.2 >= bounds.2
            || subdimensions.1 + subdimensions.3 >= bounds.3
        {
            return Err(BufferError::SubdimensionsTooLarge {
                subdimensions,
                bounds,
            });
        }

        Ok(Buffer {
            buf: self.buf,
            dimensions: self.dimensions,
            subdimensions: Some((
                subdimensions.0 + bounds.0,
                subdimensions.1 + bounds.1,
                subdimensions.2,
                subdimensions.3,
            )),
        })
    }

    pub fn offset(&mut self, offset: Vect) -> Result<Buffer<'_>, BufferError> {
        let bounds = self.get_bounds();
        if offset.0 > bounds.2 || offset.1 > bounds.3 {
            return Err(BufferError::OffsetOutOfBounds { offset, bounds });
        }

        Ok(Buffer {
            buf: self.buf,
            dimensions: self.dimensions,
            subdimensions: Some((
                offset.0 + bounds.0,
                offset.1 + bounds.1,
                bounds.2 - offset.0,
                bounds.3 - offset.1,
            )),
        })
    }

    pub fn memset(&mut self, c: &Color) {
        if let Some(subdim) = self.subdimensions {
            unsafe {
                let ptr = self.buf.as_mut_ptr();
                for y in subdim.1..(subdim.1 + subdim.3) {
                    for x in subdim.0..(subdim.0 + subdim.2) {
                        *((ptr as *mut u32).offset((x + y * self.dimensions.0) as isize)) =
                            c.as_argb8888();
                    }
                }
            }
        } else {
            unsafe {
                let ptr = self.buf.as_mut_ptr();
                for p in 0..(self.dimensions.0 * self.dimensions.1) {
                    *((ptr as *mut u32).offset(p as isize)) = c.as_argb8888();
                }
            }
        }
    }

    pub fn put(&mut self, pos: Vect, c: &Color) -> Result<(), BufferError> {
        let true_pos = if let Some(subdim) = self.subdimensions {
            if pos.0 >= subdim.2 || pos.1 >= subdim.3 {
                return Err(BufferError::PixelOutOfSubdimBounds { pos, subdim });
            }
            (pos.0 + subdim.0, pos.1 + subdim.1)
        } else {
            if pos.0 >= self.dimensions.0 || pos.1 >= self.dimensions.1 {
                return Err(BufferError::PixelOutOfBounds {
                    pos,
                    dim: self.dimensions,
                });
            }
            pos
        };

        unsafe {
            let ptr = self
                .buf
                .as_mut_ptr()
                .offset(4 * (true_pos.0 + (true_pos.1 * self.dimensions.0)) as isize);
            *(ptr as *mut u32) = c.as_argb8888();
        };

        Ok(())
    }
}
