mod day_01 {
    pub mod guessing_game;
}

mod day_02 {
    pub mod compound_data_types;
    pub mod scalar_data_types;
    pub mod variables;
}

fn main() {
    // day-01: guessing game
    // day_01::guessing_game::play_guessing_game();

    // day-02: variables
    // day_02::variables::variables_in_rust();

    // day-02: scalar-data-types
    // day_02::scalar_data_types::scalar_data_types_in_rust();

    // day-02: compound-data-types
    day_02::compound_data_types::compound_data_types_in_rust();
}
