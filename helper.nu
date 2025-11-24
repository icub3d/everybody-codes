#!/usr/bin/env nu

source ~/.config/nushell/config.nu

def get-input [workspace: string, name: string, part="1": string] {
  let inputs_path = $"($workspace)/src/bin/inputs"
  let input_path = $"($inputs_path)/($name).json"
  let keys_path = $"($inputs_path)/($name)-keys.json"
  
  # Strip first character if it's not a number
  let part_str = ($part | into string)
  let first_char = ($part_str | str substring 0..1)
  let part_num = if ($first_char in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
    $part_str
  } else {
    $part_str | str substring 1..
  }
  
  mkdir $inputs_path

  let year = if ($workspace | str starts-with "story") {
    ($workspace | path basename | str substring 6..)
  } else {
    ($workspace | path basename | str substring 3..)
  }

  let quest = ($name | str replace "quest" "" | str trim | into int | into string)

  if ($env.EC_SESSION? | is-empty) {
    error make {msg: "EC_SESSION environment variable is not set"}
  }

  # Fetch the updated key info.
  http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year)/quest/($quest)" | save -f $keys_path

  # Get the input.
  let file_missing = not ($input_path | path exists)
  let file_empty = if ($input_path | path exists) { (ls $input_path | get 0.size) == 0B } else { false }
  if $file_missing or $file_empty {
    let user_info = (http get --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] https://everybody.codes/api/user/me)
    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody-codes.b-cdn.net/assets/($year)/($quest)/input/($user_info.seed).json" out> $input_path
  }

  # decode the input.
  let input_part_path = $"($inputs_path)/($name)-($part_num).txt"
  let key = (cat $keys_path | from json | get $"key($part_num)")
  let iv = ($key | str substring 0..15)
  cat $input_path | from json | get $"($part_num)" | aes decrypt --iv $"($iv)" -k $"($key)" | save --force $input_part_path
  
  # If this is part 1, copy to parts 2 and 3 for template to work
  if $part_num == "1" {
    cp $input_part_path $"($inputs_path)/($name)-2.txt"
    cp $input_part_path $"($inputs_path)/($name)-3.txt"
  }
}

def get-target [workspace: string, name: string, part="1": string] {
  let inputs_path = $"($workspace)/src/bin/inputs"
  let keys_path = $"($inputs_path)/($name)-keys.json"
  
  # Strip first character if it's not a number
  let part_str = ($part | into string)
  let first_char = ($part_str | str substring 0..1)
  let part_num = if ($first_char in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
    $part_str
  } else {
    $part_str | str substring 1..
  }
  
  if not ($keys_path | path exists) {
    error make {msg: $"Keys file not found: ($keys_path)"}
  }
  
  let keys = (cat $keys_path | from json)
  let answer_key = $"answer($part_num)"
  
  if ($answer_key in $keys) {
    print ($keys | get $answer_key)
  } else {
    error make {msg: $"Key '($answer_key)' not found in ($keys_path)"}
  }
}

def youtube [year: int, quest: int] {
  let quest_str = if $quest < 10 { $"0($quest)" } else { $"($quest)" };
  let folder = if $year < 2000 { "story" } else { "event" };

  let url = $"https://everybody.codes/($folder)/($year)/quests/($quest)" 
  let times = $"~/Videos/($year)-($quest_str).json" 
  let desc = if $year < 2000 {
    $"Solution for Everybody Codes Story ($year) Quest ($quest)" 
  } else {
    $"Solution for Everybody Codes ($year) Quest ($quest)" 
  };
  let file = if $year < 2000 {
    $"story_($year)/src/bin/quest($quest_str).rs" 
  } else {
    $"ec_($year)/src/bin/quest($quest_str).rs" 
  };

  youtube-description $url $times $desc $file
}

def main [command?: string, ...args] {
  if ($command | is-empty) {
    print "Usage: helper.nu <command> [args...]"
    print "Available commands:"
    print "  get-input <workspace> <name> [part]"
    print "  get-target <workspace> <name> [part]"
    print "  youtube <year> <quest>"
    return
  }

  let func_name = $command | str replace "-" "_"
  
  match $command {
    "get-input" => {
      if ($args | length) < 2 {
        print "Usage: get-input <workspace> <name> [part]"
        return
      }
      let workspace = ($args | get 0)
      let name = ($args | get 1)
      let part = (if ($args | length) >= 3 { $args | get 2 } else { "1" })
      get-input $workspace $name $part
    }
    "get-target" => {
      if ($args | length) < 2 {
        print "Usage: get-target <workspace> <name> [part]"
        return
      }
      let workspace = ($args | get 0)
      let name = ($args | get 1)
      let part = (if ($args | length) >= 3 { $args | get 2 } else { "1" })
      get-target $workspace $name $part
    }
    "youtube" => {
      if ($args | length) < 2 {
        print "Usage: youtube <year> <quest>"
        return
      }
      let year = ($args | get 0 | into int)
      let quest = ($args | get 1 | into int)
      youtube $year $quest
    }
    _ => {
      print $"Unknown command: ($command)"
      print "Available commands:"
      print "  get-input <workspace> <name> [part]"
      print "  get-target <workspace> <name> [part]"
      print "  youtube <year> <quest>"
    }
  }
}

