include("./util.jl")

depths = map(l -> parse(Int, l), util.input_lines(1))

changes = depths[2:end] - depths[1:end-1]

num_increases = sum(map(d -> d > 0, changes))

println("Part 1: $num_increases")

windows = []

for i = 1:length(depths)
    if i + 3 <= length(depths)
        push!(windows, depths[i:i+3])
    end
end

window_sums = map(sum, windows)
window_changes = window_sums[2:end] - window_sums[1:end-1]
num_window_increases = sum(map(d -> d > 0, window_changes))

println("Part 2: $num_window_increases")
