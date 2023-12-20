package com.markkovari.adventofcode

import org.scalatest.funsuite.AnyFunSuite
import scala.io.Source
import scala.collection.immutable.HashMap
import day05._

class Day05Tests extends AnyFunSuite {

  test("Part1 example") {
    val input = io.Source.fromFile("./src/test/resources/5/example_1").mkString
    val result = part1(input)
    assert(result == 35)
  }

  test("Part1 is calculated") {
    val input = io.Source.fromFile("./src/test/resources/5/values").mkString
    val result = part1(input)
    assert(result == 1181555926)
  }

  test("Part2 example") {
    val input = io.Source.fromFile("./src/test/resources/5/example_1").mkString
    val result = part2(input)
    assert(result == 46)
  }

  test("Part2 is calculated") {
    val input = io.Source.fromFile("./src/test/resources/5/values").mkString
    val result = part2(input)
    assert(result == 37806486)
  }
}
