# Plua

A Lua preprocessor/metaprogramming language.

## To Do

- [ ] Meta includes
- [ ] Environment globals

## Example

Plua code:

```lua
-------------------------------------------------------------------------------
-- Basic if/else structure
-------------------------------------------------------------------------------

-- This should evaluate to the following resulting code:
-- ```
-- function isTest()
--   return true
-- end
-- ```
## local test = true
function isTest()
  ## if test then
  return true
  ## else
  return false
  ## end
end
expect(isTest())

-------------------------------------------------------------------------------
-- Defining functions and inlining values
-------------------------------------------------------------------------------

## function pow(n, e)
##   return n ^ e
## end

-- This should evaluate to the following metaprogram code:
-- ```
-- __emit("assert(" .. (pow(2, 3) .. " == 8)")
-- ```
assert(#[pow(2, 3)]# == 8)

-------------------------------------------------------------------------------
-- Emitting code
-------------------------------------------------------------------------------

-- This should evaluate to the following metaprogram code:
-- ```
-- function increment(identifier, amount)
--   __emit("  " .. (identifier) .. " = " .. (identifier) .. " + " .. (amount))
-- end
-- ```
## function increment(identifier, amount)
  #[identifier]# = #[identifier]# + #[amount]#
## end

-- This should evaluate to the following resulting code:
-- ```
-- do
--   local v = 1
--   v = v + 2
--   assert(v == 3)
-- end
-- ```
do
  local v = 1
  ## increment("v", 2)
  assert(v == 3)
end

-------------------------------------------------------------------------------
-- Meta Program Includes
-- Includes another plua file inline into the metaprogram, allowing metaprogram
-- functions and variables to be used across files.
-------------------------------------------------------------------------------

-- Defines a local `foo` as `"bar"`.
##!include "include"

print("#[foo]#")
```

Outputted Lua code:

```lua
-------------------------------------------------------------------------------
-- Basic if/else structure
-------------------------------------------------------------------------------
-- This should evaluate to the following resulting code:
-- ```
-- function isTest()
--   return true
-- end
-- ```
function isTest()
  return true
end
expect(isTest())
-------------------------------------------------------------------------------
-- Defining functions and inlining values
-------------------------------------------------------------------------------
-- This should evaluate to the following metaprogram code:
-- ```
-- __emit("assert(" .. (pow(2, 3) .. " == 8)")
-- ```
assert(8.0 == 8)
-------------------------------------------------------------------------------
-- Emitting code
-------------------------------------------------------------------------------
-- This should evaluate to the following metaprogram code:
-- ```
-- function increment(identifier, amount)
--   __emit("  " .. (identifier) .. " = " .. (identifier) .. " + " .. (amount))
-- end
-- ```
-- This should evaluate to the following resulting code:
-- ```
-- do
--   local v = 1
--   v = v + 2
--   assert(v == 3)
-- end
-- ```
do
  local v = 1
  v = v + 2
  assert(v == 3)
end
-------------------------------------------------------------------------------
-- TODO: Meta Program Includes
-- Includes another plua file inline into the metaprogram, allowing metaprogram
-- functions and variables to be used across files.
-------------------------------------------------------------------------------
-- Defines a local `foo` as `"bar"`.
local foo = "bar"
print("bar")
```
