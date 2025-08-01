So far just a simple 2D Engine. Can draw Text, images and color-rects.
"cargo run -p engine" to run engine

Need to implement Text drawing, not just char, for that It would be usefull to implement an AtlasTexture (batchin in general)
ALso SDF-rendering, not bitmap-texture, for smooth-scaling

So steps:
1. Render text as one instead of singular chars
2. Make Texture drawing easier by creating a Texture struct for external use (not internal Texture)
3. Add extra Shapes, like circles, etc
4. Add Texture-batching