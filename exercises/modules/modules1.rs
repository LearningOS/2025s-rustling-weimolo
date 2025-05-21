// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

//AI



mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> Result<String, String> {
        // 这里假设获取配方总是成功，实际应用中可能会失败
        Ok(String::from("Ginger"))
    }

    pub fn make_sausage() {
        match get_secret_recipe() {
            Ok(recipe) => {
                println!("Making sausage with recipe: {}", recipe);
                println!("sausage!");
            },
            Err(e) => {
                println!("Failed to get recipe: {}", e);
            }
        }
    }
}

fn main() {
    sausage_factory::make_sausage();
}
