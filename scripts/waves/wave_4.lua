-- Wave 4: Overwhelming Force
return {
	wave_number = 4,
	name = "Overwhelming Force",
	prep_time = 3.0,

	spawns = {
		{
			type = "BasicFighter",
			count = 10,
			interval = 1.2,
			delay = 0.0
		},
		{
			type = "Sniper",
			count = 5,
			interval = 2.0,
			delay = 3.0
		},
		{
			type = "Tank",
			count = 3,
			interval = 3.0,
			delay = 8.0
		},
		{
			type = "Elite",
			count = 6,
			interval = 1.0,
			delay = 12.0
		},
	},

	on_start = function()
		print_wave_start(4, "Overwhelming Force")
	end,

	on_complete = function()
		print_wave_complete(4)
	end
}
