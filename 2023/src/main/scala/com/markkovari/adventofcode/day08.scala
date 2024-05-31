package com.markkovari.adventofcode.day08

type State = String

type Transition = (State, Instr) => State

enum Instr:
  case GoLeft, GoRight

object Instr:
  def parse(inp: String): LazyList[Instr] =
    inp
      .map {
        case 'L' => Instr.GoLeft
        case 'R' => Instr.GoRight
      }
      .to(LazyList) #::: Instr.parse(inp)

def parseNetwork(inp: List[String]): Map[String, Vector[String]] =
  inp.map { case s"$a = ($b, $c)" =>
    (a -> Vector(b, c))
  }.toMap

def countStepsUntil(
    state: State,
    instrs: LazyList[Instr],
    trans: Transition,
    count: Int,
    pred: State => Boolean
): Int =
  if pred(state) then count
  else
    countStepsUntil(
      trans(state, instrs.head),
      instrs.tail,
      trans,
      count + 1,
      pred
    )

def transitions(network: Map[String, Vector[String]]): Transition =
  (n, d) =>
    d match
      case Instr.GoLeft  => network(n)(0)
      case Instr.GoRight => network(n)(1)

def part1(input: String): Int =
  val inpL = input.split("\n\n")
  val instructions = Instr.parse(inpL.head)
  val network = parseNetwork(inpL.tail.head.split("\n").toList)
  val trans = transitions(network)

  countStepsUntil("AAA", instructions, trans, 0, _ == "ZZZ")

def part2(input: String): Long =
  val inpL = input.split("\n\n")
  val instructions = Instr.parse(inpL.head)
  val network = parseNetwork(inpL.tail.head.split("\n").toList)
  val trans = transitions(network)

  val starts: Set[State] = network.keySet.filter(_.endsWith("A"))

  def leastCommonMultiple(a: Long, b: Long): Long =
    a * b / greatestCommonDivisor(a, b)

  def greatestCommonDivisor(a: Long, b: Long): Long =
    if b == 0 then a else greatestCommonDivisor(b, a % b)

  starts
    .map(state =>
      countStepsUntil(state, instructions, trans, 0, _.endsWith("Z")).toLong
    )
    .reduce(leastCommonMultiple)
