--- PLUA HEADER ---
local __output_lines = {}

function Plua.emit(line)
	if line then
		table.insert(__output_lines, line)
	end
end

function Plua.is_array(val)
	if type(val) ~= "table" then
		return false
	end
	local count = 0
	for _, v in pairs(val) do
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

function Plua.format_value(val)
	local t = type(val)
	if t == "string" then
		return '"' .. val .. '"'
	elseif t == "table" then
		local fields = {}
		if Plua.is_array(val) then
			fields = val
		else
			for k, v in pairs(val) do
				table.insert(fields, k .. "=" .. Plua.format_value(v))
			end
		end
		return "{" .. table.concat(fields, ",") .. "}"
	elseif t == "function" or t == "thread" or t == "userdata" then
		error("Interpolating " .. t .. " values is not supported")
	else
		return tostring(val)
	end
end
--- PLUA METAPROGRAM ---
