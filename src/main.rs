use std::process::Command;
use device_query::{DeviceQuery, DeviceState, Keycode};

mod fm;
use fm::FM;

fn clear_terminal()
{
    if cfg!(target_os = "windows")
    {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }
}

fn main()
{
    let mut file_manager = FM::new();
    let device_state: DeviceState = DeviceState::new();
    let mut successfully_selected: bool = false;

    println!("[0] Exit");
    println!("[1] Select file");

    'outer: loop
    {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::Key0) {
            break;
        }

        if keys.contains(&Keycode::Key1)
        {
            file_manager.load_file();
            successfully_selected = true;
        }

        if successfully_selected
        {
            clear_terminal();
            println!("[0] Exit");
            println!("[1] Create C/C++ file");

            loop
            {
                let inner_keys: Vec<Keycode> = device_state.get_keys();
                if inner_keys.contains(&Keycode::Key1)
                {
                    file_manager.create_c_file().expect("Can't create C file");
                    println!("Successfully created a C file");

                    break 'outer;
                }

                if inner_keys.contains(&Keycode::Key0) {
                    break;
                }
            }
        }
    }

    Command::new("cmd")
        .args(&["/C", "pause"])
        .status()
        .expect("Error on executing pause");
}
