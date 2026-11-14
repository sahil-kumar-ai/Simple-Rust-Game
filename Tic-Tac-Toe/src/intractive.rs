use dialoguer::Select;

pub fn get_option() -> &'static str {
    let options = vec![
        "Continue Game",
        "Load New Game",
        "Exit",
    ];

    let selection = Select::new()
        .with_prompt("What do you want to do?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    options[selection]
}