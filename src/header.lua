-----------------------------------------------------------------------------------------------------------------------
--- Plua Header Start
--- These functions are used internally in the Plua preprocessor to enable metaprograms to work.
-----------------------------------------------------------------------------------------------------------------------

local __output_lines = {}

--- Emit output lua code
function __emit(line)
	if line then
		table.insert(__output_lines, line)
	end
end

--- Check if a value is an array
function __is_array(val)
	if type(val) ~= "table" then
		return false
	end
	local count = 0
	for k, v in pairs(val) do
		if type(v) ~= "number" then
			return false
		else
			count = count + 1
		end
	end
	for i = 1, count do
		if not val[i] and type(val[i]) ~= "nil" then
			return false
		end
	end
	return true
end

--- Format a value into a Lua literal
function __format_value(val)
	local t = type(val)
	if t == "string" then
		return '"' .. val .. '"'
	elseif t == "table" then
		local fields = {}
		if __is_array(val) then
			for _, item in pairs(val) do
				table.insert(fields, item)
			end
		else
			for k, v in pairs(val) do
				table.insert(fields, k .. "=" .. __format_value(v))
			end
		end
		return "{" .. table.concat(fields, ",") .. "}"
	elseif t == "function" or t == "thread" or t == "userdata" then
		error("Cannot interpolate value of type " .. t)
	else
		return tostring(val)
	end
end

-----------------------------------------------------------------------------------------------------------------------
--- Plua Header End
-----------------------------------------------------------------------------------------------------------------------
