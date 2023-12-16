package com.markkovari.adventofcode

import scala.collection.immutable.HashMap
import scala.io.Source

@main def part1: Unit = println(s"The solution is ${firstResult}")
@main def part2: Unit = println(s"The solution is ${secondResult}")

val numbersAsStringsAndValues =
  HashMap[String, Int](
    "one" -> 1,
    "two" -> 2,
    "three" -> 3,
    "four" -> 4,
    "five" -> 5,
    "six" -> 6,
    "seven" -> 7,
    "eight" -> 8,
    "nine" -> 9
  )

private def stringStartsStringifiedDigit(text: String): Option[Int] = {
  numbersAsStringsAndValues
    .find { case (key, _) => text.startsWith(key) }
    .map { case (_, value) => value }
}

private val exampleFilename = "example_1"
private val exampleFilename2 = "example_2"
private val valuesFilename = "values"

private val lines =
  Source.fromFile(s"./src/main/resources/1/${valuesFilename}").getLines
private val linesForSecond =
  Source.fromFile(s"./src/main/resources/1/${valuesFilename}").getLines

val firstResult =
  lines
    .map(line => getFirstAndLastMultipliedTen(getDigits(line)))
    .sum

val secondResult =
  linesForSecond
    .map(line => getFirstAndLastMultipliedTen(getMixedUpDigits(line)))
    .sum

private def getFirstAndLastMultipliedTen(list: List[Int]): Int = {
  list match {
    case a :: Nil       => a * 10 + a
    case a :: b :: Nil  => a * 10 + b
    case a :: b :: tail => a * 10 + tail.last
    case _              => 0
  }
}

def getMixedUpDigits(of: String): List[Int] = of match {
  case "" => List()
  case other => {
    stringStartsStringifiedDigit(other) match {
      case Some(value) => value :: getMixedUpDigits(other.splitAt(1)._2)
      case None => {
        val (head, tail) = other.splitAt(1)
        head.toIntOption match {
          case None    => getMixedUpDigits(tail)
          case Some(_) => head.toInt :: getMixedUpDigits(tail)
        }
      }
    }
  }

}

def getDigits(of: String): List[Int] = of match {
  case "" => List()
  case a => {
    val (head, tail) = a.splitAt(1)
    head.toIntOption match {
      case None    => getDigits(tail)
      case Some(_) => head.toInt :: getDigits(tail)
    }
  }
}
