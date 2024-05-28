package com.markkovari.adventofcode

import com.markkovari.adventofcode.day05.*
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source

class Day05Tests extends AnyFunSuite {

  test("Part1 example") {
    val source = io.Source.fromFile("./src/test/resources/5/example_1")

    val input = source.mkString
    val result = part1(input)
    assert(result == 35)
    source.close
  }

  test("Part1 is calculated") {
    val source = io.Source.fromFile("./src/test/resources/5/values")
    val input = source.mkString
    val result = part1(input)
    assert(result == 1181555926)
    source.close
  }

  test("Part2 example") {
    val source = io.Source.fromFile("./src/test/resources/5/example_1")
    val input = source.mkString
    val result = part2(input)
    assert(result == 46)
    source.close
  }

  test("Part2 is calculated") {
    val source = io.Source.fromFile("./src/test/resources/5/values")
    val input = source.mkString
    val result = part2(input)
    assert(result == 37806486)
    source.close
  }
}
