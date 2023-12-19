package com.markkovari.adventofcode

import org.scalatest.funsuite.AnyFunSuite
import scala.io.Source
import scala.collection.immutable.HashMap
import day02._

class Day02Tests extends AnyFunSuite {

  test("examples result is the same as in the description") {

    val text = Source.fromFile("./src/test/resources/2/example_1").mkString
    val result = part1(text)
    assert(result == 8)
  }

  test("result is the same as in the description") {

    val text = Source.fromFile("./src/test/resources/2/values").mkString
    val result = part1(text)
    assert(result == 2617)
  }

  test("result is the same as in the description part 2") {

    val text = Source.fromFile("./src/test/resources/2/values").mkString
    val result = part2(text)
    assert(result == 59795)
  }

}
