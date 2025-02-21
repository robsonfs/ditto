//! # Document Conversion Library
//!
//! This library provides utilities for converting `.docx` files to `.pdf` using LibreOffice (`soffice`).
//! It ensures proper validation of input and output paths before invoking the conversion process.
//!
//! ## Requirements
//! - LibreOffice (`soffice`) must be installed and available in the system's `PATH`.
//! - The function assumes that the output directory exists or can be created.
//!
//! ## Example Usage
//! ```no_run
//! use std::path::Path;
//! use ditto::docx_to_pdf;
//!
//! let input = Path::new("/path/to/input.docx");
//! let output = Path::new("/path/to/output.pdf");
//!
//! match docx_to_pdf(input, output) {
//!     Ok(_) => println!("Conversion successful!"),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```

use std::path::Path;
use std::process::Command;

/// Converts a `.docx` file to `.pdf` using LibreOffice (`soffice`).
///
/// # Arguments
/// * `input_path` - The path to the `.docx` file that needs to be converted.
/// * `output_path` - The desired output path for the `.pdf` file.
///
/// # Returns
/// * `Ok(())` if the conversion is successful.
/// * `Err(Box<dyn std::error::Error>)` if an error occurs during validation or conversion.
///
/// # Errors
/// This function returns an error if:
/// - The input file does not exist or is not a valid file.
/// - The output path points to an existing directory.
/// - LibreOffice fails to convert the file.
/// - The expected output PDF file is not found after conversion.
///
/// # Requirements
/// - LibreOffice (`soffice`) must be installed and accessible via the system `PATH`.
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// use ditto::docx_to_pdf;
///
/// let input = Path::new("example.docx");
/// let output = Path::new("example.pdf");
///
/// if let Err(e) = docx_to_pdf(input, output) {
///     eprintln!("Conversion failed: {}", e);
/// }
/// ```
pub fn docx_to_pdf(
    input_path: &Path,
    output_path: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    if !input_path.exists() {
        return Err(
            format!("Input file not found: {}", input_path.display()).into()
        );
    }
    if !input_path.is_file() {
        return Err(
            format!("Input path is not a file: {}", input_path.display()).into()
        );
    }

    if output_path.exists() && output_path.is_dir() {
        return Err("Output path is a directory".into());
    }

    let output_dir = output_path
        .parent()
        .ok_or("Output path has no parent directory")?;
    std::fs::create_dir_all(output_dir)?;

    let status = Command::new("soffice")
        .args(&[
            "--headless",
            "--convert-to",
            "pdf",
            "--outdir",
            output_dir.to_str().ok_or("Invalid output directory")?,
            input_path.to_str().ok_or("Invalid input path")?,
        ])
        .status()?;

    if !status.success() {
        return Err(format!(
            "Conversion failed with exit code: {:?}",
            status.code()
        )
        .into());
    }

    let generated_pdf = output_dir
        .join(input_path.file_stem().ok_or("Input file has no stem")?)
        .with_extension("pdf");

    if !generated_pdf.exists() {
        return Err(format!(
            "Generated PDF not found at: {}",
            generated_pdf.display()
        )
        .into());
    }

    if generated_pdf != output_path {
        std::fs::rename(&generated_pdf, output_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_docx_to_pdf_invalid_input() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("nonexistent.docx");
        let output_path = temp_dir.path().join("output.pdf");
        let result = docx_to_pdf(&input_path, &output_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_docx_to_pdf_invalid_output_directory() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("test.docx");
        let mut file = File::create(&input_path).unwrap();
        writeln!(file, "Test content").unwrap();
        let output_path = temp_dir.path().join("some_dir");
        std::fs::create_dir(&output_path).unwrap();
        let result = docx_to_pdf(&input_path, &output_path);
        assert!(result.is_err());
    }
}
