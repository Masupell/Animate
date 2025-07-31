use ab_glyph::{point, Font, FontArc, PxScale, ScaleFont};
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