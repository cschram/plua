# Plua

A WIP Lua preprocessor, inspired by the preprocessor of [Nelua](https://nelua.io/overview/#preprocessor).

## Features

- [x] Metaprogramming
    - [x] Compile time Lua
    - [x] Interpolation
- [x] CLI
    - [ ] Glob input
    - [ ] TOML config
- [x] Environment globals
- [x] Metacode includes
- [ ] Multiline metacode
- [x] Error and warning API
- [ ] [Lua LS plugin](https://luals.github.io/wiki/plugins/)
- [ ] Editor syntax highlighting
    - [ ] Vim/Neovim
    - [ ] VS Code

### Possible Features

- [ ] Utilities/common library for metaprograms
- [ ] AST API for code generation in metaprograms

## Usage

```
$ plua --help
Lua preprocessor

Usage: plua [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input plua file
  <OUTPUT>  Output lua file

Options:
  -e, --env <ENV>  Pass an environment global in the format name=value
  -q, --quiet      Supress stdout logging
  -h, --help       Print help
  -V, --version    Print version
```

## Example

Plua code:

```lua
-------------------------------------------------------------------------------
-- Basic if/else structure
-------------------------------------------------------------------------------

## local test = true
function isTest()
  ## if test then
    return true
  ## else
    return false
  ## end
end
assert(isTest())

-------------------------------------------------------------------------------
-- Defining functions and inlining values
-------------------------------------------------------------------------------

## function pow(n, e)
##   return n ^ e
## end

assert(#[pow(2, 3)]# == 8)

-------------------------------------------------------------------------------
-- Emitting code
-------------------------------------------------------------------------------

##-- This evaluates to the following metaprogram code:
##-- ```
##-- function increment(identifier, amount)
##--   __emit("  " .. (identifier) .. " = " .. __format_value(identifier) .. " + " .. __format_value(amount))
##-- end
##-- ```
## function increment(identifier, amount)
  #{identifier}# = #{identifier}# + #[amount]#
## end

do
  local v = 1
  ## increment("v", 2)
  assert(v == 3)
end

-------------------------------------------------------------------------------
-- TODO: Meta Program Includes
-- Includes another plua file inline into the metaprogram, allowing metaprogram
-- functions and variables to be used across files.
-------------------------------------------------------------------------------

-- Defines a local `foo` as `"bar"`.
##!include "include"

print(#[foo]#)

-------------------------------------------------------------------------------
-- Environment Variables
-- Globals can be defined in the preprocessor and used in plua code.
-------------------------------------------------------------------------------

## if debug then
  print("Debug Mode")
## else
  print("Release Mode")
## end
```

Command:
```
$ plua test.plua test.lua --env debug=true
Wrote lua test.lua
```

Outputted Lua code:

```lua
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
-- TODO: Meta Program Includes
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
```
