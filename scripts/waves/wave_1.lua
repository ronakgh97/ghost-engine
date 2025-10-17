-- Wave 1: First Contact
return {
	wave_number = 1,
	name = "First Contact",
	prep_time = 3.0,

	spawns = {

		{
			type = "BasicFighter",
			count = 8,
			interval = 2.0,
			delay = 0.0
		},
		{
			type = "Splitter",
			count = 3,
			interval = 2.5,
			delay = 10.0
		}
	},

	on_start = function()
		print_wave_start(1, "First Contact")
	end,

	on_complete = function()
		print_wave_complete(1)
	end
}

