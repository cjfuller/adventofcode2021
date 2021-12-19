include("./util.jl")
import LinearAlgebra
using Graphs: SimpleGraph, is_connected, add_edge!, a_star

struct Orientation
    rotation_matrix::Array{Int,2}
    translation::Array{Int,1}
end

struct BeaconPosition
    rel_position::Array{Int,1}
end

struct Scanner
    index::Int
    detected_beacons::Array{BeaconPosition}
    abs_pos::Array{Int,1}
end

@enum Dim X Y Z

function rot(num_turns::Int, dim::Dim)::Array{Int,2}
    num_turns = num_turns % 4
    if num_turns == 0
        return [
            1 0 0
            0 1 0
            0 0 1
        ]
    end
    if dim == X
        if num_turns == 1
            [
                1 0 0
                0 0 -1
                0 1 0
            ]
        elseif num_turns == 2
            [
                1 0 0
                0 -1 0
                0 0 -1
            ]
        elseif num_turns == 3
            [
                1 0 0
                0 0 1
                0 -1 0
            ]
        end
    elseif dim == Y
        if num_turns == 1
            [
                0 0 1
                0 1 0
                -1 0 0
            ]
        elseif num_turns == 2
            [
                -1 0 0
                0 1 0
                0 0 -1
            ]
        elseif num_turns == 3
            [
                0 0 -1
                0 1 0
                1 0 0
            ]
        end
    elseif dim == Z
        if num_turns == 1
            [
                0 -1 0
                1 0 0
                0 0 1
            ]
        elseif num_turns == 2
            [
                -1 0 0
                0 -1 0
                0 0 1
            ]
        elseif num_turns == 3
            [
                0 1 0
                -1 0 0
                0 0 1
            ]
        end
    end
end

function apply(orient::Orientation, scanner::Scanner)::Scanner
    Scanner(
        scanner.index,
        map(bp -> BeaconPosition((orient.rotation_matrix * bp.rel_position) + orient.translation), scanner.detected_beacons),
        (orient.rotation_matrix * scanner.abs_pos) + orient.translation
    )
end

function apply_inv(orient::Orientation, scanner::Scanner)::Scanner
    Scanner(
        scanner.index,
        map(bp -> BeaconPosition(inv(orient.rotation_matrix) * (bp.rel_position - orient.translation)), scanner.detected_beacons),
        inv(orient.rotation_matrix) * (scanner.abs_pos - orient.translation)
    )
end

function parse_input()::Array{Scanner}
    lines = util.input_lines(19)
    scanners = []
    new_scanner_r = r"--- scanner (\d+) ---"
    triple_r = r"(-?\d+),(-?\d+),(-?\d+)"
    curr_scanner = -1
    curr_scanner_triples = []
    for line in lines
        m = match(new_scanner_r, line)
        if !isnothing(m)
            if curr_scanner != -1
                push!(scanners, Scanner(curr_scanner, curr_scanner_triples, [0, 0, 0]))
            end
            curr_scanner = parse(Int, m.captures[1]) + 1
            curr_scanner_triples = []
        end
        m = match(triple_r, line)
        if !isnothing(m)
            push!(
                curr_scanner_triples,
                BeaconPosition([
                    parse(Int, m.captures[1]),
                    parse(Int, m.captures[2]), parse(Int, m.captures[3])
                ])
            )
        end
    end
    push!(scanners, Scanner(curr_scanner, curr_scanner_triples, [0, 0, 0]))
    scanners
end

function check_align(s0::Scanner, s1::Scanner, orient::Orientation)::Bool
    bs0 = map(it -> it.rel_position, s0.detected_beacons)
    bs1 = map(it -> it.rel_position, apply(orient, s1).detected_beacons)
    length(intersect(bs0, bs1)) >= 12
end

function try_align(s0::Scanner, s1::Scanner)::Union{Orientation,Nothing}
    for x_turns = 0:3
        for y_turns = 0:3
            for z_turns = 0:3
                rot_mat = rot(x_turns, X) * rot(y_turns, Y) * rot(z_turns, Z)
                for b0 in s0.detected_beacons
                    for b1 in s1.detected_beacons
                        offset = b0.rel_position - (rot_mat * b1.rel_position)
                        orient = Orientation(rot_mat, offset)
                        if check_align(s0, s1, orient)
                            return orient
                        end
                    end
                end
            end
        end
    end
    nothing
end

function align(scanners::Array{Scanner})::Dict{Tuple{Int,Int},Orientation}
    alignments = Dict{Tuple{Int,Int},Orientation}()
    for s0 in scanners
        for s1 in scanners
            if s1.index <= s0.index
                continue
            end
            # println("Aligning $(s0.index) to $(s1.index)")
            alignment = try_align(s0, s1)
            if !isnothing(alignment)
                # println(alignment)
                alignments[(s0.index, s1.index)] = alignment
            end
        end
    end
    alignments
end

function transform_to_1(scanners::Array{Scanner}, alignment::Dict{Tuple{Int,Int},Orientation})::Array{Scanner}
    g = SimpleGraph(length(scanners))
    for (a, b) in keys(alignment)
        add_edge!(g, a, b)
    end
    @assert is_connected(g)
    output = []
    for scanner in scanners
        curr_transformed = scanner
        path_to_1 = a_star(g, scanner.index, 1)
        for e in path_to_1
            if haskey(alignment, (e.src, e.dst))
                curr_transformed = apply_inv(alignment[(e.src, e.dst)], curr_transformed)
            else
                curr_transformed = apply(alignment[(e.dst, e.src)], curr_transformed)
            end
        end
        push!(output, curr_transformed)
    end
    output
end

function extract_beacons(scanners::Array{Scanner})::Set{Array{Int,1}}
    Set([pos.rel_position for scan in scanners for pos in scan.detected_beacons])
end

function max_manhattan_distance(scanners::Array{Scanner})::Int
    best = 0
    for s0 in scanners
        for s1 in scanners
            best = max(best, LinearAlgebra.norm(s0.abs_pos - s1.abs_pos, 1))
        end
    end
    best
end

scanners = parse_input()
alignment = align(scanners)
transformed = transform_to_1(scanners, alignment)
beacons = extract_beacons(transformed)
println("Part 1: $(length(beacons))")
dist = max_manhattan_distance(transformed)
println("Part 2: $dist")
