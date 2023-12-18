package com.markkovari.adventofcode.day01

import scala.collection.immutable.HashMap
import scala.io.Source

val numbersAsStringsAndValues =
  HashMap(
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

def stringStartsStringifiedDigit(text: String): Option[Int] =
  numbersAsStringsAndValues
    .find { case (key, _) => text.startsWith(key) }
    .map { case (_, value) => value }

def getFirstAndLastMultipliedTen(list: List[Int]): Int =
  list match {
    case a :: Nil       => a * 10 + a
    case a :: b :: Nil  => a * 10 + b
    case a :: b :: tail => a * 10 + tail.last
    case _              => 0
  }

def getMixedUpDigits(of: String): List[Int] = of match {
  case "" => List()
  case other =>
    stringStartsStringifiedDigit(other) match {
      case Some(value) => value :: getMixedUpDigits(other.splitAt(1)._2)
      case None =>
        val (head, tail) = other.splitAt(1)
        head.toIntOption match {
          case None        => getMixedUpDigits(tail)
          case Some(value) => value :: getMixedUpDigits(tail)
        }
    }
}

def getDigits(of: String): List[Int] = of match {
  case "" => List()
  case a =>
    val (head, tail) = a.splitAt(1)
    head.toIntOption match {
      case None        => getDigits(tail)
      case Some(value) => value :: getDigits(tail)
    }
}
