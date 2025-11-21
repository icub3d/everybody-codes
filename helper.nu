# Helper to get the formatted quest module name (e.g., quest01)
def "get-quest-mod" [quest: int] {
    if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" }
}

# Helper to get common paths and names for a given quest
def "get-quest-paths" [year: int, quest: int] {
    let year_str = $"($year)"
    let quest_mod = (get-quest-mod $quest)
    let crate_name = $"ec_($year_str)"
    let crate_path = $"solutions/($crate_name)"
    let src_path = $"($crate_path)/src/bin"
    let inputs_path = $"($crate_path)/src/bin/inputs"
    let quest_file = $"($src_path)/($quest_mod).rs"
    let input_path = $"($inputs_path)/($quest_mod).json"
    let keys_path = $"($inputs_path)/($quest_mod)-keys.json"

    {
        crate_name: $crate_name,
        crate_path: $crate_path,
        src_path: $src_path,
        inputs_path: $inputs_path,
        quest_mod: $quest_mod,
        quest_file: $quest_file,
        input_path: $input_path,
        keys_path: $keys_path,
    }
}

# Helper to run a command and print stderr if it fails
def "run-or-die" [
    args: list<string>, # The command and its arguments to run
    --allow-failure, # If true, don't exit on failure
] {
    let run = (do -c { ^$args } | complete)

    if ($run.exit_code != 0) {
        if not $allow_failure {
            exit 1
        }
    }
    $run
}

# Helper to print detailed unit test failures
def "print-test-failures" [
    test_run: record, # The completed process record from the test run
    test_failures: table, # A table of tests that failed
] {
    if ($test_failures | is-empty) {
        return
    }

    print-error "\n\nUnit Test Failures:"
    let stdout_lines = ($test_run.stdout | lines)
    for failure in $test_failures {
        print-error $"\n- ($failure.name):"
        
        mut start_index = -1
        for line in ($stdout_lines | enumerate) {
            if ($line.item | str contains $"---- ($failure.name) stdout ----") {
                $start_index = $line.index
                break
            }
        }
        
        if $start_index > -1 {
            let failure_lines = ($stdout_lines | skip ($start_index + 2) | take while { not ($in | is-empty) })
            print ($failure_lines | str join "\n")
        }
    }
}

