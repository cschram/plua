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
		error("Cannot interpolate value of type " .. t)
	else
		return tostring(val)
	end
end
--- PLUA METAPROGRAM ---
Plua.emit("print(\"Hello from plua include!\")\n")


Plua.emit("\nlocal function log(msg)\n    ")
if debug then


Plua.emit("        print(msg)\n    ")
end


Plua.emit("end\n\n")

function pow(n, e)
  return n ^ e
end

Plua.emit("\n\nlog(")
Plua.emit(Plua.format_value(pow(2, 4)))
Plua.emit(")\n\nlocal hello = \"world\"\nprint(")
Plua.emit(Plua.format_value("hello"))
Plua.emit(")\nprint(")
Plua.emit("hello")
Plua.emit(")\n\n")
function pfunc(name, fn)


Plua.emit("    local function ")
Plua.emit(name)
Plua.emit("()\n        return pcall(")
Plua.emit(fn)
Plua.emit(")\n    end\n")
end


Plua.emit("\n")
pfunc("throw_error", 
"function()\n    error(\"Throwing!\")\nend"
)


Plua.emit("\nprint(throw_error())\n\n")

function create_logger(prefix)
    return 
"\n        function(msg)\n            log(\"[" .. prefix .. "]: \" .. msg)\n        end\n    "

end

Plua.emit("\n\nlocal plua_log = ")
Plua.emit(create_logger("Plua"))
Plua.emit("\n\nplua_log(\":)\")\n\n")

Plua.warn("Compiler warning")
-- Plua.error("Compiler error")

Plua.emit("\n")--- PLUA FOOTER ---
return table.concat(__output_lines)
