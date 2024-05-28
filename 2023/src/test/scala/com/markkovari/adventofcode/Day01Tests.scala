package com.markkovari.adventofcode

import com.markkovari.adventofcode.day01.{getDigits, getFirstAndLastMultipliedTen, getMixedUpDigits}
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source
import scala.util.Using

class Day01Tests extends AnyFunSuite {

  private val exampleFilename = "example_1"
  private val exampleFilename2 = "example_2"
  private val valuesFilename = "values"

  test("example result is the same as in the description") {
    Using(Source.fromFile(s"./src/test/resources/1/${exampleFilename}")) {
      source =>
        val lines = source.getLines
        val firstResult =
          lines
            .map(line => getFirstAndLastMultipliedTen(getDigits(line)))
            .sum
        assert(firstResult == 101)
    }
  }

  test("result first part") {
    Using(Source.fromFile(s"./src/test/resources/1/${valuesFilename}")) {

      source =>
        val lines = source.getLines

        val firstResult =
          lines
            .map(line => getFirstAndLastMultipliedTen(getDigits(line)))
            .sum
        assert(firstResult == 54573)

      val secondLines = source.getLines
      val secondResult =
        secondLines
          .map(line => getFirstAndLastMultipliedTen(getMixedUpDigits(line)))
          .sum

      assert(secondResult == 54591)
    }

  }

}
