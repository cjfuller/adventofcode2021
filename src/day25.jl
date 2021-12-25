include("./util.jl")

using Test: @test

function line2nums(line::AbstractString)::Array{Int8,1}
    [
        if c == '.'
            0
        elseif c == '>'
            1
        elseif c == 'v'
            2
        else
            throw("Invalid character $c")
        end
        for c in line
    ]
end

parse(s::Vector)::Array{Int8,2} =
    transpose(hcat(map(line2nums, s)...))

nums = parse(util.input_lines(25))

@enum Direction east south

function next_idx(idx::CartesianIndex, direction::Direction, max::Int64)::CartesianIndex
    if direction == east
        next = idx[2] + 1
        if next > max
            next = 1
        end
        CartesianIndex(idx[1], next)
    else
        next = idx[1] + 1
        if next > max
            next = 1
        end
        CartesianIndex(next, idx[2])
    end
end

function step(curr::Array{Int8,2})::Bool
    east_shift = hcat(curr[:, 2:end], curr[:, 1:1])
    east_movers = findall((east_shift .== 0) .* (curr .== 1))
    east_max = size(curr)[2]
    for idx in east_movers
        target = next_idx(idx, east, east_max)
        curr[idx] = 0
        curr[target] = 1
    end

    south_shift = vcat(curr[2:end, :], curr[1:1, :])
    south_movers = findall((south_shift .== 0) .* (curr .== 2))
    south_max = size(curr)[1]
    for idx in south_movers
        target = next_idx(idx, south, south_max)
        curr[idx] = 0
        curr[target] = 2
    end
    length(south_movers) > 0 || length(east_movers) > 0
end

count = 1

test_input = split("""v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>""", '\n')

test_nums = parse(test_input)

test_count = 1

while step(test_nums)
    global test_count += 1
end

@test test_count == 58

n_p1 = copy(nums)

while step(n_p1)
    global count += 1
end

println("Part 1: $count")
