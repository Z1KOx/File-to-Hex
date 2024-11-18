use rfd::FileDialog;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

pub struct FM {
    path: Option<PathBuf>,
    pub(crate) save_path: Option<PathBuf>,
    file_name: Option<String>,
    buffer: Vec<u8>,
}

impl FM {
    pub fn new() -> Self {
        Self {
            path: None,
            save_path: None,
            file_name: None,
            buffer: Vec::new(),
        }
    }

    pub fn get_save_path(&self) -> Option<&PathBuf> {
        self.save_path.as_ref()
    }

    pub fn get_buffer_length(&self) -> usize {
        self.buffer.len()
    }

    pub fn load_file(&mut self) {
        self.select_file();

        let file = File::open(self.path.as_ref().expect("Failed to open file"))
            .unwrap_or_else(|err| panic!("File not found: {}", err));
        let mut reader = BufReader::new(file);
        reader
            .read_to_end(&mut self.buffer)
            .unwrap_or_else(|err| panic!("Can't read buffer: {}", err));
    }

    fn fill_file(&self, declaration: String) -> std::io::Result<()> {
        let mut file = File::create(self.save_path.as_ref().expect("No save path selected"))?;
        file.write_all(declaration.as_bytes())?;

        let mut counter: u8 = 1;
        for &value in &self.buffer {
            if counter == 1 {
                file.write_all(b"    ")?;
            }

            let hex_bytes: Vec<u8> = format!("0x{:02X}, ", value).into_bytes();
            file.write_all(&hex_bytes)?;

            if counter == 12 {
                file.write_all(b"\n")?;
                counter = 1;
            } else {
                counter += 1;
            }
        }

        file.write_all(b"\n};")?;
        Ok(())
    }

    pub(crate) fn create_file(
        &mut self,
        extension: &str,
        declaration: &str,
    ) -> std::io::Result<()> {
        if self.buffer.is_empty() || self.file_name.is_none() {
            println!("Buffer is empty. Load a file first.");
            return Ok(());
        }

        let file_name = self
            .file_name
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_default();
        self.save_path = FileDialog::new()
            .set_file_name(&format!("{}.{}", file_name, extension))
            .add_filter(
                &format!("{} sources (*.{})", extension.to_uppercase(), extension),
                &[extension],
            )
            .save_file();

        self.fill_file(declaration.to_string())?;
        Ok(())
    }

    fn select_file(&mut self) {
        if let Some(selected_path) = FileDialog::new().pick_file() {
            self.path = Some(selected_path.clone());
            self.file_name = selected_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|stem| stem.to_string());
        } else {
            panic!("No file selected");
        }
    }
}
