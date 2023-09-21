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

module Day04 = struct
  let start = 236491
  let stop = 713787
  
  let rec is_sorted : int list -> bool = function
    | [] | [_] -> true
    | x :: y :: tl -> x <= y && is_sorted (y :: tl)

  let group p l = 
    let rec grouping acc = function
      | [] -> acc
      | hd::tl ->
        let l1,l2 = List.partition (p hd) tl in
        grouping ((hd::l1)::acc) l2  
    in 
    grouping [] l
  let get_groups password = group (fun x y -> x = y) password

  let has_group_of digits amount =  List.exists (fun x -> List.length x = amount) (get_groups digits)
 
  let has_at_least_amount digits amount = 
    let groups = get_groups digits in
    List.exists (fun x -> List.length x >= amount) groups

  let get_digits digits = List.map (fun x -> int_of_string (String.make 1 x)) (string_of_int digits |> String.to_seq |> List.of_seq) 
  let password_correct number = 
    let digits = get_digits number in
    is_sorted digits && has_at_least_amount digits 2
  
  let passowrd_correct_restricted number = 
    let digits = get_digits number in
    is_sorted digits  && has_group_of digits 2
      
  let list_of_numbers = List.init (stop -1 - start+1) (fun i -> i + start )
  let correct_password = List.filter password_correct list_of_numbers
  let correct_password_restricted = List.filter passowrd_correct_restricted correct_password

  let solve = (List.length correct_password, List.length correct_password_restricted)
end

module Day05 = struct 
  let instructions = Intcode.read_instructions "input/5/data"
  let solve ~part instructions =
    let input = if part = 1 then 1 else 5 in
    let ram_size = List.length instructions in
    instructions
    |> Intcode.initialize_computer ~ram_size
    |> Intcode.receive input
    |> Intcode.run_until_halt
    |> Intcode.get_last_output
  
  let solve =(instructions |> solve ~part:1,instructions |> solve ~part:2)
end

module RelationMap = CCMap.Make(String)

module Day06 = struct
  let parse filename =
    let add = RelationMap.add in
    let get = RelationMap.get_or ~default:[]
    in
    CCIO.(with_in filename read_lines_l)
    |> List.map (String.split_on_char ')')
    |> List.fold_left
      (fun (p2c, k2p) -> function
         | [ p; k ] ->
           let c = p2c |> get p in
           ( p2c |> add p (k :: c),
             k2p |> add k p )
         | _ -> failwith "invalid input")
      (RelationMap.empty, RelationMap.empty)

  let part_1 p2c =
    let rec traverse n key =
      let children = p2c |> RelationMap.get_or ~default:[] key in
      match children with
      | [] -> n
      | _ ->
        let children_distances = List.map (traverse (n+1)) children in
        n + List.fold_left (+) 0 children_distances
    in
    traverse 0 "COM" 

  let part_2 k2p =
    let find_all_ancestors =
      let rec traverse relations acc = function
        | "COM" -> acc
        | kid ->
          let parent = relations |> RelationMap.find kid in
          traverse relations (parent::acc) parent
      in
      traverse k2p []
    in
    let rec calc_orbital_transfers you san =
      match you, san with
      | x::xs, y::ys when x = y -> calc_orbital_transfers xs ys
      | _, _ -> List.length you + List.length san
    in
    let you = find_all_ancestors "YOU" in
    let san = find_all_ancestors "SAN" in
    calc_orbital_transfers you san

  let p2c, k2p = parse "input/6/data"
  let solve = (p2c |> part_1, k2p |> part_2)
end

module Day07 = struct
  let instructions = Intcode.read_instructions "input/7/data"
  let all_permutations =
    let rec aux result other = function
      | [] -> [result]
      | hd :: tl ->
        let r = aux (hd :: result) [] (other @ tl) in
        if tl <> [] then
          r @ aux result (hd :: other) tl
        else r
    in
    aux [] []

  let create_computers =
    List.map
      (fun phase ->
        instructions
        |> Intcode.initialize_computer
        |> Intcode.receive phase)
  
  let some_halted =
    List.exists (fun comp -> Intcode.get_state comp = Intcode.Halted)
  
  let rec get_output (score, computers) =
    if some_halted computers then score
    else
      computers
      |> CCList.fold_map
        (fun last_output comp ->
              comp
              |> Intcode.receive last_output
              |> Intcode.run_until_halt
              |> fun comp -> ((Intcode.get_next_output comp), comp) )
        score
      |> get_output

  open CCFun
  let solve =
    all_permutations
    %> List.fold_left
      (fun acc perm ->
          let computers = create_computers perm in
          (0, computers) |> get_output |> max acc)
      0
  let solve = (CCList.(0 -- 4) |> solve,CCList.(5 -- 9) |> solve)
end

module Day08 = struct
  let h, w = 6, 25

  type digits = { zeros : int; ones : int; twos: int }

  let layers =
    CCIO.(with_in "input/8/data" read_all)
    |> CCString.to_list
    |> CCList.sublists_of_len (h*w)

  let count_digits x =
    let zeros = x |> CCList.count ((=) '0') in
    let ones  = x |> CCList.count ((=) '1') in
    let twos  = x |> CCList.count ((=) '2') in
    { zeros; ones; twos }

  let part_1 =
    let open CCFun in
    List.map count_digits
    %> List.fold_left
      (fun (zc, res) { zeros; ones; twos } ->
        if zeros < zc then zeros, (ones * twos) else zc, res)
      (Int.max_int, 0)
    %> snd

  let rec pixel_color layers pixel =
    match layers with
    | [] -> failwith "pixel is transparent"
    | layer :: below ->
      (match List.nth layer pixel with
      | '0' -> ' '
      | '1' -> '#'
      | '2' -> pixel_color below pixel
      | _ -> failwith "invalid input")

  let part_2 layers =
    CCList.(0 --^ (h*w))
    |> List.map (pixel_color layers)
    |> CCList.sublists_of_len w
    |> List.map CCString.of_list
    |> List.iter (Printf.printf "%s\n")
  
  let solve = (layers |> part_1, layers |> part_2)
end

let (s1,s2) = Day01.solve
let () = Printf.printf "Day 1; first: %d second: %d \n" s1 s2

let (s1,s2) = Day02.solve
let () = Printf.printf "Day 2; first: %d second: %d \n" s1 s2

let (s1,s2) = Day03.solve
let () = Printf.printf "Day 3; first: %d second: %d \n" s1 s2

let (s1,s2) = Day04.solve
let () = Printf.printf "Day 4; first: %d second: %d \n" s1 s2

let (s1,s2) = Day05.solve
let () = Printf.printf "Day 5; first: %d second: %d \n" s1 s2

let (s1,s2) = Day06.solve
let () = Printf.printf "Day 6; first: %d second: %d \n" s1 s2

let (s1,s2) = Day07.solve
let () = Printf.printf "Day 7; first: %d second: %d \n" s1 s2

let (s1,_) = Day08.solve;;
Day08.part_2(Day08.layers);;
let () = Printf.printf "Day 8; first: %d second:\n" s1

