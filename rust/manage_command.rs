mod manage_commands {
    
    struct coords {
        xpos: i32,
        ypos: i32,
    }

    struct cmd_in_data {
        command_type: char,
        flightID: String,
        current_pos: coords,
        number_allocated: i64,
        
    }
}