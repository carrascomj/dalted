/// Parse and render an SVG into PNG bytes
///
/// This is required for SVG because the `image` crate works with pixel maps.
/// A much more performant alternative would take the svg nodes parsed by
/// `usvg` and transform the parsed colors without relying on `image`.
pub fn decode_svg(
    bytes: &[u8],
) -> Result<(Vec<u8>, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(bytes, &opt)?;
    let (width, height) = rtree.svg_node().size.to_screen_size().dimensions();
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();

    resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();
    Ok((pixmap.take(), width, height))
}
