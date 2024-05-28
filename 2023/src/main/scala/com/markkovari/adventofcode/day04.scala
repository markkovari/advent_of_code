package com.markkovari.adventofcode.day04

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
        numbers.split(" ").filter(_.nonEmpty).map(_.toInt).toList,
        winningNumbers.split(" ").filter(_.nonEmpty).map(_.toInt).toList
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
    .substring(card.indexOf(":") + 1)
    .split(" ")
    .filterNot(_.isEmpty())
  val (winningNumberStrings, givenNumberStrings) = numbers.span(_ != "|")
  val winningNumbers = winningNumberStrings.map(_.toInt).toSet
  val givenNumbers = givenNumberStrings.drop(1).map(_.toInt).toSet
  winningNumbers.intersect(givenNumbers).size

def winningCounts(input: String): Iterator[Int] =
  input.linesIterator.map(countWinning)

def part1_(input: String): String =
  winningCounts(input)
    .map(winning => if winning > 0 then Math.pow(2, winning - 1).toInt else 0)
    .sum
    .toString

def part2_(input: String): String =
  winningCounts(input)
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
    .toString
