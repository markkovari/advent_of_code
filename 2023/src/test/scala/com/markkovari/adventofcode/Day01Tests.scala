package com.markkovari.adventofcode

import com.markkovari.adventofcode.day01.{getDigits, getFirstAndLastMultipliedTen, getMixedUpDigits}
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source

class Day01Tests extends AnyFunSuite {
  test("example result is the same as in the description") {
    val source = Source.fromFile(s"./src/test/resources/1/example_1")

    val lines = source.getLines

    val firstResult =
      lines
        .map(line => getFirstAndLastMultipliedTen(getDigits(line)))
        .sum
    assert(firstResult == 142)
    source.close()
  }

  test("result first part") {
    val source = Source.fromFile(s"./src/test/resources/1/values")

    val lines = source.getLines

    val firstResult =
      lines
        .map(line => getFirstAndLastMultipliedTen(getDigits(line)))
        .sum
    assert(firstResult == 54573)

    source.close()

    val resource = Source.fromFile(s"./src/test/resources/1/values")

    val linesForSecond = resource.getLines
    val secondResult =
      linesForSecond
        .map(line => getFirstAndLastMultipliedTen(getMixedUpDigits(line)))
        .sum

    assert(secondResult == 54591)
    resource.close()
  }

}
