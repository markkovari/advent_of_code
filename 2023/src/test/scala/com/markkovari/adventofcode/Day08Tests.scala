package com.markkovari.adventofcode

import com.markkovari.adventofcode.day08.*
import org.scalatest.funsuite.AnyFunSuite

class Day08Tests extends AnyFunSuite {

  test("Part1 example1") {
    val source = io.Source.fromFile("./src/test/resources/8/example_1")
    assert(part1(source.mkString) === 2)
    source.close
  }

  test("Part1 example2") {
    val source = io.Source.fromFile("./src/test/resources/8/example_2")
    assert(part1(source.mkString) === 6)
    source.close
  }

  test("Part1 solution") {
    val source = io.Source.fromFile("./src/test/resources/8/values")
    assert(part1(source.mkString) === 19241)
    source.close
  }

  test("Part2 example") {
    val source = io.Source.fromFile("./src/test/resources/8/example_3")
    assert(part2(source.mkString) === 6)
    source.close
  }

  test("Part2 solution") {
    val source = io.Source.fromFile("./src/test/resources/8/values")
    assert(part2(source.mkString) === 9606140307013L)
    source.close
  }
}
