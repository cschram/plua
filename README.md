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
    - [ ] Inline functions (maybe?)
- [x] CLI
    - [x] Compilation
    - [x] Glob input
- [x] Environment globals
- [ ] [Lua LS plugin](https://luals.github.io/wiki/plugins/)
- [ ] Editor syntax highlighting
    - [ ] Vim/Neovim
    - [ ] VS Code

## Usage

````
$ plua --help
Lua preprocessor

Usage: plua [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Source plua file

Options:
  -o, --output <OUTPUT>  Output directory. If omitted, the source directory will be used
  -e, --env <ENV>        Pass an environment global in the format name=value
  -q, --quiet            Supress stdout logging
  -d, --debug            Enable debug mode. Metaprograms will be written as a .meta.lua file
  -h, --help             Print help
  -V, --version          Print version
````

## Example

Plua code:

````lua
##!include "lib"

local function log(msg)
    ##if debug then
        print(msg)
    ##end
end

##[[
function pow(n, e)
  return n ^ e
end
]]

log(#[pow(2, 4)]#)

local hello = "world"
print(#["hello"]#)
print(#|"hello"|#)

##function pfunc(name, fn)
    local function #|name|#()
        return pcall(#|fn|#)
    end
##end

##pfunc("throw_error", #{function()
    error("Throwing!")
end}#)

print(throw_error())

##[[
function create_logger(prefix)
    return #{
        function(msg)
            log("[#|prefix|#]: " .. msg)
        end
    }#
end
]]

local plua_log = #|create_logger("Plua")|#

plua_log(":)")

##[[
Plua.warn("Compiler warning")
-- Plua.error("Compiler error")
]]
````

Command:
````
$ plua examples/syntax.plua examples/syntax.lua 
WARN  [plua] Warning on line 104: Compiler warning
INFO  [plua] Wrote lua build\syntax.lua
````

Resulting Lua code:

````lua
print("Hello from plua include!")

local function log(msg)
	print(msg)
end

log(16.0)

local hello = "world"
print("hello")
print(hello)

local function throw_error()
	return pcall(function()
		error("Throwing!")
	end)
end

print(throw_error())

local plua_log = function(msg)
	log("[Plua]: " .. msg)
end

plua_log(":)")
````

See the [examples folder](https://github.com/cschram/plua/tree/main/examples) for more.
