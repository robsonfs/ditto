# Ditto

Ditto is a Python library powered by Rust and LibreOffice that provides a seamless conversion feature from DOCX to PDF.

## Introduction

Ditto leverages the speed and safety of Rust along with the robust document processing capabilities of LibreOffice. With Ditto, you can easily convert DOCX files into PDF format within your Python applications.

## Why Ditto?

Ditto is named after the Pokémon known for its ability to transform into any other Pokémon. Similarly, this library is all about transformation—taking a DOCX document and converting it into a PDF. Just as Ditto adapts to mimic other forms, our library adapts your documents from one format to another effortlessly.

## LibreOffice Dependency

Ditto depends on LibreOffice’s command-line tool (`soffice`) to perform the conversion process. **Important points:**

- **External Requirement:** LibreOffice is not bundled with Ditto. It must be installed on your system and available in your `PATH`.
- **Installation:** Make sure LibreOffice is installed for Ditto to work correctly on your target environment.
- **Error Handling:** If LibreOffice is missing or misconfigured, Ditto will return an error, so proper documentation and checks are advised in your deployment process.

## How to Compile and Run the Rust Code

If you wish to compile and run the underlying Rust code independently, follow these steps:

1. **Install Rust:** Ensure that [Rust](https://www.rust-lang.org/tools/install) is installed on your system.
2. **Build the Project:** In the project’s root directory, run:
   ```bash
   cargo build --release
   ```
   This command compiles the project in release mode. The resulting binary can be found in the `target/release` directory.
3. **Run the Binary: [Work in progress...]** Use the compiled binary to test the conversion functionality. For instance:
   ```bash
   ./target/release/ditto --input /path/to/input.docx --output /path/to/output.pdf
   ```

## Integration with Python (pyO3) [Work in progress...]

Ditto’s core is implemented in Rust, and it is designed to be seamlessly integrated with Python using [pyO3](https://pyo3.rs/). Check the additional project documentation for detailed instructions on building and using the Python bindings.
