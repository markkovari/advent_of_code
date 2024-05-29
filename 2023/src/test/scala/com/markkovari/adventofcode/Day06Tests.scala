package com.markkovari.adventofcode

import com.markkovari.adventofcode.day06.*
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source

class Day06Tests extends AnyFunSuite {
  test("Part1 example") {
    val source = io.Source.fromFile("./src/test/resources/6/example_1")
    assert(part1(source.mkString) === 288)
    source.close()
  }

  test("Part1 solution") {
    val source = io.Source.fromFile("./src/test/resources/6/values")
    assert(part1(source.mkString) === 4403592)
    source.close()
  }

  test("Part2 example") {
    val source = io.Source.fromFile("./src/test/resources/6/example_1")
    assert(part2(source.mkString) === 71503)
    source.close()
  }

  test("Part2 solution") {
    val source = io.Source.fromFile("./src/test/resources/6/values")
    assert(part2(source.mkString) === 38017587)
    source.close()
  }
}
