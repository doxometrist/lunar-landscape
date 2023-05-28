use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;

pub enum Status {
    Done,
    Doing,
    Lapsed,
    Future,
}

pub struct PDFReference {
    path: Vec<String>,
    topic_tags: Vec<String>,
    status: Status,
    project_tags: Vec<String>,
}

// Function to read pdf references
pub fn read_pdf_references(file_location: &str, predefined_pdf_references: Vec<PDFReference>) -> Vec<PDFReference> {
    let mut matching_pdf_references = Vec::new();

    // Create a regex for PDF files
    let pdf_re = Regex::new(r"\.pdf$").unwrap();

    // Iterate over all entries in the directory
    for entry in WalkDir::new(file_location) {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        // If the entry is a PDF file
        if entry_path.is_file() && pdf_re.is_match(entry_path.to_string_lossy().as_ref()) {
            // Convert the path to a Vec<String>
            let path_as_vec: Vec<String> = entry_path
                .iter()
                .map(|os_str| os_str.to_string_lossy().to_string())
                .collect();

            // Compare it with the predefined PDF references
            for pdf_reference in &predefined_pdf_references {
                if pdf_reference.path == path_as_vec {
                    // If the PDF reference matches, add it to the output
                    matching_pdf_references.push(pdf_reference.clone());
                }
            }
        }
    }

    matching_pdf_references
}
