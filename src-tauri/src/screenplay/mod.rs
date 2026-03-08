// Module declarations for screenplay document model and business logic

/// Document structs for the .screenplay file format (content, meta, settings).
pub mod document;

/// Typst-based PDF generation: converts ProseMirror JSON to Typst markup and PDF bytes.
pub mod pdf;
