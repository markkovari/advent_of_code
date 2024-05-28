package com.markkovari.adventofcode

import com.markkovari.adventofcode.day03.*
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source

class Day03Tests extends AnyFunSuite {

  test("examples result is the same as in the description") {

    val source = Source.fromFile("./src/test/resources/3/example_1")
    val str = source.mkString
    val result = part1(str)
    val result2 = part2(str)
    assert(result == 4361)
    assert(result2 == 467835)
    source.close
  }

  test("result part1 and part2") {
    val source = Source.fromFile("./src/test/resources/3/values")
    val text = source.mkString
    val result = part1(text)
    val result2 = part2(text)
    assert(result == 540212)
    assert(result2 == 87605697)
    source.close
  }

}
