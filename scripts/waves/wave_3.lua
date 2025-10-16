-- Wave 3: Heavy Assault
return {
	wave_number = 3,
	name = "Heavy Assault",
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
			interval = 2.5,
			delay = 2.5
		},
		{
			type = "Tank",
			count = 2,
			interval = 4.0,
			delay = 7.5
		},
		{
			type = "Healer",
			count = 2,
			interval = 3.0,
			delay = 10.0
		},
		{
			type = "Splitter",
			count = 6,
			interval = 2.0,
			delay = 15.0
		}
	},

	on_start = function()
		print_wave_start(3, "Heavy Assault")
	end,

	on_complete = function()
		print_wave_complete(3)
	end
}
