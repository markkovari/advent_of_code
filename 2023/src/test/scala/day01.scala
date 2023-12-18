package com.markkovari.adventofcode

import org.scalatest.funsuite.AnyFunSuite
import scala.io.Source
import scala.collection.immutable.HashMap
import day02._
import com.markkovari.adventofcode.day01.getFirstAndLastMultipliedTen
import com.markkovari.adventofcode.day01.getDigits
import com.markkovari.adventofcode.day01.getMixedUpDigits

class Day01Tests extends AnyFunSuite {

  private val exampleFilename = "example_1"
  private val exampleFilename2 = "example_2"
  private val valuesFilename = "values"

  test("example result is the same as in the description") {
    val lines =
      Source.fromFile(s"./src/main/resources/1/${exampleFilename}").getLines

    val firstResult =
      lines
        .map(line => getFirstAndLastMultipliedTen(getDigits(line)))
        .sum
    assert(firstResult == 142)
  }

  test("result first part") {
    val lines =
      Source.fromFile(s"./src/main/resources/1/${valuesFilename}").getLines

    val firstResult =
      lines
        .map(line => getFirstAndLastMultipliedTen(getDigits(line)))
        .sum
    assert(firstResult == 54573)

    val linesForSecond =
      Source.fromFile(s"./src/main/resources/1/${valuesFilename}").getLines
    val secondResult =
      linesForSecond
        .map(line => getFirstAndLastMultipliedTen(getMixedUpDigits(line)))
        .sum

    assert(secondResult == 54591)
  }

}
