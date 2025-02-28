# Plua

A WIP Lua preprocessor, inspired by the preprocessor of [Nelua](https://nelua.io/overview/#preprocessor).

## Features

- [x] Metaprogramming
    - [x] Compile time Lua
    - [x] Interpolation
- [x] CLI 
- [x] Environment globals
- [ ] Metacode includes
- [ ] Multiline metacode
- [ ] Error and warning API
- [ ] [Lua LS plugin](https://luals.github.io/wiki/plugins/)
- [ ] Editor syntax highlighting
    - [ ] Vim/Neovim
    - [ ] VS Code
- [ ] TOML config

### Possible Features

- [ ] Utilities/common library for metaprograms
- [ ] AST API for code generation in metaprograms

## Usage

```
$ plua --help
Lua preprocessor/metaprogramming language.

Usage: plua [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input plua file
  <OUTPUT>  Output lua file

Options:
  -f, --format     Format the lua output
  -m, --meta       Output the metaprogram as a .meta.lua file alongside the output
  -e, --env <ENV>  Pass an environment global in the format name=value
  -h, --help       Print help
  -V, --version    Print version
```

## Example

Plua code:

```lua
##-------------------------------------------------------------------------------
##-- Basic if/else structure
##-------------------------------------------------------------------------------

##-- This should evaluate to the following lua code:
##-- ```
##-- function isTest()
##--   return true
##-- end
##-- ```
## local test = true
function isTest()
  ## if test then
  return true
  ## else
  return false
  ## end
end
assert(isTest())

##-------------------------------------------------------------------------------
##-- Defining functions and inlining values
##-------------------------------------------------------------------------------

## function pow(n, e)
##   return n ^ e
## end

##-- This should evaluate to the following lua code:
##-- ```
##-- __emit("assert(" .. (pow(2, 3) .. " == 8)")
##-- ```
assert(#[pow(2, 3)]# == 8)

##-------------------------------------------------------------------------------
##-- Emitting code
##-------------------------------------------------------------------------------

##-- This should evaluate to the following metaprogram code:
##-- ```
##-- function increment(identifier, amount)
##--   __emit("  " .. (identifier) .. " = " .. (identifier) .. " + " .. (amount))
##-- end
##-- ```
## function increment(identifier, amount)
  #{identifier}# = #{identifier}# + #[amount]#
## end

##-- This should evaluate to the following resulting code:
##-- ```
##-- do
##--   local v = 1
##--   v = v + 2
##--   assert(v == 3)
##-- end
##-- ```
do
  local v = 1
  ## increment("v", 2)
  assert(v == 3)
end

##-------------------------------------------------------------------------------
##-- TODO: Meta Program Includes
##-- Includes another plua file inline into the metaprogram, allowing metaprogram
##-- functions and variables to be used across files.
##-------------------------------------------------------------------------------

##-- Defines a local `foo` as `"bar"`.
##!include "include"

## local foo = "bar"
print(#[foo]#)

##-------------------------------------------------------------------------------
##-- Envrionment Variables
##-- Globals can be defined in the preprocessor and used in plua code.
##-------------------------------------------------------------------------------

## if debug then
print("Debug Mode")
## else
print("Relase Mode")
## end
```

Command:
```
$ plua test.plua test.lua --format --env debug=true
Wrote test.lua
```

Outputted Lua code:

```lua
function isTest()
	return true
end
assert(isTest())

assert(8.0 == 8)

do
	local v = 1
	v = v + 2
	assert(v == 3)
end

print("bar")

print("Relase Mode")```
