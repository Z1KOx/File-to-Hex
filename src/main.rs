use std::process::Command;
use device_query::{DeviceQuery, DeviceState, Keycode};

mod fm;
use fm::FM;

fn main()
{
    let mut file_manager = FM::new();
    let device_state: DeviceState = DeviceState::new();

    println!("[0] Exit");
    println!("[1] Select file");
    println!("[2] Create C/C++ file");

    loop
    {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Key0) {
            break;
        }

        if keys.contains(&Keycode::Key1) {
            file_manager.load_file();
        }

        if keys.contains(&Keycode::Key2)
        {
            file_manager.create_c_file().expect("Can't create C file");
            println!("Successfully created a C file");
            break;
        }
    }

    Command::new("cmd")
        .args(&["/C", "pause"])
        .status()
        .expect("Error on executing pause");
}
