package com.markkovari.adventofcode

import com.markkovari.adventofcode.day07.*
import org.scalatest.funsuite.AnyFunSuite

class Day07Tests extends AnyFunSuite {

  test("Part1 example") {
    val source = io.Source.fromFile("./src/test/resources/7/example_1")
    assert(part1(source.mkString) === 6440)
    source.close()

  }

  test("Part1 solution") {
    val source = io.Source.fromFile("./src/test/resources/7/values")
    assert(part1(source.mkString) === 250602641)
    source.close()
  }

  test("Part2 example") {
    val source = io.Source.fromFile("./src/test/resources/7/example_1")
    assert(part2(source.mkString) === 5905)
    source.close()
  }

  test("Part2 solution") {
    val source = io.Source.fromFile("./src/test/resources/7/values")
    assert(part2(source.mkString) === 251037509)
    source.close()
  }
}
