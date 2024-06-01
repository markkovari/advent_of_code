package com.markkovari.adventofcode

import com.markkovari.adventofcode.day09.*
import org.scalatest.funsuite.AnyFunSuite

class Day09Tests extends AnyFunSuite {

  test("Part1 example1") {
    val source = io.Source.fromFile("./src/test/resources/9/example_1")
    assert(part1(source.mkString) === 114)
    source.close
  }

  test("Part1 solution") {
    val source = io.Source.fromFile("./src/test/resources/9/values")
    assert(part1(source.mkString) === 2101499000)
    source.close
  }

  test("Part2 example") {
    val source = io.Source.fromFile("./src/test/resources/9/example_1")
    assert(part2(source.mkString) === 2)
    source.close
  }

  test("Part2 solution") {
    val source = io.Source.fromFile("./src/test/resources/9/values")
    assert(part2(source.mkString) === 1089)
    source.close
  }
}
