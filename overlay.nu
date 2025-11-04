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
    let crate_name = $"ec_($year)"
    let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
    print $"🐥 EC ($year) - Quest ($quest) 🐥"
    cargo run --release -q -p $crate_name --bin $quest_mod
}

export def "ec test" [year: int, quest: int] {
    let crate_name = $"ec_($year)"
    let quest_mod = (if $quest < 10 { $"quest0($quest)" } else { $"quest($quest)" })
    
    print $"🐥 Test EC ($year) - Quest ($quest) 🐥"
    cargo test -p $crate_name --bin $quest_mod
}

export def "ec watch" [
    year: int, 
    quest: int,
    --test # Run tests instead of the solution
] {
  reset-terminal 
  run-quest $year $quest --test=$test
    watch --quiet . --glob=**/*.rs {||
      reset-terminal 
      run-quest $year $quest --test=$test
    }
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
    year: int, # The Everybody Codes year (e.g., 2025)
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
    $quest_boiler | str replace -a "[QUEST]" $quest_mod | save --force $quest_file
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

    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year_str)/quest/($quest_str)" out> $keys_path

    let input_part_path = $"($inputs_path)/($quest_mod)-($part_str).txt"
    let key = (cat $keys_path | from json | get $"key($part_str)")
    let iv = ($key | str substring 0..15)

    cat $input_path | from json | get $"($part_str)" | aes decrypt --iv $"($iv)" -k $"($key)" | save --force $input_part_path
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

    echo "" | save --force $"($inputs_path)/($quest_mod)-2.txt"
    echo "" | save --force $"($inputs_path)/($quest_mod)-3.txt"

    let user_info = (http get --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] https://everybody.codes/api/user/me)
    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody-codes.b-cdn.net/assets/($year_str)/($quest_str)/input/($user_info.seed).json" out> $input_path

    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year_str)/quest/($quest_str)" out> $keys_path

    ec input decode $year $quest 1
    cp $"($inputs_path)/($quest_mod)-1.txt" $"($inputs_path)/($quest_mod)-2.txt"
    cp $"($inputs_path)/($quest_mod)-1.txt" $"($inputs_path)/($quest_mod)-3.txt"
}

# Uploads a quest's solution file to a GitHub Gist using the gh CLI.
# Usage: upload-gist 2015 1 [--public]
export def "upload-gist" [
    year: int, # The Everybody Code year (e.g., 2015)
    quest: int,  # The day number (e.g., 1)
] {
    let year_str = $"($year)"
    let quest_str = (if $quest < 10 { ("quest0" ++ ($quest | into string)) } else { ("quest" ++ ($quest | into string)) })
    let file_path = ("solutions/ec_" ++ $year_str ++ "/src/bin/" ++ $quest_str ++ ".rs")

    if not ($file_path | path exists) {
        print-error ("Solution file not found: " ++ $file_path)
        return
    }

    let gist_desc = ($"Everybody Codes ($year_str) Quest ($quest) Solution")
    let public_flag = "--public"

    print ("🚀 Uploading " ++ $file_path ++ " to GitHub Gist...")
    let cmd = ["gh" "gist" "create" $file_path "--desc" $gist_desc $public_flag]
    let result = do -i { ^$cmd }
    if ($result | describe) == 'string' {
        print-info "Gist uploaded successfully!"
        $result
    } else if ($result.exit_code? | default 1) == 0 {
        print-info "Gist uploaded successfully!"
        $result.stdout? | default ""
    } else {
        print-error "Failed to upload Gist."
        $result.stderr? | default $result
    }
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

    # --- Find or Create Gist ---
    let filter_str = $"($year) Quest ($quest)"
    let gist_id = (gh gist list --limit 1 --filter $filter_str | split column "\t" | get column1 | first)

    let solution_url = if not ($gist_id | is-empty) {
        $"https://gist.github.com/icub3d/($gist_id)"
    } else {
        # No gist found, so create one and capture the output URL
        upload-gist $year $quest
    }


    # Build problem URL https://everybody.codes/event/2024/quests/1
    let problem_url = $"https://everybody.codes/event/($year)/quests/($quest)"

    # Print header for description
    print "[TODO]"
    print ""
    print $"Problem: ($problem_url)"
    print $"Solution: ($solution_url)"
    print ""

    ec stages $file

}


export def "ec stages" [file: string] {
    let file = ($file | path expand)

    # Validate file exists
    if not ($file | path exists) {
        print-error $"JSON file not found: '($file)'"
        return
    }

    # Parse JSON
    let data = (open --raw $file | from json)

    # Get stage times (fall back to empty if missing)
    let stages = ($data | get stageTimes | default [])

    # Sort stages by startMs to ensure order
    let stages = ($stages | sort-by startMs)

    if ($stages | is-empty) {
        print-info "No 'stageTimes' found in JSON."
        return
    }

    # Print timestamp lines, converting startMs (milliseconds) to either 'M:SS' or 'H:MM:SS'.
    for $st in $stages {
        let start_ms = ($st | get startMs | default 0)
        let total_secs = ($start_ms / 1000 | into int)
        let hours = (($total_secs / 3600) | into int)
        let mins = ((($total_secs mod 3600) / 60) | into int)
        let secs = (($total_secs mod 60) | into int)

        let time_str = (
            if $hours > 0 {
                # H:MM:SS — zero-pad minutes and seconds to 2 digits
                if $mins < 10 {
                    if $secs < 10 { $"($hours):0($mins):0($secs)" } else { $"($hours):0($mins):($secs)" }
                } else {
                    if $secs < 10 { $"($hours):($mins):0($secs)" } else { $"($hours):($mins):($secs)" }
                }
            } else {
                # M:SS — seconds zero-padded
                if $secs < 10 { $"($mins):0($secs)" } else { $"($mins):($secs)" }
            }
        )

        let name = ($st | get stageName | default "Unnamed Stage")
        print $"($time_str) ($name)"
    }
}
