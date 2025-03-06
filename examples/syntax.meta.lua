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
Plua.emit()
 local test = true
Plua.emit("local function is_test()")
 if test then
Plua.emit("        return true")
 else
Plua.emit("        return false")
 end
Plua.emit("end")
Plua.emit("assert(is_test())")
Plua.emit()
Plua.emit()
 if debug then
Plua.emit("    print(\"Debug Mode\")")
 else
Plua.emit("    print(\"Release Mode\")")
 end
Plua.emit()
Plua.emit()

function pow(n, e)
    return n ^ e
end

Plua.emit()
Plua.emit()
Plua.emit("assert(" .. Plua.format_value(pow(2, 3)) .. " == 8)")
Plua.emit()
Plua.emit()
 function increment(identifier, amount)
Plua.emit("    " .. (identifier) .. " = " .. (identifier) .. " + " .. Plua.format_value(amount))
 end
Plua.emit()
Plua.emit("do")
Plua.emit("    local v = 1")
 increment("v", 2)
Plua.emit("    assert(v == 3)")
Plua.emit("end")
Plua.emit()
Plua.emit()
 local foo = "bar"
Plua.emit()
Plua.emit()
Plua.emit("print(" .. Plua.format_value(foo) .. ")")
Plua.emit()
Plua.emit()

-- Emit Lua code.
Plua.emit("local one = 1")
-- Format a value as a Lua literal (identical to #[]#)
Plua.emit(Plua.format_value({ table = true }))
-- Emit a compiler warning
Plua.warn("Test warning")
-- Emit a compiler error, immediately stopping metaprogram execution
-- Plua.error("Test error")

Plua.emit()
--- PLUA FOOTER ---
return table.concat(__output_lines, "\n")
