package com.markkovari.adventofcode

import com.markkovari.adventofcode.day06.part1
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source

class Day06Tests extends AnyFunSuite {
  test("Part1 example") {
    val source = io.Source.fromFile("./src/test/resources/6/example_1")
    assert(part1(source.mkString) === 288)
    source.close()
  }
}
