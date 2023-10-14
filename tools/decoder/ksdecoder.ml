let decode_char kchar = kchar lxor 0b00110110

let decode_ks ksin =
  let ic = open_in ksin in
  let oc = open_out (ksin ^ ".decoded.ks") in
  try
    (* read bytes per bytes *)
    let rec read_bytes () =
      let kchar = input_byte ic in
      let dchar = decode_char kchar in
      output_byte oc dchar ; read_bytes ()
    in
    read_bytes ()
  with End_of_file -> close_in ic

let is_game_ks filename =
  let len = String.length filename in
  let ext =
    if String.length filename > 2 then String.sub filename (len - 3) 3 else ""
  in
  let longer_ext =
    if String.length filename > 10 then String.sub filename (len - 11) 11
    else ""
  in
  ext = ".ks" && longer_ext <> ".decoded.ks"

let rec decode_ks_recursive kdir =
  let files = Sys.readdir kdir in
  Array.iter
    (fun file ->
      let path = kdir ^ "/" ^ file in
      if Sys.is_directory path then decode_ks_recursive path
      else if is_game_ks path then decode_ks path
      else () )
    files

let () =
  for i = 1 to Array.length Sys.argv - 1 do
    decode_ks_recursive Sys.argv.(i)
  done
