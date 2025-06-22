use std::fs::File;
use std::io::{self, Write};
use printpdf::*;
use chrono::Local;
use std::io::BufWriter;

#[derive(Debug)]
struct Student {
    name: String,
    total_marks: f64,
    num_subjects: u32,
}

impl Student {
    fn new(name: String, total_marks: f64, num_subjects: u32) -> Self {
        Student {
            name,
            total_marks,
            num_subjects,
        }
    }

    fn calculate_average(&self) -> f64 {
        self.total_marks / self.num_subjects as f64
    }

    fn assign_grade(&self) -> char {
        let average = self.calculate_average();
        match average {
            a if a >= 90.0 => 'A',
            a if a >= 75.0 => 'B',
            a if a >= 60.0 => 'C',
            _ => 'D',
        }
    }
}

fn generate_pdf_report(student: &Student) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new(
        "Student Report Card",
        Mm(210.0),  // A4 width
        Mm(297.0),  // A4 height
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    // Add title
    current_layer.use_text("STUDENT REPORT CARD", 24.0, Mm(50.0), Mm(250.0), &font);
    
    // Add date
    let date = Local::now().format("%d/%m/%Y").to_string();
    current_layer.use_text(format!("Date: {}", date), 12.0, Mm(150.0), Mm(240.0), &font);

    // Add student details
    current_layer.use_text(format!("Student Name: {}", student.name), 14.0, Mm(20.0), Mm(220.0), &font);
    current_layer.use_text(format!("Total Marks: {:.2}", student.total_marks), 14.0, Mm(20.0), Mm(200.0), &font);
    current_layer.use_text(format!("Number of Subjects: {}", student.num_subjects), 14.0, Mm(20.0), Mm(180.0), &font);
    
    // Add calculated results
    let average = student.calculate_average();
    let grade = student.assign_grade();
    current_layer.use_text(format!("Average Score: {:.2}", average), 16.0, Mm(20.0), Mm(150.0), &font);
    current_layer.use_text(format!("Grade: {}", grade), 16.0, Mm(20.0), Mm(130.0), &font);

    // Add some decoration
    current_layer.use_text("Congratulations!", 18.0, Mm(20.0), Mm(100.0), &font);
    
    // Save the PDF
    doc.save(&mut BufWriter::new(File::create("report_card.pdf")?))?;
    
    Ok(())
}

fn main() {
    println!("Student Report Card Generator");
    println!("------------------------------");

    // Get student details
    print!("Enter student name: ");
    io::stdout().flush().unwrap();
    let name: String = text_io::read!();

    print!("Enter total marks: ");
    io::stdout().flush().unwrap();
    let total_marks: f64 = text_io::read!();

    print!("Enter number of subjects: ");
    io::stdout().flush().unwrap();
    let num_subjects: u32 = text_io::read!();

    // Create student
    let student = Student::new(name, total_marks, num_subjects);

    // Calculate and display results
    let average = student.calculate_average();
    let grade = student.assign_grade();

    println!("\nStudent Results:");
    println!("----------------");
    println!("Name: {}", student.name);
    println!("Total Marks: {:.2}", student.total_marks);
    println!("Number of Subjects: {}", student.num_subjects);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);

    // Generate PDF report
    match generate_pdf_report(&student) {
        Ok(_) => println!("\nPDF report card generated successfully as 'report_card.pdf'"),
        Err(e) => eprintln!("Error generating PDF: {}", e),
    }
}