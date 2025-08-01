use std::collections::HashMap;

use ab_glyph::{point, Font, FontArc, PxScale};
use anyhow::{Ok, anyhow};

pub fn rasterize_char(font_path: &str, char: char) -> std::result::Result<(Vec<u8>, usize, usize), anyhow::Error>
{
    let font_data = std::fs::read(font_path)?;

    let font = FontArc::try_from_vec(font_data)?;

    let scale = PxScale::from(64.0);

    let glyph = font.glyph_id(char).with_scale_and_position(scale, point(0.0, 0.0));

    let outline = font.outline_glyph(glyph).ok_or_else(|| anyhow!("Could not outline glyph '{}'", char))?;

    let bounds = outline.px_bounds();
    let width = bounds.width().ceil() as usize;
    let height = bounds.height().ceil() as usize;

    let mut bitmap = vec![0u8; width * height];
    outline.draw(|x, y, c| 
    { 
        let xx = x as usize;
        let yy = y as usize;

        if xx < width && yy < height
        {
            bitmap[yy * width + xx] = (c * 255.0) as u8;
        }
    });

    Ok((bitmap, width, height))
}

// pub struct TextHandler
// {
//     fonts: Vec<FontArc> // Not sure if this is the best way, but for now it's fine
// }

// impl TextHandler
// {
//     pub fn new(font_paths: Vec<&str>) -> std::result::Result<TextHandler, anyhow::Error>
//     {
//         let mut fonts: Vec<FontArc> = Vec::new();

//         for path in font_paths
//         {
//             let font_data = std::fs::read(path)?;
//             let font = FontArc::try_from_vec(font_data)?;
//             fonts.push(font);
//         }

//         Ok(TextHandler
//         { 
//             fonts 
//         })
//     }

//     pub fn add_font(&mut self, font_path: &str) -> std::result::Result<usize, anyhow::Error> // returns id of font (just vector-position)
//     {
//         let font_data = std::fs::read(font_path)?;
//         let font = FontArc::try_from_vec(font_data)?;
//         let id = self.fonts.len();
//         self.fonts.push(font);
//         Ok(id)
//     }

//     pub fn rasterize_text(text: &str, text_scale: f32) -> std::result::Result<(Vec<u8>, usize, usize), anyhow::Error>
//     {
//         let scale = PxScale::from(text_scale);


//     }
// }

pub struct FontAtlas
{
    pub texture: wgpu::BindGroup,
    pub glyphs: HashMap<char, Glyph>
}

pub struct Glyph
{
    pub uv_min: [f32; 2], // top left (uv coordinate in texture)
    pub uv_max: [f32; 2], // top right ("-")
    pub size: [f32; 2], // width and height of bitmap
    // pub offset: [f32; 2], // offset of baseline (like down or Up, forp or l)
    pub advance: f32, // Space after Glyph
}