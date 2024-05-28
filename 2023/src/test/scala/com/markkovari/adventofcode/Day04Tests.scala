package com.markkovari.adventofcode

import com.markkovari.adventofcode.day04.*
import org.scalatest.funsuite.AnyFunSuite

import scala.io.Source
import scala.util.Using

class Day04Tests extends AnyFunSuite {

  test("Card is parsed correctly") {
    val input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    val card = parseCard(input)
    assert(card.id == 6)
    assert(card.numbers == List(31, 18, 13, 56, 72))
    assert(card.winningNumbers == List(74, 77, 10, 23, 35, 67, 36, 11))
  }

  test("Card is scored correctly") {
    Using(Source.fromFile("./src/test/resources/4/example_1")) { source =>
      val example = source.mkString
      val cards = parseCards(example)
      val scores = cards.map(_.getScore())
      assert(scores == List(8, 2, 2, 1, 0, 0))
    }
  }

  test("First solution") {
    Using(Source.fromFile("./src/test/resources/4/values")) { source =>
      val example = source.mkString
      val cards = parseCards(example)
      val scores = cards.map(_.getScore())
      assert(scores.sum == 22193)
    }
  }

  test("Example part2") {
    Using(Source.fromFile("./src/test/resources/4/example_1")) { source =>
      val example = source.mkString
      val ticketAmount = part2(example)
      assert(ticketAmount == 29)
    }
  }

  test("Solution part2") {
    Using(Source.fromFile("./src/test/resources/4/values")) { source =>
      val example = source.mkString
      val ticketAmount = part2_(example)
      assert(ticketAmount == "5625994")
    }
  }

}
