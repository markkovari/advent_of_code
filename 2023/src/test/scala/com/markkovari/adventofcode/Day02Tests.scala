package com.markkovari.adventofcode

import com.markkovari.adventofcode.day02.*
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source

class Day02Tests extends AnyFunSuite {

  test("examples result is the same as in the description") {

    val source = Source.fromFile("./src/test/resources/2/example_1")
    assert(part1(source.mkString) == 8)
    source.close
  }

  test("result is the same as in the description") {

    val source = Source.fromFile("./src/test/resources/2/values")
    assert(part1(source.mkString) == 2617)
    source.close()
  }

  test("result is the same as in the description part 2") {
    val source = Source.fromFile("./src/test/resources/2/values")
    assert(part2(source.mkString) == 59795)
    source.close
  }

}
