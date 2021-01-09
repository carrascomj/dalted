/// Parse and render an SVG into PNG bytes
///
/// This is required for SVG because the `image` crate works with pixel maps.
/// A much more performant alternative would take the svg nodes parsed by
/// `usvg` and transform the parsed colors without relying on `image`.
pub fn decode_svg(
    bytes: &[u8],
) -> Result<image::DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(bytes, &opt)?;
    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();
    Ok(image::load_from_memory(pixmap.encode_png()?.as_slice())?)
}
