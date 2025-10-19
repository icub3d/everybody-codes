# A helper to print messages in a consistent style
def "print-info" [message: string] {
    print $"✅ ($message)"
}

# A helper to print error messages
def "print-error" [message: string] {
    print -e $"❌ ERROR: ($message)"
}

# Clear the terminal and scrollback so watch output starts from a clean buffer.
def "reset-terminal" [] {
    print -n (ansi clsb)
    print -n (ansi home)
}

# Normalize and convert a debug-formatted Rust Duration string into a Nushell duration
def "parse-duration" [value: string] {
    $value
    | str trim
    | split row " "
    | where {|part| not ($part | str trim | is-empty) }
    | each {|part|
        let normalized = (
            if ($part | str ends-with "ms") {
                $part
            } else if ($part | str ends-with "µs") {
                $part
            } else if ($part | str ends-with "us") {
                $part | str replace --regex "us$" "µs"
            } else if ($part | str ends-with "ns") {
                $part
            } else if ($part | str ends-with "s") {
                $part | str replace --regex "s$" "sec"
            } else if ($part | str ends-with "m") {
                $part | str replace --regex "m$" "min"
            } else if ($part | str ends-with "h") {
                $part | str replace --regex "h$" "hr"
            } else if ($part | str ends-with "d") {
                $part | str replace --regex "d$" "day"
            } else if ($part | str ends-with "w") {
                $part | str replace --regex "w$" "wk"
            } else {
                $part
            }
        )

        try { $normalized | into duration } catch { 0ns }
    }
    | math sum
}

# run all quests for a given year
export def "ec all" [year: int] {
    let crate_name = $"ec_($year)"
    let bin_path = $"solutions/($crate_name)/src/bin"

    let results = (
        1..25
        | each {|quest|
            let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
            let quest_file = $"($bin_path)/($quest_mod).rs"

            if not ($quest_file | path exists) {
                []
            } else {
                let run = (cargo run --release -q -p $crate_name --bin $quest_mod | complete)

                if $run.exit_code != 0 {
                    print-error $"Quest ($quest_mod) failed with exit code ($run.exit_code)"
                    let stderr = ($run.stderr | default "" | str trim)
                    if not ($stderr | is-empty) {
                        print $stderr
                    }
                    []
                } else {
                    $run.stdout
                    | str trim
                    | lines
                    | parse "{part} {time} {solution}"
                    | insert quest $quest
                    | insert year $year
                }
            }
        }
        | flatten
    )

    if ($results | is-empty) {
        print-info "No puzzle output detected."
    } else {
        print $"🐥 Everybody Codes ($year) Summary 🐥"
        let table = (
            $results
            | select year quest part time solution
            | sort-by year quest part
        )

        let total_duration = (
            $results
            | each {|row| parse-duration ($row.time | default "0ns") }
            | math sum
        )

        print $"total time: ($total_duration)"
        $table
    }
}

# Everybody Codes runner
export def "ec" [year: int, quest: int] {
    let crate_name = $"ec_($year)"
    let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
    print $"🐥 Everybody Codes ($year) - Quest ($quest) 🐥"
    cargo run --release -q -p $crate_name --bin $quest_mod
}

export def "ec test" [year: int, quest: int] {
    let crate_name = $"aoc_($year)"
    let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
    
    print $"🐥 Test Everybody Codes ($year) - Quest ($quest) 🐥"
    cargo test -p $crate_name --bin $quest_mod
}

