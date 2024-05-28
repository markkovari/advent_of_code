package com.markkovari.adventofcode

import com.markkovari.adventofcode.day05.*
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source
import scala.util.Using

class Day05Tests extends AnyFunSuite {

  test("Part1 example") {
    Using(io.Source.fromFile("./src/test/resources/5/example_1")) { source =>
      val input = source.mkString
      val result = part1(input)
      assert(result == 35)
    }
  }

  test("Part1 is calculated") {
    Using(io.Source.fromFile("./src/test/resources/5/values")) { source =>
      val input = source.mkString
      val result = part1(input)
      assert(result == 1182)
    }
  }

  test("Part2 example") {
    Using(io.Source.fromFile("./src/test/resources/5/example_1")) { source =>
      val input = source.mkString
      val result = part2(input)
      assert(result == 8)
    }
  }

  test("Part2 is calculated") {
    Using(io.Source.fromFile("./src/test/resources/5/values")) { source =>
      val input = source.mkString
      val result = part2(input)
      assert(result == 1181555926)
    }
  }
}
