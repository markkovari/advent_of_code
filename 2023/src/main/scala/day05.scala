package com.markkovari.adventofcode.day05

import scala.util.matching.Regex.Match

final case class Resource(start: Long, end: Long, kind: ResourceKind)

enum ResourceKind:
  case Seed, Soil, Fertilizer, Water,
    Light, Temperature, Humidity, Location

final case class ResourceMap(
    from: ResourceKind,
    to: ResourceKind,
    properties: Seq[Property]
)

final case class Property(
    destinationStart: Long,
    sourceStart: Long,
    rangeLength: Long
):

  lazy val sourceEnd: Long = sourceStart + rangeLength - 1
end Property

def findNext(resource: Resource, map: ResourceMap): Seq[Resource] =
  val ResourceMap(from, to, properties) = map
  val (newResources, explore) =
    val initial = (Seq.empty[Resource], Option(resource))
    properties.foldLeft(initial) {
      case ((acc, Some(explore)), prop) =>
        val Resource(start, end, _) = explore
        val propStart = prop.sourceStart
        val propEnd = prop.sourceEnd
        val underRange = Option.when(start < propStart)(
          Resource(start, Math.min(propStart - 1, end), to)
        )
        val overlaps =
          start >= propStart && start <= propEnd
            || end >= propStart && end <= propEnd
            || start <= propStart && end >= propEnd
        val inRange = Option.when(overlaps) {
          val delay = prop.destinationStart - propStart
          Resource(
            Math.max(start, propStart) + delay,
            Math.min(end, propEnd) + delay,
            to
          )
        }
        val aboveRange = Option.when(end > propEnd)(
          Resource(Math.max(start, propEnd + 1), end, to)
        )
        (Seq(underRange, inRange, acc).flatten, aboveRange)
      case ((acc, None), _) => (acc, None)
    }
  Seq(newResources, explore).flatten
end findNext

object ResourceMap:
  def buildFromLines(lines: Seq[String]): Seq[ResourceMap] =
    def isRangeLine(line: String) =
      line.forall(ch => ch.isDigit || ch.isSpaceChar)
    lines
      .filter(line =>
        !line.isBlank &&
          (line.endsWith("map:") || isRangeLine(line))
      )
      .foldLeft(Seq.empty[(String, Seq[String])]) {
        case (acc, line) if line.endsWith("map:") =>
          (line, Seq.empty) +: acc
        case (Seq((definition, properties), last*), line) =>
          (definition, line +: properties) +: last
      }
      .flatMap(build)

  def build(map: String, ranges: Seq[String]): Option[ResourceMap] =
    val mapRow = map.replace("map:", "").trim.split("-to-")
    val properties = ranges
      .map(line => line.split(" ").flatMap(_.toLongOption))
      .collect:
        case Array(startFrom, startTo, range) =>
          Property(startFrom, startTo, range)
    def resourceKindOf(optStr: Option[String]) =
      optStr.map(_.capitalize).map(ResourceKind.valueOf)
    for
      from <- resourceKindOf(mapRow.headOption)
      to <- resourceKindOf(mapRow.lastOption)
    yield ResourceMap(from, to, properties.sortBy(_.sourceStart))
end ResourceMap

object Seeds:
  private def parseSeedsRaw(line: String): Seq[Long] =
    if !line.startsWith("seeds:") then Seq.empty[Long]
    else
      line
        .replace("seeds:", "")
        .trim
        .split(" ")
        .flatMap(_.toLongOption)

  // parse seeds without range
  def parseWithoutRange(line: String): Seq[Resource] =
    parseSeedsRaw(line).map: start =>
      Resource(start, start, ResourceKind.Seed)

  // parse seeds with range
  def parse(line: String): Seq[Resource] =
    parseSeedsRaw(line)
      .grouped(2)
      .map { case Seq(start, length) =>
        Resource(start, start + length - 1, ResourceKind.Seed)
      }
      .toSeq
end Seeds

def calculate(seeds: Seq[Resource], maps: Seq[ResourceMap]): Long =
  def inner(resource: Resource): Seq[Resource] =
    if resource.kind == ResourceKind.Location then Seq(resource)
    else
      val map = maps.find(_.from == resource.kind).get
      findNext(resource, map).flatMap(inner)
  seeds.flatMap(inner).minBy(_.start).start
end calculate

type ParseSeeds = String => Seq[Resource]

def solution(input: String, parse: ParseSeeds): Long =
  val lines = input.linesIterator.toSeq
  val seeds = lines.headOption.map(parse).getOrElse(Seq.empty)
  val maps = ResourceMap.buildFromLines(lines)
  calculate(seeds, maps)

def part1(input: String): Long =
  solution(input, Seeds.parseWithoutRange)

def part2(input: String): Long =
  solution(input, Seeds.parse)
