package com.markkovari.adventofcode.day06

// Oh boy, did I port this https://github.com/tlareg/advent-of-code/blob/master/src/2023/day06/index.ts ?
// I sure did

type Race = (Long, Long)

def part1(input: String): Long = {
  // should not collect iterator to array, but I'm lazy, just like my algorithms
  val lines = input.linesIterator.toArray
  val times = lines(0).split("\\s+").drop(1).map(_.toInt)
  val distances = lines(1).split("\\s+").drop(1).map(_.toInt)
  val races = times.map(_.toLong).zip(distances.map(_.toLong))
  solveForRaces(races)
}

def solveForRaces(races: Array[Race]): Long = {
  races
    .map((a, b) => getNumberOfWaysBeatingRecord(a, b))
    .product
}

def getNumberOfWaysBeatingRecord(raceTime: Long, recordDistance: Long): Long = {
  (0 until raceTime.toInt)
    .map(holdTime => getDistance(holdTime, raceTime))
    .count(distanceByHold => distanceByHold > recordDistance)
}

def getDistance(holdTime: Long, raceTime: Long): Long =
  holdTime * (raceTime - holdTime)

def part2(input: String): Long = {
  val lines = input.linesIterator.toArray
  val time = parseLineForPart2(lines(0))
  val distance = parseLineForPart2(lines(1))
  solveForRaces(Array[Race]((time, distance)))
}

def parseLineForPart2(line: String): Long = {
  val num = line.split(":")(1).trim().split("\\s+").mkString
  num.toLong
}
