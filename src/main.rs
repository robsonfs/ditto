use ditto::docx_to_pdf;
use std::path::Path;

fn main() {
    let input_doc = Path::new("document.docx");
    let output_path = Path::new("document.pdf");

    let _ = docx_to_pdf(input_doc, output_path);
}
