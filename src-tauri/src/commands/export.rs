// Tauri commands for PDF, Fountain, and plain text export
//
// These commands are called from the Svelte frontend via `invoke()`.
// Each command receives a ScreenplayDocument, processes it, and returns
// the exported data or an error string.

use crate::fonts;
use crate::screenplay::document::ScreenplayDocument;
use crate::screenplay::pdf;

/// Generates Typst markup from a screenplay document.
///
/// Returns the Typst markup string for debugging and preview purposes.
/// This is useful during development while PDF compilation is being implemented,
/// and may also be useful for advanced users who want to customize the Typst source.
///
/// # Arguments
///
/// * `document` — The full `.screenplay` document, deserialized from JSON by Tauri.
///
/// # Returns
///
/// * `Ok(String)` — The generated Typst markup source code.
/// * `Err(String)` — An error message if markup generation fails.
#[tauri::command]
pub fn export_typst_markup(document: ScreenplayDocument) -> Result<String, String> {
    // Map the font setting slug to the human-readable font name that Typst expects.
    // The document stores a slug like "manjari", but Typst needs the actual font
    // family name like "Manjari" to match against the embedded font data.
    let font_name = match document.settings.font.as_str() {
        "manjari" => "Manjari",
        _ => "Noto Sans Malayalam", // Default font if unrecognized
    };

    // `&document.content` passes a reference (borrow) to the content field.
    // We don't need to take ownership — we just need to read the JSON.
    Ok(pdf::generate_typst_markup(&document.content, font_name))
}

/// Exports a screenplay document as PDF bytes.
///
/// The document's ProseMirror JSON content is converted to Typst markup,
/// then compiled to PDF with the selected font embedded. The PDF bytes
/// are returned to the frontend, which can then save them to disk.
///
/// # Arguments
///
/// * `document` — The full `.screenplay` document, deserialized from JSON by Tauri.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` — The raw PDF file bytes ready to write to disk.
/// * `Err(String)` — An error message if PDF generation fails.
#[tauri::command]
pub fn export_pdf(document: ScreenplayDocument) -> Result<Vec<u8>, String> {
    // `bundled_fonts()` returns a Vec<BundledFont> — all fonts compiled into the binary.
    let bundled = fonts::bundled_fonts();

    // Determine which font to use based on the document's settings.
    // We need both the Typst font name and the matching BundledFont struct.
    let (font_name, font) = match document.settings.font.as_str() {
        "manjari" => (
            "Manjari",
            // `iter().find()` searches through the vector for the first matching item.
            // It returns `Option<&BundledFont>` — Some if found, None if not.
            bundled.iter().find(|f| f.name == "Manjari"),
        ),
        _ => (
            "Noto Sans Malayalam",
            bundled.iter().find(|f| f.name == "Noto Sans Malayalam"),
        ),
    };

    // `ok_or_else` converts an Option to a Result:
    // Some(value) → Ok(value), None → Err(the error string we provide).
    // The `?` at the end means: if this is Err, return that error immediately
    // from the whole function. This is Rust's error propagation operator.
    let font = font.ok_or_else(|| "Selected font not found in bundled fonts".to_string())?;

    // Build the FontData struct that pdf::generate_pdf expects.
    // `font.regular` and `font.bold` are `&'static [u8]` — static byte slices
    // that live for the entire program because they were embedded at compile time.
    let font_data = pdf::FontData {
        regular: font.regular,
        bold: font.bold,
    };

    pdf::generate_pdf(&document.content, font_name, &font_data)
}
