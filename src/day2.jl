using Accessors: @set
include("./util.jl")

struct Pos
    horiz::Int
    depth::Int
    aim::Int
end

struct Forward
    amt::Int
end

struct Down
    amt::Int
end

struct Up
    amt::Int
end

Command = Union{Forward,Down,Up}

function apply(pos::Pos, cmd::Forward)::Pos
    @set pos.horiz += cmd.amt
end

function apply(pos::Pos, cmd::Down)::Pos
    @set pos.depth += cmd.amt
end

function apply(pos::Pos, cmd::Up)::Pos
    @set pos.depth -= cmd.amt
end

function parse_cmd(s::AbstractString)::Command
    cmd, s_amt = split(s, ' ')
    amt = parse(Int, s_amt)

    if cmd == "forward"
        Forward(amt)
    elseif cmd == "up"
        Up(amt)
    elseif cmd == "down"
        Down(amt)
    else
        throw(DomainError("Unknown command $cmd"))
    end
end

inputs = util.input_lines(2)
commands = map(parse_cmd, inputs)

state = Pos(0, 0, 0)

for c in commands
    global state = apply(state, c)
end

println("Part 1: $(state.depth * state.horiz)")

function apply2(pos::Pos, cmd::Forward)::Pos
    Pos(pos.horiz + cmd.amt, pos.depth + pos.aim * cmd.amt, pos.aim)
end

function apply2(pos::Pos, cmd::Down)::Pos
    @set pos.aim += cmd.amt
end

function apply2(pos::Pos, cmd::Up)::Pos
    @set pos.aim -= cmd.amt
end

state2 = Pos(0, 0, 0)

for c in commands
    global state2 = apply2(state2, c)
end

println("Part 2: $(state2.depth * state2.horiz)")
