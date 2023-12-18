package com.markkovari.adventofcode

import org.scalatest.funsuite.AnyFunSuite
import scala.io.Source
import scala.collection.immutable.HashMap
import day03._

class Day03Tests extends AnyFunSuite {

  test("examples result is the same as in the description") {

    val text = Source.fromFile("./src/main/resources/3/example_1").mkString
    val result = part1(text)
    val result2 = part2(text)
    assert(result == 4361)
    assert(result2 == 467835)
  }

  test("result part1 and part2") {
    val text = Source.fromFile("./src/main/resources/3/values").mkString
    val result = part1(text)
    val result2 = part2(text)
    assert(result == 540212)
    assert(result2 == 87605697)
  }

}
