-------------------------------------------------------------------------------
-- Basic if/else structure
-------------------------------------------------------------------------------

function isTest()
	return true
end
assert(isTest())

-------------------------------------------------------------------------------
-- Defining functions and inlining values
-------------------------------------------------------------------------------

assert(8.0 == 8)

-------------------------------------------------------------------------------
-- Emitting code
-------------------------------------------------------------------------------

do
	local v = 1
	v = v + 2
	assert(v == 3)
end

-------------------------------------------------------------------------------
-- Meta Program Includes
-- Includes another plua file inline into the metaprogram, allowing metaprogram
-- functions and variables to be used across files.
-------------------------------------------------------------------------------

-- Defines a local `foo` as `"bar"`.

print("bar")

-------------------------------------------------------------------------------
-- Environment Variables
-- Globals can be defined in the preprocessor and used in plua code.
-------------------------------------------------------------------------------

print("Debug Mode")
