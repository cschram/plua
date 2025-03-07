print("Hello from plua include!")

function log(msg)
            print(msg)
    end



log(16.0)

print("Hello!")
print("Hello!")


    function throw_error()
        return pcall(function()
    error("Throwing!")
end)
    end

print(throw_error())



local log_pow = 
        function(msg)
            log("pow: " .. msg)
        end
    

print(log_pow(16.0))


