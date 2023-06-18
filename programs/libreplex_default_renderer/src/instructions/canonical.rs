use anchor_lang::prelude::*;

extern crate bmp;
use bmp::{Image, Pixel};


use super::input::{RenderInput, RenderContext};



const RED_GEN: u8 = 7;
const GREEN_GEN: u8 = 13;
const BLUE_GEN: u8 = 17;

const RESOLUTION: u32 = 8;


pub fn handler(ctx: Context<RenderContext>,
               _render_input: RenderInput,
) -> anchor_lang::Result<Vec<u8>> {



    let metadata_key = &ctx.accounts.metadata.key();
    let keyref = metadata_key.as_ref();
    msg!("len: {}", keyref.len());

    let mut img = Image::new(8, 8);



    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let red = ((RED_GEN as u32).checked_mul(x+7).unwrap().checked_mul(
                keyref[x as usize] as u32
            ).unwrap() % 32)*8;
            let green = ((GREEN_GEN as u32).checked_mul(x+13).unwrap().checked_mul(
                keyref[y as usize] as u32
            ).unwrap() % 32 )* 8;
            let blue = ((BLUE_GEN as u32).checked_mul(x+17).unwrap().checked_mul(
                keyref[(x.checked_mul(y).unwrap() % 32) as usize] as u32
            ).unwrap() % 256 ) *8;
            img.set_pixel(x, y, Pixel::new(red as u8, green as u8, blue as u8))
        }
    }


    // img.set_pixel(0,0, Pixel::new(255,255,255));
    // img.set_pixel(0,0, Pixel::new(255,255,255));
    // img.set_pixel(0,0, Pixel::new(255,255,255));

    let mut buf = Vec::new();
    img.to_writer(&mut buf)?;
    buf.push(1);
    Ok(buf)
}
