module util

function load_input(day::Int, strip::Bool = true)::String
    s = open(joinpath(dirname(@__DIR__), "inputs/day_$day.txt")) do f
        read(f, String)
    end
    if strip
        Base.strip(s)
    else
        s
    end
end

end
