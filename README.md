📝 Student Report Card Generator
A Rust CLI app that generates PDF report cards with grades

https://img.shields.io/badge/Rust-v1.70+-orange
https://img.shields.io/badge/License-MIT-blue

✨ Features
📌 Input student details (name, marks, subjects)

🧮 Auto-calculates average and grade (A/B/C/D)

📄 Generates a clean PDF report card

🖨️ Console preview of results

🚀 Quick Start
Prerequisites
Rust (v1.70+)

Steps
Clone the repo

bash
git clone https://github.com/Harshanb21/report_card.git
cd report_card
Run the program

bash
cargo run
Follow prompts to enter student data.

Check the output

Console: See grades instantly

PDF: Generated as report_card.pdf in the project root

📂 File Structure
text
report_card/
├── src/
│   └── main.rs          # Core logic
├── Cargo.toml           # Rust config
├── sample_report.pdf    # Example output (optional)
└── .gitignore           # Ignores target/ and local PDFs
🛠️ Customization
Change Grading System
Edit the assign_grade() function in src/main.rs:

rust
match average {
    a if a >= 90.0 => 'A',  // Modify thresholds here
    a if a >= 75.0 => 'B',
    // ...
}
Use a Different PDF Font
Replace Helvetica in main.rs with:

rust
doc.add_builtin_font(BuiltinFont::TimesRoman)  // Options: Courier, HelveticaBold, etc.


bash
mv report_card.pdf sample_report.pdf
Commit it manually.

📜 License
MIT © Harshan

🔗 Recommended Tools
VS Code with Rust Analyzer

