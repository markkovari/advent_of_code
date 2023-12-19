package com.markkovari.adventofcode.day04

import scala.util.matching.Regex.Match

case class Card(id: Int, numbers: List[Int], winningNumbers: List[Int]) {
  private def matches(): Set[Int] = {
    numbers.toSet.intersect(winningNumbers.toSet)
  }

  def getScore(): Int = {
    math.pow(2, matches().size - 1).intValue
  }
}

def parseCard(input: String): Card =
  input match {
    case s"Card ${id}: ${numbers} | ${winningNumbers}" =>
      Card(
        id.trim().toInt,
        // Sould be able to do this with a regex, but I can't figure it out right now :| by "whitespaces""
        numbers.split(" ").filter(!_.isEmpty).map(_.toInt).toList,
        winningNumbers.split(" ").filter(!_.isEmpty).map(_.toInt).toList
      )
  }

def parseCards(input: String): List[Card] =
  input.split("\n").map(parseCard).toList

def part2(input: String): BigInt =
  parseCards(input)
    .map(_.getScore())
    .foldLeft((0, Vector(1))) { case ((numCards, multiplicities), winning) =>
      val thisMult = multiplicities(0)
      val restMult = multiplicities
        .drop(1)
        .padTo(Math.max(1, winning), 1)
        .zipWithIndex
        .map((mult, idx) => if idx < winning then mult + thisMult else mult)
      (numCards + thisMult, restMult)
    }
    ._1

def countWinning(card: String): Int =
  val numbers = card
    .substring(card.indexOf(":") + 1) // discard "Card X:"
    .split(" ")
    .filterNot(_.isEmpty())
  val (winningNumberStrs, givenNumberStrs) = numbers.span(_ != "|")
  val winningNumbers = winningNumberStrs.map(_.toInt).toSet
  // drop the initial "|"
  val givenNumbers = givenNumberStrs.drop(1).map(_.toInt).toSet
  winningNumbers.intersect(givenNumbers).size
end countWinning

def winningCounts(input: String): Iterator[Int] =
  input.linesIterator.map(countWinning)
end winningCounts

def part1_(input: String): String =
  winningCounts(input)
    .map(winning => if winning > 0 then Math.pow(2, winning - 1).toInt else 0)
    .sum
    .toString()
end part1_

def part2_(input: String): String =
  winningCounts(input)
    // we only track the multiplicities of the next few cards as needed, not all of them;
    // and the first element always exists, and corresponds to the current card;
    // and the elements are always positive (because there is at least 1 original copy of each card)
    .foldLeft((0, Vector(1))) { case ((numCards, multiplicities), winning) =>
      val thisMult = multiplicities(0)
      val restMult = multiplicities
        .drop(1)
        // these are the original copies of the next few cards
        .padTo(Math.max(1, winning), 1)
        .zipWithIndex
        // these are the extra copies we just won
        .map((mult, idx) => if idx < winning then mult + thisMult else mult)
      (numCards + thisMult, restMult)
    }
    ._1
    .toString()
end part2_
