# Plua

<p>
    <a href="https://github.com/cschram/plua/blob/main/LICENSE">  
        <img alt="GitHub License" src="https://img.shields.io/github/license/cschram/plua?style=for-the-badge">
    </a>
</p>

A WIP Lua preprocessor, inspired by [Nelua's preprocessor](https://nelua.io/overview/#preprocessor).

## Features

- [x] Metaprogramming
    - [x] Compile time Lua
    - [x] Interpolation
    - [x] Metacode includes
    - [x] Emit compiler warnings and errors
- [x] CLI
    - [x] Compilation
    - [ ] Glob input
- [x] Environment globals
- [ ] [Lua LS plugin](https://luals.github.io/wiki/plugins/)
- [ ] Editor syntax highlighting
    - [ ] Vim/Neovim
    - [ ] VS Code

## Usage

````
$ plua --help
Lua preprocessor

Usage: plua [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input plua file
  <OUTPUT>  Output lua file

Options:
  -e, --env <ENV>  Pass an environment global in the format name=value
  -q, --quiet      Supress stdout logging
  -d, --debug      Enable debug mode. Metaprograms will be written as a .meta.lua file
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
local function is_test()
    ## if test then
        return true
    ## else
        return false
    ## end
end
assert(is_test())

-------------------------------------------------------------------------------
-- Environment Variables
-- Globals can be defined in the preprocessor and used in Plua code.
-------------------------------------------------------------------------------

## if debug then
    print("Debug Mode")
## else
    print("Release Mode")
## end

-------------------------------------------------------------------------------
-- Defining functions and inlining values
-------------------------------------------------------------------------------

##```
function pow(n, e)
    return n ^ e
end
```##


assert(#[pow(2, 3)]# == 8)

-------------------------------------------------------------------------------
-- Emitting code
-- While `#[value]#` will format `value` as a literal in the output Lua,
-- `#{value}#` will output `value` as-is, meaning a string can be used as any
-- piece of Lua code, for example as an identifier. Be aware that types other
-- than string and number may not behave as you expect when used this way.
-------------------------------------------------------------------------------

## function increment(identifier, amount)
    #{identifier}# = #{identifier}# + #[amount]#
## end

do
    local v = 1
    ## increment("v", 2)
    assert(v == 3)
end

-------------------------------------------------------------------------------
-- Meta Program Includes
-- Other Plua files can be included inline to allow re-use of functions and
-- values.
-------------------------------------------------------------------------------

-- Defines a local `foo` as `"bar"`.
##!include "include"

print(#[foo]#)

-------------------------------------------------------------------------------
-- Meta Program Built-ins
-- Apart from the Lua 5.4 standard library, a collection of functions are
-- available to metaprograms under the `Plua` namespace.
-------------------------------------------------------------------------------

##```
-- Emit Lua code.
Plua.emit("local one = 1")
-- Format a value as a Lua literal (identical to #[]#)
Plua.emit(Plua.format_value({ table = true }))
-- Emit a compiler warning
Plua.warn("Test warning")
-- Emit a compiler error, immediately stopping metaprogram execution
-- Plua.error("Test error")
```##
````

Command:
````
$ plua examples/syntax.plua examples/syntax.lua --env debug=true
WARN  [plua] Warning on line 103: Test warning
INFO  [plua] Wrote lua examples/syntax.lua
````

Resulting Lua code:

````lua
local function is_test()
        return true
end
assert(is_test())
    print("Debug Mode")
assert(8.0 == 8)
do
    local v = 1
    v = v + 2
    assert(v == 3)
end
print("bar")
local one = 1
{table=true}
````