# run all quests for a given year
export def "ec all" [year: int] {
    let results = (
        1..25
        | each {|quest|
            let paths = (get-quest-paths $year $quest)
            let quest_file = $paths.quest_file

            if not ($quest_file | path exists) {
                []
            } else {
                let run = (run-or-die ["cargo" "run" "--release" "-q" "-p" $paths.crate_name "--bin" $paths.quest_mod] --allow-failure)

                if $run.exit_code != 0 {
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
        print $"🐥 EC ($year) Summary 🐥"
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
    let paths = (get-quest-paths $year $quest)
    
    let answers = if not ($paths.keys_path | path exists) {
        []
    } else {
        (open $paths.keys_path | items {|k, v| {name: $k, value: $v}} )
        | where { |i| ($i.name | str starts-with "answer") and not ($i.value | is-empty) }
        | each { |i| { part: ($i.name | str substring 6.. | into int), answer: $i.value } }
    }

    # --- Run tests for all parts ---
    let test_run = (do -i { cargo test --no-fail-fast -p $paths.crate_name "--bin" $paths.quest_mod } | complete)
    let test_lines = ($test_run.stdout | default "" | lines | where {|it| ($it | str trim | str starts-with "test ") })

    # --- Run quest for all parts ---
    let run_output = (run-or-die ["cargo" "run" "--release" "-q" "-p" $paths.crate_name "--bin" $paths.quest_mod] --allow-failure)
    let part_outputs = ($run_output.stdout | default "" | lines | where {|l| not ($l | is-empty)} | parse "{part} {time} {solution}")

    mut results = []
    for part_num in 1..3 {
        # --- Determine test_status ---
        let test_name_prefix = $"tests::test_p($part_num)"
        let part_test_lines = ($test_lines | where {|it| $it | str contains $test_name_prefix })
        let failed_tests = ($part_test_lines | where {|it| not ($it | str contains "ok") })
        let test_status = if ($part_test_lines | is-empty) {
            "❓" # No test for this part
        } else if ($failed_tests | is-empty) {
            "✅" # All tests for this part passed
        } else {
            "❌" # Some tests for this part failed
        }

        # --- Get solution, time, and correct_answer ---
        let part_output = ($part_outputs | where part == $"p($part_num)")
        let solution = if not ($part_output | is-empty) { ($part_output | get solution | first) } else { "❓" }
        let time = if not ($part_output | is-empty) { ($part_output | get time | first) } else { "❓" }

        let matching_answer = ($answers | where part == $part_num)
        let correct_answer = if not ($matching_answer | is-empty) {
            ($matching_answer | get answer | first)
        } else {
            null
        }

        # --- Determine solution status ---
        let status = if $correct_answer == null {
            "🔄"
        } else if $solution == "❓" {
            "❓"
        } else if $solution == $correct_answer {
            "✅"
        } else {
            "❌"
        }

        $results = ($results | append {
            part: $"p($part_num)",
            time: $time,
            solution: $solution,
            correct_answer: ($correct_answer | default ""),
            status: $status,
            test_status: $test_status
        })
    }

    if not ($results | is-empty) {
        $results | select part status test_status time solution correct_answer | rename "🧩" "🚦" "🧪" "⏰" "💡" "🎯" | table -i false | print
        print ""
    }
}

export def "ec test" [year: int, quest: int] {
    let paths = (get-quest-paths $year $quest)
    
    print $"🐥 Test EC ($year) - Quest ($quest) 🐥\n"
    let test_run = (do -i { cargo test --no-fail-fast -p $paths.crate_name "--bin" $paths.quest_mod } | complete)
    print $test_run.stdout
}

export def "ec watch" [
    year: int, 
    quest: int,
    --test,
] {
  reset-terminal 
  run-quest $year $quest --test=$test
  try {
    watch --quiet . --glob=**/*.rs {||
      reset-terminal 
      run-quest $year $quest --test=$test
    }
  } catch {}
}

def run-quest [year: int, quest: int, --test] {
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

export def "ec debug" [
    year: int, 
    quest: int
] {
  reset-terminal 
  run-debug $year $quest
  try {
    watch --quiet . --glob=**/*.rs {||
      reset-terminal 
      run-debug $year $quest
    }
  } catch {}
}

def "run-debug" [year: int, quest: int] {
    let paths = (get-quest-paths $year $quest)
    
    print $"🧪 Tests 🧪"
    try {
      cargo test --release -p $paths.crate_name -q --no-fail-fast "--bin" $paths.quest_mod -- --nocapture
    } catch {}

    print $"🚀 Quest 🚀"
    try {
      cargo run --release -p $paths.crate_name -q "--bin" $paths.quest_mod 
    } catch {}
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
edition = "2021"

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
    year: int, # The Everybody Codes year (e.g., 2025)
    quest: int   # The quest number (e.g., 1)
] {
    let paths = (get-quest-paths $year $quest)

    ec input new $year $quest

    # --- 1. Validation ---
    if not ($paths.crate_path | path exists) {
        print-error $"Crate for year ($year) does not exist at '($paths.crate_path)'!"
        return
    }

    # --- 2. Create the quest module file ---
    if ($paths.quest_file | path exists) {
        print-error $"Quest ($quest) already exists for year ($year) at '($paths.quest_file)'!"
        return
    }
    let quest_boiler = (cat template.rs)
    $quest_boiler | str replace -a "[QUEST]" $paths.quest_mod | save --force $paths.quest_file
    print-info $"Created boilerplate for ($paths.quest_mod) at '($paths.quest_file)'"
}

export def "ec submit" [year: int, quest:int, part:int, answer:string] {

    http post --content-type application/json --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year)/quest/($quest)/part/($part)/answer" { "answer": $"($answer)"}
}

export def "ec fetch key" [year: int, quest:int] {
    let paths = (get-quest-paths $year $quest)
    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year)/quest/($quest)" | save -f $paths.keys_path
}

export def "ec input decode" [year: int, quest:int, part:int] {
    let paths = (get-quest-paths $year $quest)
    let part_str = $"($part)"

    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year)/quest/($quest)" out> $paths.keys_path

    let input_part_path = $"($paths.inputs_path)/($paths.quest_mod)-($part_str).txt"
    let key = (cat $paths.keys_path | from json | get $"key($part_str)")
    let iv = ($key | str substring 0..15)

    cat $paths.input_path | from json | get $"($part_str)" | aes decrypt --iv $"($iv)" -k $"($key)" | save --force $input_part_path
}

export def "ec input new" [year: int, quest:int] {
    let paths = (get-quest-paths $year $quest)
    mkdir $paths.inputs_path

    echo "" | save --force $"($paths.inputs_path)/($paths.quest_mod)-2.txt"
    echo "" | save --force $"($paths.inputs_path)/($paths.quest_mod)-3.txt"

    let user_info = (http get --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] https://everybody.codes/api/user/me)
    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody-codes.b-cdn.net/assets/($year)/($quest)/input/($user_info.seed).json" out> $paths.input_path

    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year)/quest/($quest)" out> $paths.keys_path

    ec input decode $year $quest 1
    cp $"($paths.inputs_path)/($paths.quest_mod)-1.txt" $"($paths.inputs_path)/($paths.quest_mod)-2.txt"
    cp $"($paths.inputs_path)/($paths.quest_mod)-1.txt" $"($paths.inputs_path)/($paths.quest_mod)-3.txt"
}


# Generate a YouTube description with timestamps from a stage progress JSON file.
# Usage: youtube-desc path/to/2015-13.json
export def "ec yt" [
    file: string # The path to the JSON file (e.g., '2015-13.json')
] {
    let file = ($file | path expand)

    # Validate file exists
    if not ($file | path exists) {
        print-error $"JSON file not found: '($file)'"
        return
    }

    # Derive year and day from the filename (e.g., '2015-13.json')
    let base = ($file | path basename)
    let base_no_ext = ($base | str replace ".json" "")
    let parts = ($base_no_ext | split row "-")
    if ($parts | length) < 2 {
        print-error "Filename must be in the format 'YEAR-DAY.json' (e.g., '2015-13.json')."
        return
    }
    let year = ($parts | get 0 | into int)
    let quest = ($parts | get 1 | into int)


    let paths = (get-quest-paths $year $quest)
    let gist_desc = ($"Everybody Codes ($year) Quest ($quest) Solution")
    let problem_url = $"https://everybody.codes/event/($year)/quests/($quest)"

    youtube-description $problem_url $file $gist_desc $paths.quest_file
}
