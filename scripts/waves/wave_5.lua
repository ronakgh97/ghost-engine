-- Wave 5: Final Stand
return {
    wave_number = 5,
    name = "Final Stand",
    prep_time = 5.0,
    
    spawns = {
        {
            type = "BasicFighter",
            count = 4,
            interval = 1.5,
            delay = 0.0
        },
        {
            type = "Sniper",
            count = 2,
            interval = 2.0,
            delay = 2.0
        },
        {
            type = "Boss",
            count = 1,
            interval = 1.0,
            delay = 5.0
        },
        {
            type = "Tank",
            count = 2,
            interval = 4.0,
            delay = 10.0
        },
        {
            type = "Sniper",
            count = 3,
            interval = 2.5,
            delay = 15.0
        },
        {
            type = "BasicFighter",
            count = 5,
            interval = 1.0,
            delay = 20.0
        },
    },
    
    on_start = function()
        print_wave_start(5, "Final Stand")
    end,
    
    on_complete = function()
        print_wave_complete(5)
    end
}