export def "ec watch" [
    year: int, 
    quest: int,
    --test # Run tests instead of the solution
] {
    watch --quiet . --glob=**/*.rs {||
        reset-terminal
        try { 
            if $test {
                ec test $year $quest
            } else {
                ec $year $quest 
            }
        } catch { |err| 
            print-error $"Compilation failed: ($err.msg)"
            print "🔄 Watching for changes..."
        }
    }
}

###*
# Sets up a new solution crate for a given year.
#
# This command will:
# 1. Create a new crate like `solutions/ec_YYYY`.
# 2. Add boilerplate code to the new crate.
#
# Usage:
# > ec new year 2025
###
export def "ec new year" [
    year: int, # The Everybody Codes year to set up (e.g., 2025)
] {
    let year_str = $"($year)"
    let crate_name = $"ec_($year_str)"
    let crate_path = $"solutions/($crate_name)"

    # --- 1. Validation ---
    if not ("solutions" | path exists) {
        print-error "This script must be run from the root of your EC workspace."
        return
    }

    if ($crate_path | path exists) {
        print-error $"Solution crate for year ($year_str) already exists at '($crate_path)'."
        return
    }

    print $"🚀 Setting up Everybody Codes ($year_str)..."

    # --- 2. Create the new solution crate ---
    # We use `--vcs none` to avoid creating a nested Git repository.
    cargo new --lib --vcs none $crate_path
    print-info $"Created new crate at '($crate_path)'"

    # --- 3. Configure the new crate's Cargo.toml ---
    let new_cargo_toml = $"
[package]
name = \"($crate_name)\"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = { workspace = true }
pathfinding = { workspace = true }
serde_json = { workspace = true }
rustc-hash = { workspace = true }
itertools = { workspace = true }
rayon = { workspace = true }
"

    $new_cargo_toml | save --force $"($crate_path)/Cargo.toml"
    print-info $"Configured '($crate_path)/Cargo.toml'"

    rm $"($crate_path)/src/lib.rs"
    mkdir $"($crate_path)/src/bin/inputs"
    print-info $"Created inputs directory"

    print $"\n🎉 Successfully set up year ($year)! You can now add daily solutions."
}

# Adds a new quest module to a given year crate and creates boilerplate for that quest.
# Usage: ec new quest 2025 1
export def "ec new quest" [
    year: int, # The Advent of Code year (e.g., 2025)
    quest: int   # The quest number (e.g., 1)
] {
    let year_str = $"($year)"
    let quest_str = $"($quest)"
    let crate_name = $"ec_($year_str)"
    let crate_path = $"solutions/($crate_name)"
    let src_path = $"($crate_path)/src/bin"
    let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
    let quest_file = $"($src_path)/($quest_mod).rs"

    ec input new $year $quest

    # --- 1. Validation ---
    if not ($crate_path | path exists) {
        print-error $"Crate for year ($year_str) does not exist at '($crate_path)'!"
        return
    }

    # --- 2. Create the quest module file ---
    if ($quest_file | path exists) {
        print-error $"Quest ($quest) already exists for year ($year_str) at '($quest_file)'!"
        return
    }
    let quest_boiler = (cat template.rs)
    $quest_boiler | str replace "[QUEST]" $quest_mod | save --force $quest_file
    print-info $"Created boilerplate for ($quest_mod) at '($quest_file)'"
}

export def "ec submit" [year: int, quest:int, part:int, answer:string] {

    http post --content-type application/json --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year)/quest/($quest)/part/($part)/answer" { "answer": $"($answer)"}
}

export def "ec input decode" [year: int, quest:int, part:int] {
    let year_str = $"($year)"
    let quest_str = $"($quest)"
    let part_str = $"($part)"
    let crate_name = $"ec_($year_str)"
    let crate_path = $"solutions/($crate_name)"
    let inputs_path = $"($crate_path)/src/bin/inputs"
    let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
    let input_path = $"($inputs_path)/($quest_mod).json"
    let keys_path = $"($inputs_path)/($quest_mod)-keys.json"

    let input_part_path = $"($inputs_path)/($quest_mod)-($part_str).txt"

    let key = (cat $keys_path | from json |  get $"key($part_str)")

    cat $input_path | from json | get $"($part_str)" | aes decrypt -k $"($key)" out> $input_part_path
}

export def "ec input new" [year: int, quest:int] {
    let year_str = $"($year)"
    let quest_str = $"($quest)"
    let crate_name = $"ec_($year_str)"
    let crate_path = $"solutions/($crate_name)"
    let inputs_path = $"($crate_path)/src/bin/inputs"
    mkdir $inputs_path
    let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
    let input_path = $"($inputs_path)/($quest_mod).json"
    let keys_path = $"($inputs_path)/($quest_mod)-keys.json"

    let user_info = (http get --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] https://everybody.codes/api/user/me)
    
    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody-codes.b-cdn.net/assets/($year_str)/($quest_str)/input/($user_info.seed).json" out> $input_path

    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year_str)/quest/($quest_str)" out> $keys_path

    ec input decode $year $quest 1
}
