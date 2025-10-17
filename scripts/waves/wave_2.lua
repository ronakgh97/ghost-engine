-- Wave 2: Sharpshooters
return {
	wave_number = 2,
	name = "Sharpshooters",
	prep_time = 2.0,

	spawns = {
		{
			type = "BasicFighter",
			count = 3,
			interval = 1,
			delay = 0.0
		},
		{
			type = "Sniper",
			count = 3,
			interval = 3.0,
			delay = 5.0
		},
		{
			type = "Splitter",
			count = 3,
			interval = 2.5,
			delay = 15.0
		},
	},

	on_start = function()
		print_wave_start(2, "Sharpshooters")
	end,

	on_complete = function()
		print_wave_complete(2)
	end
}
