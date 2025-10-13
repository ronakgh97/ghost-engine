-- Helper functions available to all Lua scripts
function print_wave_start(wave_num, name)
	print(" Wave " .. wave_num .. ": " .. name)
end

function print_wave_complete(wave_num)
	print("✓ Wave " .. wave_num .. " Complete!")
end