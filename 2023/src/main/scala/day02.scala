package com.markkovari.adventofcode.day02

import scala.collection.immutable.HashMap
import scala.io.Source
import scala.annotation.static

@main def part1: Unit = println(s"The solution is ${firstResult}")
@main def part2: Unit = println(s"The solution is ${secondResult}")

private val exampleFilename = "example_1"
private val valuesFilename = "values"

private val lines =
  Source.fromFile(s"./src/main/resources/2/${exampleFilename}").getLines

private val linesForSecond =
  Source.fromFile(s"./src/main/resources/2/${valuesFilename}").getLines

val firstResult =
  getGamesFromFile(lines)
    .filter(game =>
      // Add all cubes together
      game
        .addSetsTogether()
        .sets
        .head
        .tapEach(cube => println(cube))
        .isSubsetOf(existingCubes)
    )
    .length

val secondResult =
  2

trait Comparable[T] {
  def compare(that: T): Int
}

class Cube(val amount: Int, val color: String) {
  override def toString: String = s"${this.amount} ${this.color}"
}

def createCubeFromString(cubeString: String): Cube = {
  val (amount, color) = cubeString.split(" ").toList.splitAt(1)
  new Cube(amount.head.toInt, color.head)
}

case class Game(val index: Int, val sets: List[List[Cube]]) {
  def addSetsTogether(): Game =
    new Game(this.index, List(this.sets.flatten))

  override def toString: String =
    s"Game ${this.index}: ${this.sets.map(set => set.mkString(", ")).mkString("; ")}"
}

extension (cubes: List[Cube]) {
  def +(other: List[Cube]): List[Cube] =
    cubes ++ other

  def isSubsetOf(other: List[Cube]): Boolean =
    if other == null then false
    else if (cubes.length > other.length) false
    else
      cubes.forall(cube =>
        other
          .filter(otherCube => otherCube.color == cube.color)
          .exists(otherCube => otherCube.amount >= cube.amount)
      )
}

def createGameFromString(gameString: String): Game = {
  val stripped = gameString.stripPrefix("Game ")
  val (head, tail) = stripped.split(": ").toList.splitAt(1)
  val index = head.head.toInt
  val sets = tail.head
    .split("; ")
    .map(set =>
      set
        .split(", ")
        .map(cube => createCubeFromString(cube))
        .toList
    )
    .toList
  new Game(index, sets)
}

def getGamesFromFile(lines: Iterator[String]): List[Game] =
  lines.map(line => createGameFromString(line)).toList

val existingCubes: List[Cube] =
  List(
    new Cube(12, "red"),
    new Cube(13, "green"),
    new Cube(14, "blue")
  )
