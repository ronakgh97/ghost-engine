-- Wave 4: Overwhelming Force
return {
	wave_number = 4,
	name = "Overwhelming Force",
	prep_time = 3.0,

	spawns = {
		{
			type = "BasicFighter",
			count = 8,
			interval = 2.0,
			delay = 0.0
		},
		{
			type = "Sniper",
			count = 3,
			interval = 2.0,
			delay = 3.0
		},
		{
			type = "Tank",
			count = 3,
			interval = 3.5,
			delay = 7.5
		},
		{
			type = "Elite",
			count = 1,
			interval = 5.0,
			delay = 15.0
		},
	},

	on_start = function()
		print_wave_start(4, "Overwhelming Force")
	end,

	on_complete = function()
		print_wave_complete(4)
	end
}
