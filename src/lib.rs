use pyo3::prelude::*;

mod image_cleaner;
use image_cleaner::text_cleaner;

/// Get the text from an image.
#[pyfunction]
fn get_text(location: &str) -> PyResult<String> {

    let mut lt = leptess::LepTess::new(Some("./assets/tessdata/"), "eng").unwrap();
    lt.set_variable(leptess::Variable::TesseditPagesegMode, "10").unwrap();
    lt.set_variable(leptess::Variable::TesseditCharWhitelist, "ABCDEFGHIJKLMNOPQRSTUWXY").unwrap();
    let img = text_cleaner::Image::new(location);
    let tiff_buffer = img.binarize();
    // binarized_image.save("./result.png").unwrap();
    lt.set_image_from_mem(&tiff_buffer).unwrap();
    lt.set_source_resolution(70);
    let text = lt.get_utf8_text().unwrap();
    
    Ok(text.replace("\n", "").replace(" ", ""))
}
/// A Python module implemented in Rust.
#[pymodule]
fn ocr_rusty(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_text, m)?)?;
    Ok(())
}

#[test]
fn test_get_text() {
    let text = get_text("./assets/images/text.jpg").unwrap();
    println!("{}", text);
    assert_eq!(text, "YNIUSDT");
}