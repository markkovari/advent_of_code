package com.markkovari.adventofcode.day03

import scala.util.matching.Regex.Match

case class Detail(content: String, x: Int, y: Int)

case class Coordinate(x: Int, y: Int):
  def within(start: Coordinate, end: Coordinate) =
    if y < start.y || y > end.y then false
    else if x < start.x || x > end.x then false
    else true

case class PartNumber(value: Int, start: Coordinate, end: Coordinate)

case class Symbol(sym: String, pos: Coordinate):
  def neighborOf(number: PartNumber) = pos.within(
    Coordinate(number.start.x - 1, number.start.y - 1),
    Coordinate(number.end.x + 1, number.end.y + 1)
  )

object IsInt:
  def unapply(in: Match): Option[Int] = in.matched.toIntOption

def findPartsAndSymbols(
    source: String
): Array[PartNumber | com.markkovari.adventofcode.day03.Symbol] =
  val extractor = """(\d+)|[^.\d]""".r
  source
    .split("\n")
    .zipWithIndex
    .flatMap: (line, i) =>
      extractor
        .findAllMatchIn(line)
        .map:
          case m @ IsInt(nb) =>
            PartNumber(nb, Coordinate(m.start, i), Coordinate(m.end - 1, i))
          case s => Symbol(s.matched, Coordinate(s.start, i))

def part1(input: String) =
  val all = findPartsAndSymbols(input)
  val symbols = all.collect { case s: Symbol => s }
  all
    .collect:
      case n: PartNumber if symbols.exists(_.neighborOf(n)) =>
        n.value
    .sum

case class Gear(part: PartNumber, symbol: Symbol)

def part2(input: String) =
  val all = findPartsAndSymbols(input)
  val symbols = all.collect { case s: Symbol => s }
  all
    .flatMap:
      case n: PartNumber =>
        symbols
          .find(_.neighborOf(n))
          .filter(_.sym == "*")
          .map(Gear(n, _))
      case _ => None
    .groupMap(_.symbol)(_.part.value)
    .filter(_._2.length == 2)
    .foldLeft(0) { _ + _._2.product }
