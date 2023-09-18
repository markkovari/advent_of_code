module Day01 = struct
  let text = In_channel.with_open_text "input/1/data" In_channel.input_all
  let splitted = 
    String.split_on_char '\n' text
  let numbers = List.map int_of_string splitted
  let required_fuel mass = 
    let fuel = mass / 3 - 2 in
    if fuel < 0 then 0 else fuel

  let rec required_fuel_with_tank mass = 
    let fuel = mass / 3 - 2 in
    if fuel < 0 then 0 else fuel + required_fuel_with_tank fuel

  let necessary_fuels = List.map required_fuel numbers
  let necessary_fuels_with_tanks = List.map required_fuel_with_tank numbers

  let first_result = 
    List.fold_left 
    (fun acc x -> acc + x)
    0 necessary_fuels
  let second_result = 
    List.fold_left 
    (fun acc x -> acc + x)
    0 necessary_fuels_with_tanks

  let solve = (first_result, second_result)
end

module Day02 = struct
  let text = In_channel.with_open_text "input/2/data" In_channel.input_all

  let splitted = String.split_on_char ',' text
  let numbers = List.map int_of_string splitted


  let run_program program =
    let get_indirect index = program.(program.(index)) in
    let set_indirect index value = program.(program.(index)) <- value in
    let rec solve index =
      let apply_simple_op operator =
        let value = operator (get_indirect (index + 1)) (get_indirect (index + 2)) in
        set_indirect (index + 3) value;
        solve (index + 4)
      in
      match program.(index) with
      | 1 -> apply_simple_op ( + )
      | 2 -> apply_simple_op ( * )
      | 99 -> ()
      | _opcode -> failwith "Unknown opcode" in
    solve 0
  ;;

  let run_program_with_inputs program first second =
    let program = Array.of_list program in
    program.(1) <- first;
    program.(2) <- second;
    run_program program;
    program.(0)
  ;;
  let solve_part_1 program = run_program_with_inputs program 12 2
  let target_output = 19690720

  let solve_part_2 program =
    let rec solve noun verb =
      let output = run_program_with_inputs program noun verb in
      if output = target_output then (noun, verb)
      else if output > target_output then solve (noun - 1) verb
      else solve noun (verb + 1) in
    solve 99 0

  let (noun, verb) = solve_part_2 numbers
  let solve = (solve_part_1 numbers, 100 * noun + verb)
end


let (s1,s2) = Day01.solve
let () = Printf.printf "Day 1; first: %d second: %d \n" s1 s2

let (s1,s2) = Day02.solve
let () = Printf.printf "Day 2; first: %d second: %d \n" s1 s2