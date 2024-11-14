use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::Command;
use rfd::FileDialog;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn get_path() -> File
{
    let path = FileDialog::new()
        .pick_file()
        .expect("No file selected");

    let file = File::open(path).expect("Failed to open file");

    file
}

fn get_file_buffer(file: File) -> Vec<u8>
{
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer).expect("Can't read buffer");

    buffer
}

fn create_c_file(buffer: &Vec<u8>) -> std::io::Result<()>
{
    let save_path = FileDialog::new()
        .set_file_name("test.c")
        .add_filter("C sources (*.c)", &["c"])
        .save_file();

    let mut c_file: File = File::create(save_path.unwrap())?;

    let declaration: String = format!("unsigned char rawData[{}] = {{\n", buffer.len());
    c_file.write_all(declaration.as_bytes())?;

    let mut counter: i32 = 1;
    for value in buffer
    {
        if counter == 12 {
            c_file.write_all(b"\n")?;
            counter = 1;
        }

        if counter == 1 {
            c_file.write_all(b"    ")?;
        }

        let str = if *value <= 0xF {
            format!("0x0{:X}, ", value)
        } else {
            format!("0x{:X}, ", value)
        };
        c_file.write_all(str.as_bytes())?;

        counter += 1;
    }

    c_file.write_all(b"\n};")?;

    Ok(())
}

fn main()
{
    let mut buffer: Vec<u8> = Vec::new();
    let device_state: DeviceState = DeviceState::new();

    println!("[0] Exit");
    println!("[1] Select file");
    println!("[2] Create C file");

    loop
    {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Key0) {
            break;
        }

        if keys.contains(&Keycode::Key1)
        {
            let path: File = get_path();
            buffer = get_file_buffer(path);
        }

        if keys.contains(&Keycode::Key2)
        {
            if buffer.is_empty() {
                println!("You need to select a file");
                break;
            }

            create_c_file(&buffer)
                .expect("Can't create C file");

            println!("Successfully created a C file");
            break;
        }
    }

    Command::new("cmd").args(&["/C", "pause"]).status()
        .expect("Error on executing pause");
}
