#!/usr/bin/env nu

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

  # Make empty inputs so the template.rs works until new ones come in. Don't overwrite though.
  try {
      echo "" | save $"($inputs_path)/($name)-1.txt"
      echo "" | save $"($inputs_path)/($name)-2.txt"
      echo "" | save $"($inputs_path)/($name)-3.txt"
  } catch {}

  let year = ($workspace | path basename | str substring 3..)
  let quest = ($name | str replace "quest" "" | str trim)

  if ($env.EC_SESSION? | is-empty) {
    error make {msg: "EC_SESSION environment variable is not set"}
  }

  # Fetch the updated key info.
  http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody.codes/api/event/($year)/quest/($quest)" | save -f $keys_path

  # Get the input.
  if not ($input_path | path exists) {
    let user_info = (http get --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] https://everybody.codes/api/user/me)
    http get --raw --headers [Cookie $"everybody-codes=($env.EC_SESSION)"] $"https://everybody-codes.b-cdn.net/assets/($year)/($quest)/input/($user_info.seed).json" out> $input_path
  }

  # decode the input.
  let input_part_path = $"($inputs_path)/($name)-($part_num).txt"
  let key = (cat $keys_path | from json | get $"key($part_num)")
  let iv = ($key | str substring 0..15)
  cat $input_path | from json | get $"($part_num)" | aes decrypt --iv $"($iv)" -k $"($key)" | save --force $input_part_path
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

def main [command?: string, ...args] {
  if ($command | is-empty) {
    print "Usage: helper.nu <command> [args...]"
    print "Available commands:"
    print "  get-input <workspace> <name> [part]"
    print "  get-target <workspace> <name> [part]"
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
    _ => {
      print $"Unknown command: ($command)"
      print "Available commands:"
      print "  get-input <workspace> <name> [part]"
      print "  get-target <workspace> <name> [part]"
    }
  }
}

