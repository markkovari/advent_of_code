package com.markkovari.adventofcode

import org.scalatest.funsuite.AnyFunSuite
import scala.io.Source
import scala.collection.immutable.HashMap
import day04._

class Day04Tests extends AnyFunSuite {

  test("Card is parsed correctly") {

    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    val input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    val card = parseCard(input)
    assert(card.id == 6)
    assert(card.numbers == List(31, 18, 13, 56, 72))
    assert(card.winningNumbers == List(74, 77, 10, 23, 35, 67, 36, 11))
  }

  test("Card is scored correctly") {
    val example = Source.fromFile("./src/test/resources/4/example_1").mkString
    val cards = parseCards(example)
    val scores = cards.map(_.getScore())
    assert(scores == List(8, 2, 2, 1, 0, 0))
  }

  test("First solution") {
    val example = Source.fromFile("./src/test/resources/4/values").mkString
    val cards = parseCards(example)
    val scores = cards.map(_.getScore())
    assert(scores.sum == 22193)
  }

  test("Example part2") {
    val example = Source.fromFile("./src/test/resources/4/example_1").mkString
    val ticketAmount = part2(example)
    assert(ticketAmount - 1 == 30)
  }

  test("Solution part2") {
    val example = Source.fromFile("./src/test/resources/4/values").mkString
    val ticketAmount = part2_(example)
    assert(ticketAmount == "5625994")
  }

}
