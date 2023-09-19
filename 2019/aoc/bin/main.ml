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

module Day03 = struct
          
  type point = {
    x : float ;
    y : float
  }

  type line = {
    p1 : point ;
    p2 : point
  }

  type line_eq = {
    a : float ;
    b : float ;
    c : float
  }          

  let as_line_eq (l1 : line)  =
  let a1 = l1.p2.y -. l1.p1.y and
      b1 = l1.p1.x -. l1.p2.x in
  let c1 = (a1 *. l1.p1.x) +. (b1 *. l1.p1.y) in
  {a = a1 ; b = b1 ; c = c1}

  let on_segment (l : line) (p : point) =
    if p.x >= (min l.p1.x l.p2.x)
      && p.x <= (max l.p1.x l.p2.x)
      && p.y >= (min l.p1.y l.p2.y)
      && p.y <= (max l.p1.y l.p2.y)
    then
      true
    else
      false

  let intersection_point (ll1 : line) (ll2 : line) =
    let l1 = as_line_eq ll1 and
        l2 = as_line_eq ll2 in
    let determinant = (l1.a *. l2.b) -. (l2.a *. l1.b) in
    if determinant = 0.0 then
      {x = 0.0 ; y = 0.0}
    else
      let x = (l2.b *. l1.c -. l1.b *. l2.c) /. determinant and
          y = (l1.a *. l2.c -. l2.a *. l1.c) /. determinant in
      {x = x ; y = y}
        
  let is_intersect (l1 : line) (l2 : line) =
    let intr_pt = (intersection_point l1 l2) in
    if intr_pt.x = 0.0 && intr_pt.y = 0. then
      false 
    else
      on_segment l1 intr_pt && on_segment l2 intr_pt
  
  
  let length (ln : line) =
    Float.sqrt (((ln.p2.x -. ln.p1.x) ** 2.0) +. ((ln.p2.y -. ln.p1.y) ** 2.0))
    
  let manhattan_distance {x ; y} =
    (Float.abs x) +. (Float.abs y)
  
  let steps lns_lst pt =
    List.fold_left
      (fun (steps , reached) ln ->
        if reached then
          (steps, reached)
        else
          if on_segment ln pt then
            let new_ln = {p1 = ln.p1 ; p2 = pt} in
            (steps +. (length new_ln) , true)
          else
            (steps +. (length ln) , reached)
      )
      (0.0 , false)
      lns_lst
      
  let path_to_line (start_pt : point) (path : string) =
    let path_lst = List.init (String.length path) (String.get path) and
        len_str = String.sub path 1 ((String.length path) - 1) in
    let path_len = float_of_string len_str in
    match path_lst with
    | 'R' :: _ -> { p1 = start_pt ; p2 = { x = start_pt.x +. path_len ; y = start_pt.y }}
    | 'L' :: _ -> { p1 = start_pt ; p2 = { x = start_pt.x -. path_len ; y = start_pt.y }}
    | 'U' :: _ -> { p1 = start_pt ; p2 = { x = start_pt.x ; y = start_pt.y +. path_len }}
    | 'D' :: _ -> { p1 = start_pt ; p2 = { x = start_pt.x ; y = start_pt.y -. path_len }}
    | _ -> {p1 = start_pt ; p2 = start_pt}


  let paths_to_lines paths_lst =
    List.tl (
      List.rev (
        List.fold_left
          (fun acc (path : string) ->
            let last_ln = List.hd acc in
            let next_ln = path_to_line last_ln.p2 path in
            next_ln :: acc)
          [{p1 = {x = 0. ; y = 0.} ; p2 = {x = 0. ; y = 0.}}]
          paths_lst)
      )
  let second w1 w2 =
    let lw1 = paths_to_lines w1 and
        lw2 = paths_to_lines w2 in
    List.fold_left
      (fun acc (ln : line) ->
        let acc_inner =
              (List.fold_left
                  (fun acc1 (ln2 : line) ->
                    if is_intersect ln ln2  then
                      let intr_pt = intersection_point ln ln2 in
                      let (steps1 , reached1) = steps lw1 intr_pt and
                          (steps2 , reached2) = steps lw2 intr_pt in
                      if reached1 && reached2 then
                        steps1 +. steps2
                      else
                        acc1
                    else acc1)
                  acc
                  lw2) in
        if acc_inner < acc then
          acc_inner
        else acc)
      max_float
      lw1

  let first w1 w2 =
    let lw1 = paths_to_lines w1 and
        lw2 = paths_to_lines w2 in
    List.fold_left
      (fun acc (ln : line) ->
        let acc_inner =
              (List.fold_left
                  (fun acc1 (ln2 : line) ->
                    if is_intersect ln ln2  then
                      let intr_pt = intersection_point ln ln2 in
                      let intr_mag = manhattan_distance intr_pt in
                      if intr_mag < acc1 then
                        intr_mag 
                      else acc1
                    else acc1)
                  acc
                  lw2) in
        if acc_inner < acc then
          acc_inner
        else acc)
      max_float
      lw1
  let text = In_channel.with_open_text "input/3/data" In_channel.input_all

  let rows = String.split_on_char '\n' text
  let first_row = String.split_on_char ','  (List.hd rows)
  
  let second_row = String.split_on_char ','  (List.nth rows 1)
  let solve = (int_of_float (first first_row second_row),  int_of_float (second first_row second_row))
end

let (s1,s2) = Day01.solve
let () = Printf.printf "Day 1; first: %d second: %d \n" s1 s2

let (s1,s2) = Day02.solve
let () = Printf.printf "Day 2; first: %d second: %d \n" s1 s2

let (s1,s2) = Day03.solve
let () = Printf.printf "Day 3; first: %d second: %d \n" s1 s2