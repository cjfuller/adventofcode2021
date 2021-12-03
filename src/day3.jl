include("./util.jl")

inputs = util.input_lines(3)

digits_at_pos(inputs::Array{String}, pos::Int)::Array{Int} =
    map(l -> parse(Int, l[pos]), inputs)

most_common_bin(inputs::Array{Int}) =
    if sum(inputs) >= length(inputs) / 2
        1
    else
        0
    end

least_common_bin(inputs::Array{Int}) =
    if most_common_bin(inputs) == 1
        0
    else
        1
    end

gamma = 0
epsilon = 0

for i = 1:length(inputs[1])
    g_value = most_common_bin(digits_at_pos(inputs, i))
    e_value = least_common_bin(digits_at_pos(inputs, i))
    global gamma += (g_value << (length(inputs[1]) - i))
    global epsilon += (e_value << (length(inputs[1]) - i))
end

println("Part 1: $(gamma * epsilon)")

function find_rating(inputs::Array{String}, f::Function, curr_pos::Int)::Int
    if length(inputs) == 1
        result = 0
        for (idx, i) in enumerate(inputs[1])
            result += parse(Int, i) << (length(inputs[1]) - idx)
        end
        result
    elseif length(inputs) == 0
        throw("No inputs remaining.")
    else
        target_digit = string(f(digits_at_pos(inputs, curr_pos)))[1]
        find_rating(filter(s -> s[curr_pos] == target_digit, inputs), f, curr_pos + 1)
    end
end

ox_rating = find_rating(inputs, most_common_bin, 1)
co2_rating = find_rating(inputs, least_common_bin, 1)

println("Part 2: $(ox_rating * co2_rating)")
