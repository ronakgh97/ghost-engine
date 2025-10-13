-- Wave 2: Sharpshooters
return {
    wave_number = 2,
    name = "Sharpshooters",
    prep_time = 3.0,
    
    spawns = {
        {
            type = "BasicFighter",
            count = 10,
            interval = 1.5,
            delay = 0.0
        },
        {
            type = "Sniper",
            count = 5,
            interval = 3.0,
            delay = 3.0
        },
    },
    
    on_start = function()
        print_wave_start(2, "Sharpshooters")
    end,
    
    on_complete = function()
        print_wave_complete(2)
    end
}
