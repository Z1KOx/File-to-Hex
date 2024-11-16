use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{PathBuf};
use rfd::FileDialog;

pub struct FM
{
    path: Option<PathBuf>,
    save_path: Option<PathBuf>,
    file_name: Option<String>,
    buffer: Vec<u8>,
}

impl FM
{
    pub fn new() -> Self
    {
        Self { path: None,
               save_path: None,
               file_name: None,
               buffer: Vec::new() }
    }

    pub fn load_file(&mut self)
    {
        self.select_file();
        let file = File::open(self.path.as_ref()
            .expect("Failed to open file")).expect("File not found");
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut self.buffer).expect("Can't read buffer");
    }

    pub fn create_c_file(&mut self) -> std::io::Result<()>
    {
        if self.buffer.is_empty() {
            println!("Buffer is empty. Load a file first.");
            return Ok(());
        }

        if self.file_name.is_none() {
            panic!("File name not set. Load a file first.");
        }

        let file_name = self.file_name.as_ref().unwrap();
        self.save_path = FileDialog::new()
            .set_file_name(&format!("{}.c", file_name))
            .add_filter("C sources (*.c)", &["c"])
            .add_filter("C++ sources (*.cpp)", &["cpp"])
            .save_file();

        let mut c_file = File::create(self.save_path.as_ref().expect("No save path selected"))?;
        let declaration = format!("unsigned char rawData[{}] = {{\n", self.buffer.len());
        c_file.write_all(declaration.as_bytes())?;

        let mut counter:u8 = 1;
        for &value in &self.buffer
        {
            if counter == 1 {
                c_file.write_all(b"    ")?;
            }

            let str: String = format!("0x{:02X}, ", value);
            c_file.write_all(str.as_bytes())?;

            if counter == 12 {
                c_file.write_all(b"\n")?;
                counter = 1;
            } else {
                counter += 1;
            }
        }

        c_file.write_all(b"\n};")?;
        Ok(())
    }

    fn select_file(&mut self)
    {
        if let Some(selected_path) = FileDialog::new().pick_file()
        {
            self.path = Some(selected_path.clone());
            self.file_name = selected_path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(|stem| stem.to_string());
        }
        else {
            panic!("No file selected");
        }
    }
}